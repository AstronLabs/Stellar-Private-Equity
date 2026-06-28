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
fn test_secondary_market_trade_success() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let token_admin = Address::generate(&env);

    // Setup USDC and LP token mocks
    let (usdc_address, _usdc_sac, usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, lp_sac, lp_client) = setup_test_token(&env, &gp);
    
    lp_sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    lp_sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    // Deploy and initialize Fund Contract
    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);
    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &1_000_000);

    // Deploy and initialize Secondary Market Contract
    let market_id = env.register_contract(None, SecondaryMarketContract);
    let market_client = SecondaryMarketContractClient::new(&env, &market_id);
    market_client.initialize(&fund_id, &usdc_address, &lp_address);

    // Setup Seller and Buyer
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);

    fund_client.whitelist_investor(&gp, &seller);
    fund_client.whitelist_investor(&gp, &buyer);

    // Mint LP tokens to Seller
    fund_client.mint_lp_tokens(&gp, &seller, &50_000);
    assert_eq!(lp_client.balance(&seller), 50_000);

    // Mint USDC to Buyer
    let usdc_sac_client = token::StellarAssetClient::new(&env, &usdc_address);
    usdc_sac_client.mint(&buyer, &60_000);
    assert_eq!(usdc_client.balance(&buyer), 60_000);

    // Seller creates listing of 40,000 LP tokens for 40,000 USDC
    let listing_id = market_client.create_listing(&seller, &40_000, &40_000);
    assert_eq!(market_client.get_listing_count(), 1);
    
    // LP tokens should be escrowed
    assert_eq!(lp_client.balance(&seller), 10_000);
    assert_eq!(lp_client.balance(&market_id), 40_000);

    let listing = market_client.get_listing(&listing_id).unwrap();
    assert_eq!(listing.seller, seller);
    assert_eq!(listing.lp_amount, 40_000);
    assert_eq!(listing.price_usdc, 40_000);
    assert!(matches!(listing.status, ListingStatus::Active));

    // Buyer buys the listing
    market_client.buy_listing(&buyer, &listing_id);

    // Verify atomic swap results
    assert_eq!(lp_client.balance(&buyer), 40_000);
    assert_eq!(lp_client.balance(&market_id), 0);
    assert_eq!(usdc_client.balance(&buyer), 20_000);
    assert_eq!(usdc_client.balance(&seller), 40_000);

    let listing = market_client.get_listing(&listing_id).unwrap();
    assert!(matches!(listing.status, ListingStatus::Bought));
}

#[test]
fn test_secondary_market_cancel() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let (usdc_address, _usdc_sac, _usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, lp_sac, lp_client) = setup_test_token(&env, &gp);
    
    lp_sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    lp_sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);
    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &1_000_000);

    let market_id = env.register_contract(None, SecondaryMarketContract);
    let market_client = SecondaryMarketContractClient::new(&env, &market_id);
    market_client.initialize(&fund_id, &usdc_address, &lp_address);

    let seller = Address::generate(&env);
    fund_client.whitelist_investor(&gp, &seller);
    fund_client.mint_lp_tokens(&gp, &seller, &50_000);

    // Create listing
    let listing_id = market_client.create_listing(&seller, &40_000, &40_000);
    assert_eq!(lp_client.balance(&seller), 10_000);

    // Cancel listing
    market_client.cancel_listing(&seller, &listing_id);

    // Verify refund of LP tokens
    assert_eq!(lp_client.balance(&seller), 50_000);
    assert_eq!(lp_client.balance(&market_id), 0);

    let listing = market_client.get_listing(&listing_id).unwrap();
    assert!(matches!(listing.status, ListingStatus::Cancelled));
}

#[test]
fn test_secondary_market_compliance_enforced() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let (usdc_address, _usdc_sac, _usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, lp_sac, _lp_client) = setup_test_token(&env, &gp);
    
    lp_sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    lp_sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);
    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &1_000_000);

    let market_id = env.register_contract(None, SecondaryMarketContract);
    let market_client = SecondaryMarketContractClient::new(&env, &market_id);
    market_client.initialize(&fund_id, &usdc_address, &lp_address);

    let seller = Address::generate(&env);
    let bad_buyer = Address::generate(&env); // Not whitelisted

    fund_client.whitelist_investor(&gp, &seller);
    fund_client.mint_lp_tokens(&gp, &seller, &50_000);

    let usdc_sac_client = token::StellarAssetClient::new(&env, &usdc_address);
    usdc_sac_client.mint(&bad_buyer, &60_000);

    let listing_id = market_client.create_listing(&seller, &40_000, &40_000);

    // Try to buy without being whitelisted (should fail)
    let res = market_client.try_buy_listing(&bad_buyer, &listing_id);
    assert!(res.is_err());
}
