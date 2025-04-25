#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Consultation {
    pub id: u64,
    pub consultant_name: String,
    pub client_name: String,
    pub topic: String,
    pub scheduled_at: u64,
}

const COUNT: Symbol = symbol_short!("C_COUNT");

#[contracttype]
pub enum Consultbook {
    Entry(u64),
}

#[contract]
pub struct ConsultingPlatformContract;

#[contractimpl]
impl ConsultingPlatformContract {
    pub fn create_consultation(env: Env, consultant_name: String, client_name: String, topic: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&COUNT).unwrap_or(0);
        count += 1;

        let scheduled_at = env.ledger().timestamp();

        let entry = Consultation {
            id: count,
            consultant_name,
            client_name,
            topic,
            scheduled_at,
        };

        env.storage().instance().set(&Consultbook::Entry(count), &entry);
        env.storage().instance().set(&COUNT, &count);
        count
    }

    pub fn get_consultation(env: Env, id: u64) -> Consultation {
        env.storage().instance().get(&Consultbook::Entry(id)).unwrap_or(Consultation {
            id: 0,
            consultant_name: String::from_str(&env, "Not_Found"),
            client_name: String::from_str(&env, "Not_Found"),
            topic: String::from_str(&env, "Not_Found"),
            scheduled_at: 0,
        })
    }

    pub fn total_consultations(env: Env) -> u64 {
        env.storage().instance().get(&COUNT).unwrap_or(0)
    }
}
