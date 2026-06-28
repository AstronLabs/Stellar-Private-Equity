#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env,
};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListingStatus {
    Active,
    Bought,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Listing {
    pub id: u64,
    pub seller: Address,
    pub lp_amount: i128,
    pub price_usdc: i128,
    pub status: ListingStatus,
}

#[contracttype]
pub enum DataKey {
    FundContract,
    UsdcToken,
    LpToken,
    ListingCount,
    Listing(u64),
}

#[soroban_sdk::contractclient(name = "FundClient")]
pub trait FundInterface {
    fn is_whitelisted(env: Env, investor: Address) -> bool;
}

#[contract]
pub struct SecondaryMarketContract;

#[contractimpl]
impl SecondaryMarketContract {
    pub fn initialize(
        env: Env,
        fund_contract: Address,
        usdc_token: Address,
        lp_token: Address,
    ) {
        assert!(!env.storage().instance().has(&DataKey::FundContract), "already initialized");

        env.storage().instance().set(&DataKey::FundContract, &fund_contract);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&DataKey::LpToken, &lp_token);
        env.storage().instance().set(&DataKey::ListingCount, &0u64);
    }

    pub fn create_listing(env: Env, seller: Address, lp_amount: i128, price_usdc: i128) -> u64 {
        seller.require_auth();
        assert!(lp_amount > 0, "LP amount must be positive");
        assert!(price_usdc > 0, "USDC price must be positive");

        // Escrow the LP tokens from seller to this contract
        let lp_token_address = Self::get_lp_token(env.clone());
        let lp_token_client = token::Client::new(&env, &lp_token_address);
        lp_token_client.transfer(&seller, &env.current_contract_address(), &lp_amount);

        let count: u64 = env.storage().instance().get(&DataKey::ListingCount).unwrap_or(0);
        let listing_id = count + 1;
        env.storage().instance().set(&DataKey::ListingCount, &listing_id);

        let listing = Listing {
            id: listing_id,
            seller,
            lp_amount,
            price_usdc,
            status: ListingStatus::Active,
        };

        env.storage().persistent().set(&DataKey::Listing(listing_id), &listing);

        listing_id
    }

    pub fn buy_listing(env: Env, buyer: Address, listing_id: u64) {
        buyer.require_auth();

        let fund_address = Self::get_fund_contract(env.clone());
        let fund_client = FundClient::new(&env, &fund_address);
        assert!(
            fund_client.is_whitelisted(&buyer),
            "buyer is not whitelisted on the fund"
        );

        let listing_key = DataKey::Listing(listing_id);
        let mut listing: Listing = env
            .storage()
            .persistent()
            .get(&listing_key)
            .expect("listing not found");

        assert!(
            matches!(listing.status, ListingStatus::Active),
            "listing is not active"
        );

        // Transfer USDC from buyer to seller
        let usdc_address = Self::get_usdc_token(env.clone());
        let usdc_client = token::Client::new(&env, &usdc_address);
        usdc_client.transfer(&buyer, &listing.seller, &listing.price_usdc);

        // Transfer LP tokens from escrow to buyer
        let lp_token_address = Self::get_lp_token(env.clone());
        let lp_token_client = token::Client::new(&env, &lp_token_address);
        lp_token_client.transfer(&env.current_contract_address(), &buyer, &listing.lp_amount);

        listing.status = ListingStatus::Bought;
        env.storage().persistent().set(&listing_key, &listing);
    }

    pub fn cancel_listing(env: Env, seller: Address, listing_id: u64) {
        seller.require_auth();

        let listing_key = DataKey::Listing(listing_id);
        let mut listing: Listing = env
            .storage()
            .persistent()
            .get(&listing_key)
            .expect("listing not found");

        assert_eq!(seller, listing.seller, "only the seller can cancel the listing");
        assert!(
            matches!(listing.status, ListingStatus::Active),
            "listing is not active"
        );

        // Return LP tokens from escrow to seller
        let lp_token_address = Self::get_lp_token(env.clone());
        let lp_token_client = token::Client::new(&env, &lp_token_address);
        lp_token_client.transfer(&env.current_contract_address(), &seller, &listing.lp_amount);

        listing.status = ListingStatus::Cancelled;
        env.storage().persistent().set(&listing_key, &listing);
    }

    // --- Getters ---

    pub fn get_fund_contract(env: Env) -> Address {
        env.storage().instance().get(&DataKey::FundContract).unwrap()
    }

    pub fn get_usdc_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::UsdcToken).unwrap()
    }

    pub fn get_lp_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::LpToken).unwrap()
    }

    pub fn get_listing_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::ListingCount).unwrap_or(0)
    }

    pub fn get_listing(env: Env, listing_id: u64) -> Option<Listing> {
        env.storage().persistent().get(&DataKey::Listing(listing_id))
    }
}

#[cfg(test)]
mod test;

