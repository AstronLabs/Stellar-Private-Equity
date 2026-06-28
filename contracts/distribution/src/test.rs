#![cfg(test)]

use super::*;
use pe_fund::{FundContract, FundContractClient};
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
fn test_distribution_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let token_admin = Address::generate(&env);

    // Setup USDC and LP token mocks
    let (usdc_address, _usdc_sac, usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, lp_sac, _lp_client) = setup_test_token(&env, &gp);
    
    lp_sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    lp_sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    // Deploy and initialize Fund Contract
    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);
    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &1_000_000);

    // Deploy and initialize Distribution Contract
    let dist_id = env.register_contract(None, DistributionContract);
    let dist_client = DistributionContractClient::new(&env, &dist_id);
    dist_client.initialize(&fund_id, &usdc_address, &lp_address);

    // Set up LPs and mint LP tokens (representing fund contributions)
    let lp1 = Address::generate(&env);
    let lp2 = Address::generate(&env);

    fund_client.whitelist_investor(&gp, &lp1);
    fund_client.whitelist_investor(&gp, &lp2);

    fund_client.mint_lp_tokens(&gp, &lp1, &60_000);
    fund_client.mint_lp_tokens(&gp, &lp2, &40_000);

    // Mint USDC to GP for distribution
    let usdc_sac_client = token::StellarAssetClient::new(&env, &usdc_address);
    usdc_sac_client.mint(&gp, &50_000);

    // GP creates a distribution of 50,000 USDC
    let dist_id_val = dist_client.create_distribution(&gp, &50_000);
    assert_eq!(dist_client.get_distribution_count(), 1);
    assert_eq!(usdc_client.balance(&dist_id), 50_000);

    let info = dist_client.get_distribution(&dist_id_val).unwrap();
    assert_eq!(info.total_amount, 50_000);
    assert_eq!(info.total_lp_supply, 100_000);

    // LP1 claims their share (60% -> 30,000 USDC)
    let claimed_lp1 = dist_client.claim_distribution(&lp1, &dist_id_val);
    assert_eq!(claimed_lp1, 30_000);
    assert_eq!(usdc_client.balance(&lp1), 30_000);
    assert!(dist_client.has_claimed(&dist_id_val, &lp1));

    // LP2 claims their share (40% -> 20,000 USDC)
    let claimed_lp2 = dist_client.claim_distribution(&lp2, &dist_id_val);
    assert_eq!(claimed_lp2, 20_000);
    assert_eq!(usdc_client.balance(&lp2), 20_000);
    assert!(dist_client.has_claimed(&dist_id_val, &lp2));

    // Verify escrow is now empty
    assert_eq!(usdc_client.balance(&dist_id), 0);
}
