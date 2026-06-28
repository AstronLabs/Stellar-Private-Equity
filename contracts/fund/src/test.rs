#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger, StellarAssetContract, IssuerFlags},
    token, Address, Env,
};

fn setup_test_token<'a>(env: &'a Env, admin: &Address) -> (Address, StellarAssetContract, token::Client<'a>) {
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token_client = token::Client::new(env, &sac.address());
    (sac.address(), sac, token_client)
}

#[test]
fn test_fund_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let token_admin = Address::generate(&env);
    
    // Setup USDC and LP token mocks as Stellar Asset Contracts
    let (usdc_address, _usdc_sac, _usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, lp_sac, lp_client) = setup_test_token(&env, &gp); // GP is the admin of LP token to allow clawbacks
    
    // Enable clawback on the LP token
    lp_sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    lp_sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);

    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    let target_size = 1_000_000i128;

    // Initialize
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &target_size);

    assert_eq!(fund_client.get_gp(), gp);
    assert_eq!(fund_client.get_usdc_token(), usdc_address);
    assert_eq!(fund_client.get_lp_token(), lp_address);
    assert_eq!(fund_client.get_target_size(), target_size);
    assert!(matches!(fund_client.get_status(), FundStatus::Fundraising));

    // Whitelist investor
    let investor = Address::generate(&env);
    assert_eq!(fund_client.is_whitelisted(&investor), false);

    fund_client.whitelist_investor(&gp, &investor);
    assert_eq!(fund_client.is_whitelisted(&investor), true);

    // Commit capital
    fund_client.commit_capital(&investor, &100_000);
    assert_eq!(fund_client.get_commitment(&investor), 100_000);
    assert_eq!(fund_client.get_total_committed(), 100_000);

    // Try committing while not whitelisted
    let bad_investor = Address::generate(&env);
    let res = fund_client.try_commit_capital(&bad_investor, &50_000);
    assert!(res.is_err());

    // Mint LP tokens
    fund_client.mint_lp_tokens(&gp, &investor, &50_000);
    assert_eq!(lp_client.balance(&investor), 50_000);
    assert_eq!(fund_client.get_contribution(&investor), 50_000);
    assert_eq!(fund_client.get_total_contributed(), 50_000);

    // Clawback LP tokens
    fund_client.clawback_lp_tokens(&gp, &investor, &10_000);
    assert_eq!(lp_client.balance(&investor), 40_000);
    assert_eq!(fund_client.get_contribution(&investor), 40_000);
    assert_eq!(fund_client.get_total_contributed(), 40_000);

    // Transition status to Active
    fund_client.set_status(&gp, &FundStatus::Active);
    assert!(matches!(fund_client.get_status(), FundStatus::Active));

    // Cannot commit after fundraising
    let res = fund_client.try_commit_capital(&investor, &10_000);
    assert!(res.is_err());
}
