#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, Vec,
};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallStatus {
    Pending,
    Subscribed,
    Cancelled,
    DrawnDown,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapitalCall {
    pub id: u64,
    pub amount_requested: i128,
    pub amount_collected: i128,
    pub due_date: u64,
    pub status: CallStatus,
}

#[contracttype]
pub enum DataKey {
    FundContract,
    GpTreasury,
    UsdcToken,
    LpToken,
    CallCount,
    CallInfo(u64),
    LpContribution(u64, Address),
    LpList(u64), // List of LPs who contributed to a specific call
}

#[soroban_sdk::contractclient(name = "FundClient")]
pub trait FundInterface {
    fn get_gp(env: Env) -> Address;
    fn is_whitelisted(env: Env, investor: Address) -> bool;
    fn get_commitment(env: Env, investor: Address) -> i128;
    fn mint_lp_tokens(env: Env, caller: Address, investor: Address, amount: i128);
}

#[contract]
pub struct CapitalCallContract;

#[contractimpl]
impl CapitalCallContract {
    pub fn initialize(
        env: Env,
        fund_contract: Address,
        gp_treasury: Address,
        usdc_token: Address,
        lp_token: Address,
    ) {
        assert!(!env.storage().instance().has(&DataKey::FundContract), "already initialized");

        env.storage().instance().set(&DataKey::FundContract, &fund_contract);
        env.storage().instance().set(&DataKey::GpTreasury, &gp_treasury);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&DataKey::LpToken, &lp_token);
        env.storage().instance().set(&DataKey::CallCount, &0u64);
    }

    pub fn create_capital_call(env: Env, gp: Address, amount: i128, duration: u64) -> u64 {
        gp.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert_eq!(gp, fund_client.get_gp(), "only GP can create capital call");
        assert!(amount > 0, "requested amount must be positive");
        assert!(duration > 0, "duration must be positive");

        let count: u64 = env.storage().instance().get(&DataKey::CallCount).unwrap_or(0);
        let call_id = count + 1;
        env.storage().instance().set(&DataKey::CallCount, &call_id);

        let capital_call = CapitalCall {
            id: call_id,
            amount_requested: amount,
            amount_collected: 0,
            due_date: env.ledger().timestamp() + duration,
            status: CallStatus::Pending,
        };

        env.storage().persistent().set(&DataKey::CallInfo(call_id), &capital_call);
        env.storage().persistent().set(&DataKey::LpList(call_id), &Vec::<Address>::new(&env));

        call_id
    }

    pub fn contribute(env: Env, lp: Address, call_id: u64, amount: i128) {
        lp.require_auth();
        assert!(amount > 0, "contribution must be positive");

        let call_key = DataKey::CallInfo(call_id);
        let mut capital_call: CapitalCall = env
            .storage()
            .persistent()
            .get(&call_key)
            .expect("capital call not found");

        assert!(
            matches!(capital_call.status, CallStatus::Pending),
            "capital call is not pending"
        );
        assert!(
            env.ledger().timestamp() <= capital_call.due_date,
            "capital call has expired"
        );

        // Verify LP is whitelisted on the Fund contract
        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert!(
            fund_client.is_whitelisted(&lp),
            "LP is not whitelisted on the fund"
        );

        // Transfer USDC from LP to this contract
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        usdc_client.transfer(
            &lp,
            &env.current_contract_address(),
            &amount,
        );

        // Update LP contribution
        let contribution_key = DataKey::LpContribution(call_id, lp.clone());
        let current_contribution = env
            .storage()
            .persistent()
            .get::<_, i128>(&contribution_key)
            .unwrap_or(0);
        env.storage().persistent().set(&contribution_key, &(current_contribution + amount));

        // Add LP to the list if it's their first contribution to this call
        let lp_list_key = DataKey::LpList(call_id);
        let mut lp_list: Vec<Address> = env.storage().persistent().get(&lp_list_key).unwrap();
        if !lp_list.contains(&lp) {
            lp_list.push_back(lp);
            env.storage().persistent().set(&lp_list_key, &lp_list);
        }

        // Update collected amount
        capital_call.amount_collected += amount;
        if capital_call.amount_collected >= capital_call.amount_requested {
            capital_call.status = CallStatus::Subscribed;
        }
        env.storage().persistent().set(&call_key, &capital_call);
    }

    pub fn drawdown(env: Env, gp: Address, call_id: u64) {
        gp.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert_eq!(gp, fund_client.get_gp(), "only GP can drawdown");

        let call_key = DataKey::CallInfo(call_id);
        let mut capital_call: CapitalCall = env
            .storage()
            .persistent()
            .get(&call_key)
            .expect("capital call not found");

        assert!(
            matches!(capital_call.status, CallStatus::Subscribed)
                || (matches!(capital_call.status, CallStatus::Pending)
                    && env.ledger().timestamp() > capital_call.due_date
                    && capital_call.amount_collected > 0),
            "capital call is not ready for drawdown"
        );

        // Transfer USDC to GP treasury
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        let gp_treasury = Self::get_gp_treasury(env.clone());
        usdc_client.transfer(
            &env.current_contract_address(),
            &gp_treasury,
            &capital_call.amount_collected,
        );

        // Mint LP tokens to contributors
        let lp_list_key = DataKey::LpList(call_id);
        let lp_list: Vec<Address> = env.storage().persistent().get(&lp_list_key).unwrap();
        for lp in lp_list.iter() {
            let contribution_key = DataKey::LpContribution(call_id, lp.clone());
            let amount = env.storage().persistent().get::<_, i128>(&contribution_key).unwrap();
            if amount > 0 {
                fund_client.mint_lp_tokens(&env.current_contract_address(), &lp, &amount);
            }
        }

        capital_call.status = CallStatus::DrawnDown;
        env.storage().persistent().set(&call_key, &capital_call);
    }

    pub fn cancel_capital_call(env: Env, gp: Address, call_id: u64) {
        gp.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert_eq!(gp, fund_client.get_gp(), "only GP can cancel capital call");

        let call_key = DataKey::CallInfo(call_id);
        let mut capital_call: CapitalCall = env
            .storage()
            .persistent()
            .get(&call_key)
            .expect("capital call not found");

        assert!(
            matches!(capital_call.status, CallStatus::Pending)
                || matches!(capital_call.status, CallStatus::Subscribed),
            "cannot cancel a completed or already cancelled call"
        );

        capital_call.status = CallStatus::Cancelled;
        env.storage().persistent().set(&call_key, &capital_call);
    }

    pub fn claim_refund(env: Env, lp: Address, call_id: u64) {
        lp.require_auth();

        let call_key = DataKey::CallInfo(call_id);
        let capital_call: CapitalCall = env
            .storage()
            .persistent()
            .get(&call_key)
            .expect("capital call not found");

        let is_cancelled = matches!(capital_call.status, CallStatus::Cancelled);
        let is_expired_undrawn = matches!(capital_call.status, CallStatus::Pending)
            && env.ledger().timestamp() > capital_call.due_date;

        assert!(
            is_cancelled || is_expired_undrawn,
            "capital call is not eligible for refunds"
        );

        let contribution_key = DataKey::LpContribution(call_id, lp.clone());
        let amount = env
            .storage()
            .persistent()
            .get::<_, i128>(&contribution_key)
            .unwrap_or(0);

        assert!(amount > 0, "no contribution to refund");

        // Refund USDC to LP
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        usdc_client.transfer(
            &env.current_contract_address(),
            &lp,
            &amount,
        );

        env.storage().persistent().set(&contribution_key, &0i128);
    }

    // --- Getters ---

    pub fn get_fund_contract(env: Env) -> Address {
        env.storage().instance().get(&DataKey::FundContract).unwrap()
    }

    pub fn get_gp_treasury(env: Env) -> Address {
        env.storage().instance().get(&DataKey::GpTreasury).unwrap()
    }

    pub fn get_usdc_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::UsdcToken).unwrap()
    }

    pub fn get_lp_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::LpToken).unwrap()
    }

    pub fn get_call_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::CallCount).unwrap_or(0)
    }

    pub fn get_call_info(env: Env, call_id: u64) -> Option<CapitalCall> {
        env.storage().persistent().get(&DataKey::CallInfo(call_id))
    }

    pub fn get_lp_contribution(env: Env, call_id: u64, lp: Address) -> i128 {
        env.storage().persistent().get(&DataKey::LpContribution(call_id, lp)).unwrap_or(0)
    }

    pub fn get_lp_list(env: Env, call_id: u64) -> Vec<Address> {
        env.storage().persistent().get(&DataKey::LpList(call_id)).unwrap_or_else(|| Vec::new(&env))
    }
}

#[cfg(test)]
mod test;

