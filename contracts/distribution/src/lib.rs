#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributionInfo {
    pub id: u64,
    pub total_amount: i128,
    pub total_lp_supply: i128,
}

#[contracttype]
pub enum DataKey {
    FundContract,
    UsdcToken,
    LpToken,
    DistributionCount,
    Distribution(u64),
    LpClaimed(u64, Address),
}

#[soroban_sdk::contractclient(name = "FundClient")]
pub trait FundInterface {
    fn get_gp(env: Env) -> Address;
    fn get_total_contributed(env: Env) -> i128;
}

#[contract]
pub struct DistributionContract;

#[contractimpl]
impl DistributionContract {
    pub fn initialize(
        env: Env,
        fund_contract: Address,
        usdc_token: Address,
        lp_token: Address,
    ) {
        assert!(!env.storage().instance().has(&DataKey::FundContract), "already initialized");

        env.storage().instance().set(&DataKey::FundContract, &fund_contract);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&DataKey::LpToken, &lp_token);
        env.storage().instance().set(&DataKey::DistributionCount, &0u64);
    }

    pub fn create_distribution(env: Env, gp: Address, amount: i128) -> u64 {
        gp.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert_eq!(gp, fund_client.get_gp(), "only GP can create distribution");
        assert!(amount > 0, "distribution amount must be positive");

        let total_lp_supply = fund_client.get_total_contributed();
        assert!(total_lp_supply > 0, "cannot distribute to a fund with no active contributions");

        // Transfer USDC from GP to this contract
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        usdc_client.transfer(&gp, &env.current_contract_address(), &amount);

        let count: u64 = env.storage().instance().get(&DataKey::DistributionCount).unwrap_or(0);
        let distribution_id = count + 1;
        env.storage().instance().set(&DataKey::DistributionCount, &distribution_id);

        let distribution = DistributionInfo {
            id: distribution_id,
            total_amount: amount,
            total_lp_supply,
        };

        env.storage().persistent().set(&DataKey::Distribution(distribution_id), &distribution);

        distribution_id
    }

    pub fn claim_distribution(env: Env, lp: Address, distribution_id: u64) -> i128 {
        lp.require_auth();

        let claimed_key = DataKey::LpClaimed(distribution_id, lp.clone());
        assert!(
            !env.storage().persistent().has(&claimed_key),
            "distribution already claimed"
        );

        let distribution_key = DataKey::Distribution(distribution_id);
        let distribution: DistributionInfo = env
            .storage()
            .persistent()
            .get(&distribution_key)
            .expect("distribution not found");

        // Get LP's current LP token balance
        let lp_token_address = Self::get_lp_token(env.clone());
        let lp_token_client = token::Client::new(&env, &lp_token_address);
        let lp_balance = lp_token_client.balance(&lp);

        assert!(lp_balance > 0, "no LP tokens held by claimant");

        // Calculate share: (total_amount * lp_balance) / total_lp_supply
        let share = (distribution.total_amount * lp_balance) / distribution.total_lp_supply;
        assert!(share > 0, "calculated share is zero due to rounding");

        // Transfer USDC share to LP
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        usdc_client.transfer(&env.current_contract_address(), &lp, &share);

        env.storage().persistent().set(&claimed_key, &true);

        share
    }

    // --- Getters ---

    pub fn get_fund_contract(env: Env) -> Address {
        env.storage().instance().get(&DataKey::FundContract).unwrap()
    }

    pub fn get_usdc_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::UsdcToken).unwrap()
    }

    pub fn get_lp_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::LpToken).unwrap()
    }

    pub fn get_distribution_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::DistributionCount).unwrap_or(0)
    }

    pub fn get_distribution(env: Env, distribution_id: u64) -> Option<DistributionInfo> {
        env.storage().persistent().get(&DataKey::Distribution(distribution_id))
    }

    pub fn has_claimed(env: Env, distribution_id: u64, lp: Address) -> bool {
        env.storage().persistent().has(&DataKey::LpClaimed(distribution_id, lp))
    }
}

#[cfg(test)]
mod test;

