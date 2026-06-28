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
fn test_capital_call_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let gp_treasury = Address::generate(&env);
    let token_admin = Address::generate(&env);

    // Setup USDC and LP token mocks as Stellar Asset Contracts
    let (usdc_address, _usdc_sac, usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, lp_sac, lp_client) = setup_test_token(&env, &gp);
    
    // Enable clawback on the LP token
    lp_sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    lp_sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    // Deploy and initialize Fund Contract
    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);
    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &1_000_000);

    // Deploy and initialize Capital Call Contract
    let cc_id = env.register_contract(None, CapitalCallContract);
    let cc_client = CapitalCallContractClient::new(&env, &cc_id);
    cc_client.initialize(&fund_id, &gp_treasury, &usdc_address, &lp_address);

    // Authorize Capital Call Contract to mint LP tokens
    fund_client.set_authorized_minter(&gp, &cc_id);

    // Whitelist and set commitment for LP
    let lp = Address::generate(&env);
    fund_client.whitelist_investor(&gp, &lp);
    fund_client.commit_capital(&lp, &100_000);

    // Mint some USDC to LP for contribution
    let usdc_sac_client = token::StellarAssetClient::new(&env, &usdc_address);
    usdc_sac_client.mint(&lp, &200_000);
    assert_eq!(usdc_client.balance(&lp), 200_000);

    // Create capital call of 50,000 USDC
    let call_id = cc_client.create_capital_call(&gp, &50_000, &3600);
    assert_eq!(cc_client.get_call_count(), 1);

    let call_info = cc_client.get_call_info(&call_id).unwrap();
    assert_eq!(call_info.amount_requested, 50_000);
    assert_eq!(call_info.amount_collected, 0);
    assert!(matches!(call_info.status, CallStatus::Pending));

    // Contribute to capital call
    cc_client.contribute(&lp, &call_id, &50_000);
    
    let call_info = cc_client.get_call_info(&call_id).unwrap();
    assert_eq!(call_info.amount_collected, 50_000);
    assert!(matches!(call_info.status, CallStatus::Subscribed));
    assert_eq!(usdc_client.balance(&lp), 150_000);
    assert_eq!(usdc_client.balance(&cc_id), 50_000);

    // Drawdown
    cc_client.drawdown(&gp, &call_id);

    let call_info = cc_client.get_call_info(&call_id).unwrap();
    assert!(matches!(call_info.status, CallStatus::DrawnDown));

    // Verify GP Treasury received USDC
    assert_eq!(usdc_client.balance(&gp_treasury), 50_000);
    assert_eq!(usdc_client.balance(&cc_id), 0);

    // Verify LP automatically received LP tokens
    assert_eq!(lp_client.balance(&lp), 50_000);
    assert_eq!(fund_client.get_contribution(&lp), 50_000);
}

#[test]
fn test_capital_call_refunds() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let gp_treasury = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let (usdc_address, _usdc_sac, usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, _lp_sac, _lp_client) = setup_test_token(&env, &gp);

    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);
    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &1_000_000);

    let cc_id = env.register_contract(None, CapitalCallContract);
    let cc_client = CapitalCallContractClient::new(&env, &cc_id);
    cc_client.initialize(&fund_id, &gp_treasury, &usdc_address, &lp_address);

    let lp = Address::generate(&env);
    fund_client.whitelist_investor(&gp, &lp);
    fund_client.commit_capital(&lp, &100_000);

    let usdc_sac_client = token::StellarAssetClient::new(&env, &usdc_address);
    usdc_sac_client.mint(&lp, &100_000);

    // Create capital call
    let call_id = cc_client.create_capital_call(&gp, &50_000, &3600);

    // Contribute 30,000 USDC
    cc_client.contribute(&lp, &call_id, &30_000);

    // Cancel the capital call
    cc_client.cancel_capital_call(&gp, &call_id);

    let call_info = cc_client.get_call_info(&call_id).unwrap();
    assert!(matches!(call_info.status, CallStatus::Cancelled));

    // Claim refund
    cc_client.claim_refund(&lp, &call_id);

    // Verify refund received
    assert_eq!(usdc_client.balance(&lp), 100_000);
    assert_eq!(usdc_client.balance(&cc_id), 0);
}
