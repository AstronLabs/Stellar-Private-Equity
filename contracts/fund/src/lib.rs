#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env,
};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundStatus {
    Fundraising,
    Active,
    Closed,
}

#[contracttype]
pub enum DataKey {
    Gp,
    UsdcToken,
    LpToken,
    FundName,
    TargetSize,
    TotalCommitted,
    TotalContributed,
    Whitelisted(Address),
    Commitment(Address),
    Contribution(Address),
    Status,
    AuthorizedMinter,
}

#[contract]
pub struct FundContract;

#[contractimpl]
impl FundContract {
    pub fn initialize(
        env: Env,
        gp: Address,
        usdc_token: Address,
        lp_token: Address,
        fund_name: soroban_sdk::String,
        target_size: i128,
    ) {
        assert!(!env.storage().instance().has(&DataKey::Gp), "already initialized");
        assert!(target_size > 0, "target size must be positive");

        env.storage().instance().set(&DataKey::Gp, &gp);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&DataKey::LpToken, &lp_token);
        env.storage().instance().set(&DataKey::FundName, &fund_name);
        env.storage().instance().set(&DataKey::TargetSize, &target_size);
        env.storage().instance().set(&DataKey::TotalCommitted, &0i128);
        env.storage().instance().set(&DataKey::TotalContributed, &0i128);
        env.storage().instance().set(&DataKey::Status, &FundStatus::Fundraising);
    }

    pub fn whitelist_investor(env: Env, gp: Address, investor: Address) {
        gp.require_auth();
        let current_gp: Address = env.storage().instance().get(&DataKey::Gp).unwrap();
        assert_eq!(gp, current_gp, "only GP can whitelist");

        env.storage().persistent().set(&DataKey::Whitelisted(investor), &true);
    }

    pub fn blacklist_investor(env: Env, gp: Address, investor: Address) {
        gp.require_auth();
        let current_gp: Address = env.storage().instance().get(&DataKey::Gp).unwrap();
        assert_eq!(gp, current_gp, "only GP can blacklist");

        env.storage().persistent().set(&DataKey::Whitelisted(investor), &false);
    }

    pub fn set_authorized_minter(env: Env, gp: Address, minter: Address) {
        gp.require_auth();
        let current_gp: Address = env.storage().instance().get(&DataKey::Gp).unwrap();
        assert_eq!(gp, current_gp, "only GP can set minter");

        env.storage().instance().set(&DataKey::AuthorizedMinter, &minter);
    }

    pub fn commit_capital(env: Env, investor: Address, amount: i128) {
        investor.require_auth();
        assert!(amount > 0, "commitment must be positive");

        let status: FundStatus = env.storage().instance().get(&DataKey::Status).unwrap();
        assert!(
            matches!(status, FundStatus::Fundraising),
            "can only commit during fundraising"
        );

        let is_whitelisted = env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::Whitelisted(investor.clone()))
            .unwrap_or(false);
        assert!(is_whitelisted, "investor is not whitelisted");

        let current_commitment = env
            .storage()
            .persistent()
            .get::<_, i128>(&DataKey::Commitment(investor.clone()))
            .unwrap_or(0);

        let new_commitment = current_commitment + amount;
        env.storage().persistent().set(&DataKey::Commitment(investor.clone()), &new_commitment);

        let total_committed: i128 = env.storage().instance().get(&DataKey::TotalCommitted).unwrap();
        env.storage().instance().set(&DataKey::TotalCommitted, &(total_committed + amount));
    }

    pub fn mint_lp_tokens(env: Env, caller: Address, investor: Address, amount: i128) {
        caller.require_auth();
        assert!(amount > 0, "amount must be positive");

        let gp: Address = env.storage().instance().get(&DataKey::Gp).unwrap();
        let minter = env.storage().instance().get::<_, Address>(&DataKey::AuthorizedMinter);

        let is_authorized = caller == gp || (minter.is_some() && caller == minter.unwrap());
        assert!(is_authorized, "caller is not authorized to mint");

        let is_whitelisted = env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::Whitelisted(investor.clone()))
            .unwrap_or(false);
        assert!(is_whitelisted, "investor is not whitelisted");

        // Mint LP tokens via the Stellar Asset Contract
        let lp_token_address: Address = env.storage().instance().get(&DataKey::LpToken).unwrap();
        let lp_token_client = token::StellarAssetClient::new(&env, &lp_token_address);
        lp_token_client.mint(&investor, &amount);

        // Update contribution records
        let current_contribution = env
            .storage()
            .persistent()
            .get::<_, i128>(&DataKey::Contribution(investor.clone()))
            .unwrap_or(0);
        env.storage().persistent().set(&DataKey::Contribution(investor), &(current_contribution + amount));

        let total_contributed: i128 = env.storage().instance().get(&DataKey::TotalContributed).unwrap();
        env.storage().instance().set(&DataKey::TotalContributed, &(total_contributed + amount));
    }

    pub fn clawback_lp_tokens(env: Env, gp: Address, investor: Address, amount: i128) {
        gp.require_auth();
        let current_gp: Address = env.storage().instance().get(&DataKey::Gp).unwrap();
        assert_eq!(gp, current_gp, "only GP can clawback");

        let lp_token_address: Address = env.storage().instance().get(&DataKey::LpToken).unwrap();
        let lp_token_client = token::StellarAssetClient::new(&env, &lp_token_address);
        lp_token_client.clawback(&investor, &amount);

        let current_contribution = env
            .storage()
            .persistent()
            .get::<_, i128>(&DataKey::Contribution(investor.clone()))
            .unwrap_or(0);
        assert!(current_contribution >= amount, "clawback amount exceeds contribution");
        env.storage().persistent().set(&DataKey::Contribution(investor), &(current_contribution - amount));

        let total_contributed: i128 = env.storage().instance().get(&DataKey::TotalContributed).unwrap();
        env.storage().instance().set(&DataKey::TotalContributed, &(total_contributed - amount));
    }

    pub fn set_status(env: Env, gp: Address, status: FundStatus) {
        gp.require_auth();
        let current_gp: Address = env.storage().instance().get(&DataKey::Gp).unwrap();
        assert_eq!(gp, current_gp, "only GP can set status");

        env.storage().instance().set(&DataKey::Status, &status);
    }

    // --- Getters ---

    pub fn get_gp(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Gp).unwrap()
    }

    pub fn get_usdc_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::UsdcToken).unwrap()
    }

    pub fn get_lp_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::LpToken).unwrap()
    }

    pub fn get_fund_name(env: Env) -> soroban_sdk::String {
        env.storage().instance().get(&DataKey::FundName).unwrap()
    }

    pub fn get_target_size(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TargetSize).unwrap()
    }

    pub fn get_total_committed(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalCommitted).unwrap()
    }

    pub fn get_total_contributed(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalContributed).unwrap()
    }

    pub fn is_whitelisted(env: Env, investor: Address) -> bool {
        env.storage().persistent().get(&DataKey::Whitelisted(investor)).unwrap_or(false)
    }

    pub fn get_commitment(env: Env, investor: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Commitment(investor)).unwrap_or(0)
    }

    pub fn get_contribution(env: Env, investor: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Contribution(investor)).unwrap_or(0)
    }

    pub fn get_status(env: Env) -> FundStatus {
        env.storage().instance().get(&DataKey::Status).unwrap()
    }
}

#[cfg(test)]
mod test;

