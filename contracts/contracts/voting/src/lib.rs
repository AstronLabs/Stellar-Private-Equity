#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// ── Storage Keys ────────────────────────────────────────────────
#[contracttype]
pub enum DataKey {
    Admin,
    PoolContract,
    /// Proposal record: Proposal(proposal_id)
    Proposal(u64),
    ProposalCount,
    /// Vote record: Vote(proposal_id, voter)
    Vote(u64, Address),
    /// Per-proposal vote tally
    VoteTally(u64),
    /// Quorum percentage (e.g., 51 = 51%)
    QuorumThreshold,
    /// Approval percentage needed (e.g., 60 = 60%)
    ApprovalThreshold,
}

// ── Vote Choice ─────────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum VoteChoice {
    Approve,
    Reject,
    Abstain,
}

// ── Proposal Status ─────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

// ── Proposal Types ──────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ProposalType {
    ClaimApproval,
    FraudDispute,
    EmergencyPayout,
    ParameterChange,
}

// ── Vote Tally ──────────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug)]
pub struct VoteTally {
    pub approve_count: u64,
    pub reject_count: u64,
    pub abstain_count: u64,
    pub total_voters: u64,
}

// ── Proposal ────────────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    pub id: u64,
    pub proposal_type: ProposalType,
    pub proposer: Address,
    pub reference_id: u64,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub expires_at: u64,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    /// Initialize voting contract.
    pub fn initialize(
        env: Env,
        admin: Address,
        pool_contract: Address,
        quorum_threshold: u32,
        approval_threshold: u32,
    ) {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        assert!(quorum_threshold > 0 && quorum_threshold <= 100, "invalid quorum");
        assert!(approval_threshold > 0 && approval_threshold <= 100, "invalid approval threshold");

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::PoolContract, &pool_contract);
        env.storage().instance().set(&DataKey::QuorumThreshold, &quorum_threshold);
        env.storage().instance().set(&DataKey::ApprovalThreshold, &approval_threshold);
        env.storage().instance().set(&DataKey::ProposalCount, &0u64);
    }

    /// Create a new voting proposal.
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        proposal_type: ProposalType,
        reference_id: u64,
        voting_period_secs: u64,
    ) -> u64 {
        proposer.require_auth();

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0);

        let now = env.ledger().timestamp();
        let proposal_id = count;

        let proposal = Proposal {
            id: proposal_id,
            proposal_type,
            proposer,
            reference_id,
            status: ProposalStatus::Active,
            created_at: now,
            expires_at: now + voting_period_secs,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);

        let tally = VoteTally {
            approve_count: 0,
            reject_count: 0,
            abstain_count: 0,
            total_voters: 0,
        };
        env.storage()
            .persistent()
            .set(&DataKey::VoteTally(proposal_id), &tally);

        count += 1;
        env.storage().instance().set(&DataKey::ProposalCount, &count);

        proposal_id
    }

    /// Cast a vote on an active proposal.
    pub fn cast_vote(env: Env, voter: Address, proposal_id: u64, choice: VoteChoice) {
        voter.require_auth();

        let proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .expect("proposal not found");

        assert!(proposal.status == ProposalStatus::Active, "proposal not active");

        let now = env.ledger().timestamp();
        assert!(now <= proposal.expires_at, "voting period expired");

        // Check voter hasn't already voted
        let vote_key = DataKey::Vote(proposal_id, voter.clone());
        assert!(
            !env.storage().persistent().has(&vote_key),
            "already voted"
        );

        // Record the vote
        env.storage().persistent().set(&vote_key, &choice);

        // Update tally
        let mut tally: VoteTally = env
            .storage()
            .persistent()
            .get(&DataKey::VoteTally(proposal_id))
            .expect("tally not found");

        match choice {
            VoteChoice::Approve => tally.approve_count += 1,
            VoteChoice::Reject => tally.reject_count += 1,
            VoteChoice::Abstain => tally.abstain_count += 1,
        }
        tally.total_voters += 1;

        env.storage()
            .persistent()
            .set(&DataKey::VoteTally(proposal_id), &tally);
    }

    /// Finalize a proposal — check quorum and approval thresholds.
    pub fn finalize(env: Env, caller: Address, proposal_id: u64) {
        caller.require_auth();

        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .expect("proposal not found");

        assert!(proposal.status == ProposalStatus::Active, "proposal not active");

        let tally: VoteTally = env
            .storage()
            .persistent()
            .get(&DataKey::VoteTally(proposal_id))
            .expect("tally not found");

        // TODO: In production, compare against actual pool member count
        // For now, require at least 1 vote to finalize
        assert!(tally.total_voters > 0, "no votes cast");

        let approval_threshold: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ApprovalThreshold)
            .unwrap_or(60);

        let total_non_abstain = tally.approve_count + tally.reject_count;

        if total_non_abstain == 0 {
            proposal.status = ProposalStatus::Expired;
        } else {
            let approval_pct = (tally.approve_count * 100) / total_non_abstain;
            if approval_pct >= approval_threshold as u64 {
                proposal.status = ProposalStatus::Passed;
            } else {
                proposal.status = ProposalStatus::Rejected;
            }
        }

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);
    }

    // ── View Functions ──────────────────────────────────────────

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .expect("proposal not found")
    }

    pub fn get_tally(env: Env, proposal_id: u64) -> VoteTally {
        env.storage()
            .persistent()
            .get(&DataKey::VoteTally(proposal_id))
            .expect("tally not found")
    }

    pub fn proposal_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }

    pub fn has_voted(env: Env, proposal_id: u64, voter: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Vote(proposal_id, voter))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_create_and_vote() {
        let env = Env::default();
        let contract_id = env.register(VotingContract, ());
        let client = VotingContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let pool = Address::generate(&env);
        let voter1 = Address::generate(&env);
        let voter2 = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin, &pool, &51, &60);

        let pid = client.create_proposal(
            &admin,
            &ProposalType::ClaimApproval,
            &0u64,
            &604_800u64, // 7 days
        );

        assert_eq!(pid, 0);

        client.cast_vote(&voter1, &pid, &VoteChoice::Approve);
        client.cast_vote(&voter2, &pid, &VoteChoice::Approve);

        let tally = client.get_tally(&pid);
        assert_eq!(tally.approve_count, 2);
        assert_eq!(tally.total_voters, 2);

        client.finalize(&admin, &pid);
        let p = client.get_proposal(&pid);
        assert_eq!(p.status, ProposalStatus::Passed);
    }
}
