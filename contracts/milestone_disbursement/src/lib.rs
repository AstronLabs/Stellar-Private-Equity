#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, String,
};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Voting,
    Approved,
    Rejected,
    Disbursed,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneProposal {
    pub id: u64,
    pub portfolio_company: Address,
    pub amount: i128,
    pub description: String,
    pub due_date: u64,
    pub votes_yes: i128,
    pub votes_no: i128,
    pub status: ProposalStatus,
}

#[contracttype]
pub enum DataKey {
    FundContract,
    UsdcToken,
    ProposalCount,
    Proposal(u64),
    LpVoted(u64, Address),
}

#[soroban_sdk::contractclient(name = "FundClient")]
pub trait FundInterface {
    fn get_gp(env: Env) -> Address;
    fn get_lp_token(env: Env) -> Address;
}

#[contract]
pub struct MilestoneDisbursementContract;

#[contractimpl]
impl MilestoneDisbursementContract {
    pub fn initialize(env: Env, fund_contract: Address, usdc_token: Address) {
        assert!(!env.storage().instance().has(&DataKey::FundContract), "already initialized");

        env.storage().instance().set(&DataKey::FundContract, &fund_contract);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&DataKey::ProposalCount, &0u64);
    }

    pub fn propose_milestone(
        env: Env,
        gp: Address,
        portfolio_company: Address,
        amount: i128,
        description: String,
        duration: u64,
    ) -> u64 {
        gp.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert_eq!(gp, fund_client.get_gp(), "only GP can propose milestone");
        assert!(amount > 0, "amount must be positive");
        assert!(duration > 0, "duration must be positive");

        // Escrow the USDC amount from GP to this contract
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        usdc_client.transfer(&gp, &env.current_contract_address(), &amount);

        let count: u64 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0);
        let proposal_id = count + 1;
        env.storage().instance().set(&DataKey::ProposalCount, &proposal_id);

        let proposal = MilestoneProposal {
            id: proposal_id,
            portfolio_company,
            amount,
            description,
            due_date: env.ledger().timestamp() + duration,
            votes_yes: 0,
            votes_no: 0,
            status: ProposalStatus::Voting,
        };

        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &proposal);

        proposal_id
    }

    pub fn vote(env: Env, lp: Address, proposal_id: u64, approve: bool) {
        lp.require_auth();

        let proposal_key = DataKey::Proposal(proposal_id);
        let mut proposal: MilestoneProposal = env
            .storage()
            .persistent()
            .get(&proposal_key)
            .expect("proposal not found");

        assert!(
            matches!(proposal.status, ProposalStatus::Voting),
            "proposal is not in voting phase"
        );
        assert!(
            env.ledger().timestamp() <= proposal.due_date,
            "voting period has expired"
        );

        let voted_key = DataKey::LpVoted(proposal_id, lp.clone());
        assert!(
            !env.storage().persistent().has(&voted_key),
            "LP has already voted"
        );

        // Get LP's voting power (their LP token balance)
        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        let lp_token_address = fund_client.get_lp_token();
        let lp_token_client = token::Client::new(&env, &lp_token_address);
        let voting_power = lp_token_client.balance(&lp);

        assert!(voting_power > 0, "only LPs with LP tokens can vote");

        if approve {
            proposal.votes_yes += voting_power;
        } else {
            proposal.votes_no += voting_power;
        }

        env.storage().persistent().set(&voted_key, &true);
        env.storage().persistent().set(&proposal_key, &proposal);
    }

    pub fn execute_disbursement(env: Env, gp: Address, proposal_id: u64) {
        gp.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert_eq!(gp, fund_client.get_gp(), "only GP can execute disbursement");

        let proposal_key = DataKey::Proposal(proposal_id);
        let mut proposal: MilestoneProposal = env
            .storage()
            .persistent()
            .get(&proposal_key)
            .expect("proposal not found");

        assert!(
            matches!(proposal.status, ProposalStatus::Voting),
            "proposal is not in voting phase"
        );
        assert!(
            env.ledger().timestamp() > proposal.due_date,
            "voting period has not ended yet"
        );

        if proposal.votes_yes > proposal.votes_no {
            proposal.status = ProposalStatus::Approved;
            
            // Disburse USDC to portfolio company
            let usdc_address = Self::get_usdc_token(env.clone());
            let usdc_client = token::Client::new(&env, &usdc_address);
            usdc_client.transfer(
                &env.current_contract_address(),
                &proposal.portfolio_company,
                &proposal.amount,
            );
            
            proposal.status = ProposalStatus::Disbursed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }

        env.storage().persistent().set(&proposal_key, &proposal);
    }

    pub fn refund_failed_proposal(env: Env, gp: Address, proposal_id: u64) {
        gp.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert_eq!(gp, fund_client.get_gp(), "only GP can claim refund");

        let proposal_key = DataKey::Proposal(proposal_id);
        let mut proposal: MilestoneProposal = env
            .storage()
            .persistent()
            .get(&proposal_key)
            .expect("proposal not found");

        assert!(
            matches!(proposal.status, ProposalStatus::Rejected),
            "can only refund rejected proposals"
        );

        // Refund USDC to GP
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        usdc_client.transfer(
            &env.current_contract_address(),
            &gp,
            &proposal.amount,
        );

        proposal.status = ProposalStatus::Refunded;
        env.storage().persistent().set(&proposal_key, &proposal);
    }

    // --- Getters ---

    pub fn get_fund_contract(env: Env) -> Address {
        env.storage().instance().get(&DataKey::FundContract).unwrap()
    }

    pub fn get_usdc_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::UsdcToken).unwrap()
    }

    pub fn get_proposal_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0)
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<MilestoneProposal> {
        env.storage().persistent().get(&DataKey::Proposal(proposal_id))
    }

    pub fn has_voted(env: Env, proposal_id: u64, lp: Address) -> bool {
        env.storage().persistent().has(&DataKey::LpVoted(proposal_id, lp))
    }
}

#[cfg(test)]
mod test;

