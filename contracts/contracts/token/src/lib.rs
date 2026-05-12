#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// ── Storage Keys ────────────────────────────────────────────────
#[contracttype]
pub enum DataKey {
    Admin,
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    /// Balance(address)
    Balance(Address),
    /// Allowance(owner, spender)
    Allowance(Address, Address),
}

#[contract]
pub struct InsuranceToken;

#[contractimpl]
impl InsuranceToken {
    /// Initialize the insurance token.
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    /// Mint tokens to an address. Admin only.
    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) {
        admin.require_auth();
        Self::require_admin(&env, &admin);
        assert!(amount > 0, "amount must be positive");

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(balance + amount));

        let supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(supply + amount));
    }

    /// Transfer tokens between addresses.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "amount must be positive");

        let from_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        assert!(from_balance >= amount, "insufficient balance");

        let to_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(to_balance + amount));
    }

    /// Approve a spender to transfer tokens on behalf of the owner.
    pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) {
        owner.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Allowance(owner, spender), &amount);
    }

    /// Transfer tokens using an allowance.
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        assert!(amount > 0, "amount must be positive");

        let allowance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Allowance(from.clone(), spender.clone()))
            .unwrap_or(0);
        assert!(allowance >= amount, "insufficient allowance");

        let from_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        assert!(from_balance >= amount, "insufficient balance");

        let to_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(to_balance + amount));
        env.storage()
            .persistent()
            .set(&DataKey::Allowance(from, spender), &(allowance - amount));
    }

    /// Burn tokens. Only the holder can burn their own tokens.
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "amount must be positive");

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        assert!(balance >= amount, "insufficient balance");

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(balance - amount));

        let supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(supply - amount));
    }

    // ── View Functions ──────────────────────────────────────────

    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(addr))
            .unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(owner, spender))
            .unwrap_or(0)
    }

    pub fn name(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Name)
            .expect("not initialized")
    }

    pub fn symbol(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Symbol)
            .expect("not initialized")
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Decimals)
            .unwrap_or(7)
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
    use soroban_sdk::{Env, String};

    #[test]
    fn test_mint_transfer_burn() {
        let env = Env::default();
        let contract_id = env.register(InsuranceToken, ());
        let client = InsuranceTokenClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(
            &admin,
            &String::from_str(&env, "NexusGuard Token"),
            &String::from_str(&env, "NGT"),
            &7u32,
        );

        // Mint
        client.mint(&admin, &alice, &1_000_000_000i128);
        assert_eq!(client.balance(&alice), 1_000_000_000i128);
        assert_eq!(client.total_supply(), 1_000_000_000i128);

        // Transfer
        client.transfer(&alice, &bob, &300_000_000i128);
        assert_eq!(client.balance(&alice), 700_000_000i128);
        assert_eq!(client.balance(&bob), 300_000_000i128);

        // Burn
        client.burn(&bob, &100_000_000i128);
        assert_eq!(client.balance(&bob), 200_000_000i128);
        assert_eq!(client.total_supply(), 900_000_000i128);
    }
}
