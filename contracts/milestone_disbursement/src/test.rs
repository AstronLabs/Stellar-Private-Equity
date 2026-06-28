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
fn test_milestone_disbursement_success() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let portfolio_company = Address::generate(&env);
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

    // Deploy and initialize Milestone Disbursement Contract
    let md_id = env.register_contract(None, MilestoneDisbursementContract);
    let md_client = MilestoneDisbursementContractClient::new(&env, &md_id);
    md_client.initialize(&fund_id, &usdc_address);

    // Create LPs with LP token balances (voting power)
    let lp1 = Address::generate(&env);
    let lp2 = Address::generate(&env);

    fund_client.whitelist_investor(&gp, &lp1);
    fund_client.whitelist_investor(&gp, &lp2);

    fund_client.mint_lp_tokens(&gp, &lp1, &60_000);
    fund_client.mint_lp_tokens(&gp, &lp2, &40_000);

    assert_eq!(lp_client.balance(&lp1), 60_000);
    assert_eq!(lp_client.balance(&lp2), 40_000);

    // Mint USDC to GP so they can escrow funds for the proposal
    let usdc_sac_client = token::StellarAssetClient::new(&env, &usdc_address);
    usdc_sac_client.mint(&gp, &50_000);
    assert_eq!(usdc_client.balance(&gp), 50_000);

    // GP proposes milestone disbursement of 20,000 USDC
    let desc = soroban_sdk::String::from_str(&env, "Product Launch Milestone");
    let proposal_id = md_client.propose_milestone(&gp, &portfolio_company, &20_000, &desc, &3600);

    assert_eq!(md_client.get_proposal_count(), 1);
    assert_eq!(usdc_client.balance(&gp), 30_000);
    assert_eq!(usdc_client.balance(&md_id), 20_000); // escrowed

    let proposal = md_client.get_proposal(&proposal_id).unwrap();
    assert_eq!(proposal.amount, 20_000);
    assert_eq!(proposal.votes_yes, 0);
    assert_eq!(proposal.votes_no, 0);
    assert!(matches!(proposal.status, ProposalStatus::Voting));

    // LPs vote
    md_client.vote(&lp1, &proposal_id, &true);  // 60,000 YES
    md_client.vote(&lp2, &proposal_id, &false); // 40,000 NO

    assert!(md_client.has_voted(&proposal_id, &lp1));
    assert!(md_client.has_voted(&proposal_id, &lp2));

    let proposal = md_client.get_proposal(&proposal_id).unwrap();
    assert_eq!(proposal.votes_yes, 60_000);
    assert_eq!(proposal.votes_no, 40_000);

    // Fast forward ledger time to after due_date
    env.ledger().set_timestamp(env.ledger().timestamp() + 3601);

    // Execute disbursement
    md_client.execute_disbursement(&gp, &proposal_id);

    let proposal = md_client.get_proposal(&proposal_id).unwrap();
    assert!(matches!(proposal.status, ProposalStatus::Disbursed));

    // Verify portfolio company received USDC
    assert_eq!(usdc_client.balance(&portfolio_company), 20_000);
    assert_eq!(usdc_client.balance(&md_id), 0);
}

#[test]
fn test_milestone_disbursement_rejection_and_refund() {
    let env = Env::default();
    env.mock_all_auths();

    let gp = Address::generate(&env);
    let portfolio_company = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let (usdc_address, _usdc_sac, usdc_client) = setup_test_token(&env, &token_admin);
    let (lp_address, lp_sac, _lp_client) = setup_test_token(&env, &gp);
    
    lp_sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    lp_sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    let fund_id = env.register_contract(None, FundContract);
    let fund_client = FundContractClient::new(&env, &fund_id);
    let fund_name = soroban_sdk::String::from_str(&env, "Stellar Growth Fund I");
    fund_client.initialize(&gp, &usdc_address, &lp_address, &fund_name, &1_000_000);

    let md_id = env.register_contract(None, MilestoneDisbursementContract);
    let md_client = MilestoneDisbursementContractClient::new(&env, &md_id);
    md_client.initialize(&fund_id, &usdc_address);

    let lp1 = Address::generate(&env);
    let lp2 = Address::generate(&env);

    fund_client.whitelist_investor(&gp, &lp1);
    fund_client.whitelist_investor(&gp, &lp2);

    fund_client.mint_lp_tokens(&gp, &lp1, &30_000);
    fund_client.mint_lp_tokens(&gp, &lp2, &70_000);

    let usdc_sac_client = token::StellarAssetClient::new(&env, &usdc_address);
    usdc_sac_client.mint(&gp, &50_000);

    // GP proposes milestone disbursement of 20,000 USDC
    let desc = soroban_sdk::String::from_str(&env, "Product Launch Milestone");
    let proposal_id = md_client.propose_milestone(&gp, &portfolio_company, &20_000, &desc, &3600);

    // LPs vote (majority NO)
    md_client.vote(&lp1, &proposal_id, &true);  // 30,000 YES
    md_client.vote(&lp2, &proposal_id, &false); // 70,000 NO

    // Fast forward ledger time
    env.ledger().set_timestamp(env.ledger().timestamp() + 3601);

    // Execute disbursement (will reject)
    md_client.execute_disbursement(&gp, &proposal_id);

    let proposal = md_client.get_proposal(&proposal_id).unwrap();
    assert!(matches!(proposal.status, ProposalStatus::Rejected));
    assert_eq!(usdc_client.balance(&portfolio_company), 0);
    assert_eq!(usdc_client.balance(&md_id), 20_000); // still in escrow

    // GP claims refund
    md_client.refund_failed_proposal(&gp, &proposal_id);

    let proposal = md_client.get_proposal(&proposal_id).unwrap();
    assert!(matches!(proposal.status, ProposalStatus::Refunded));
    assert_eq!(usdc_client.balance(&gp), 50_000); // refunded
    assert_eq!(usdc_client.balance(&md_id), 0);
}
