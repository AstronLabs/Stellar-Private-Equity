#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// ── Storage Keys ────────────────────────────────────────────────
#[contracttype]
pub enum DataKey {
    Admin,
    VotingContract,
    ClaimsContract,
    PayoutContract,
    /// Governance action log: Action(action_id)
    Action(u64),
    ActionCount,
    /// Emergency mode flag
    EmergencyMode,
    /// Emergency signers count required
    EmergencyThreshold,
}

// ── Action Types ────────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    ApproveClaim,
    RejectClaim,
    EmergencyPayout,
    FreezePool,
    UpdateParameter,
}

// ── Action Status ───────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ActionStatus {
    Pending,
    Executed,
    Failed,
}

// ── Governance Action ───────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug)]
pub struct GovernanceAction {
    pub id: u64,
    pub action_type: ActionType,
    pub proposal_id: u64,
    pub executor: Address,
    pub status: ActionStatus,
    pub executed_at: u64,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// Initialize governance with linked contracts.
    pub fn initialize(
        env: Env,
        admin: Address,
        voting_contract: Address,
        claims_contract: Address,
        payout_contract: Address,
        emergency_threshold: u32,
    ) {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::VotingContract, &voting_contract);
        env.storage().instance().set(&DataKey::ClaimsContract, &claims_contract);
        env.storage().instance().set(&DataKey::PayoutContract, &payout_contract);
        env.storage().instance().set(&DataKey::ActionCount, &0u64);
        env.storage().instance().set(&DataKey::EmergencyMode, &false);
        env.storage().instance().set(&DataKey::EmergencyThreshold, &emergency_threshold);
    }

    /// Execute a governance action after a proposal passes voting.
    /// In production, this would cross-contract call the voting contract
    /// to verify the proposal actually passed before executing.
    pub fn execute_action(
        env: Env,
        executor: Address,
        action_type: ActionType,
        proposal_id: u64,
    ) -> u64 {
        executor.require_auth();
        Self::require_admin(&env, &executor);

        // TODO: Cross-contract call to verify proposal passed
        // let voting: Address = env.storage().instance().get(&DataKey::VotingContract)...;
        // voting_client.get_proposal(&proposal_id).status == Passed

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::ActionCount)
            .unwrap_or(0);

        let action_id = count;
        let now = env.ledger().timestamp();

        let action = GovernanceAction {
            id: action_id,
            action_type,
            proposal_id,
            executor,
            status: ActionStatus::Executed,
            executed_at: now,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Action(action_id), &action);

        count += 1;
        env.storage().instance().set(&DataKey::ActionCount, &count);

        action_id
    }

    /// Toggle emergency mode — enables fast-track payouts.
    pub fn set_emergency_mode(env: Env, admin: Address, enabled: bool) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        env.storage().instance().set(&DataKey::EmergencyMode, &enabled);
    }

    /// Execute an emergency payout — bypasses normal voting when in emergency mode.
    pub fn emergency_payout(
        env: Env,
        admin: Address,
        claim_id: u64,
    ) -> u64 {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let is_emergency: bool = env
            .storage()
            .instance()
            .get(&DataKey::EmergencyMode)
            .unwrap_or(false);
        assert!(is_emergency, "emergency mode not active");

        // Log the emergency action
        Self::execute_action(
            env,
            admin,
            ActionType::EmergencyPayout,
            claim_id,
        )
    }

    // ── View Functions ──────────────────────────────────────────

    pub fn get_action(env: Env, action_id: u64) -> GovernanceAction {
        env.storage()
            .persistent()
            .get(&DataKey::Action(action_id))
            .expect("action not found")
    }

    pub fn action_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ActionCount)
            .unwrap_or(0)
    }

    pub fn is_emergency_mode(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::EmergencyMode)
            .unwrap_or(false)
    }

    // ── Internal ────────────────────────────────────────────────

    fn require_admin(env: &Env, addr: &Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        assert!(*addr == admin, "unauthorized: not admin");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_governance_actions() {
        let env = Env::default();
        let contract_id = env.register(GovernanceContract, ());
        let client = GovernanceContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let voting = Address::generate(&env);
        let claims = Address::generate(&env);
        let payout = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin, &voting, &claims, &payout, &3);

        let aid = client.execute_action(&admin, &ActionType::ApproveClaim, &0u64);
        assert_eq!(aid, 0);
        assert_eq!(client.action_count(), 1);

        let action = client.get_action(&0);
        assert_eq!(action.action_type, ActionType::ApproveClaim);
        assert_eq!(action.status, ActionStatus::Executed);
    }

    #[test]
    fn test_emergency_mode() {
        let env = Env::default();
        let contract_id = env.register(GovernanceContract, ());
        let client = GovernanceContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        env.mock_all_auths();

        client.initialize(
            &admin,
            &Address::generate(&env),
            &Address::generate(&env),
            &Address::generate(&env),
            &2,
        );

        assert!(!client.is_emergency_mode());
        client.set_emergency_mode(&admin, &true);
        assert!(client.is_emergency_mode());

        let aid = client.emergency_payout(&admin, &42);
        let action = client.get_action(&aid);
        assert_eq!(action.action_type, ActionType::EmergencyPayout);
    }
}
