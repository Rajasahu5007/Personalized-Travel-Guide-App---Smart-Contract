#![no_std]
#![allow(non_snake_case)]

use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct TravelPlan {
    pub destination: String,
    pub preferences: String,
    pub created_at: u64,
}

#[contracttype]
pub enum TravelKey {
    Plan(u64),
}

const PLAN_COUNT: Symbol = symbol_short!("PLAN_CT");

#[contract]
pub struct TravelGuideContract;

#[contractimpl]
impl TravelGuideContract {
    pub fn create_plan(env: Env, destination: String, preferences: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&PLAN_COUNT).unwrap_or(0);
        count += 1;

        let time = env.ledger().timestamp();

        let plan = TravelPlan {
            destination,
            preferences,
            created_at: time,
        };

        env.storage().instance().set(&TravelKey::Plan(count), &plan);
        env.storage().instance().set(&PLAN_COUNT, &count);

        log!(&env, "Travel Plan created with ID: {}", count);
        count
    }

    pub fn view_plan(env: Env, id: u64) -> TravelPlan {
        env.storage().instance().get(&TravelKey::Plan(id)).unwrap_or(TravelPlan {
            destination: String::from_str(&env, "Not Found"),
            preferences: String::from_str(&env, "Not Found"),
            created_at: 0,
        })
    }

    pub fn total_plans(env: Env) -> u64 {
        env.storage().instance().get(&PLAN_COUNT).unwrap_or(0)
    }
}