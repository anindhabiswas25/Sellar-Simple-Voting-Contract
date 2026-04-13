#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    YesVotes,
    NoVotes,
    HasVoted(Address),
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn vote(env: Env, voter: Address, support: bool) {
        voter.require_auth();

        if env.storage().instance().has(&DataKey::HasVoted(voter.clone())) {
            panic!("already voted");
        }

        if support {
            let yes: u32 = env.storage().instance().get(&DataKey::YesVotes).unwrap_or(0);
            env.storage().instance().set(&DataKey::YesVotes, &(yes + 1));
        } else {
            let no: u32 = env.storage().instance().get(&DataKey::NoVotes).unwrap_or(0);
            env.storage().instance().set(&DataKey::NoVotes, &(no + 1));
        }

        env.storage().instance().set(&DataKey::HasVoted(voter), &true);
    }

    pub fn result(env: Env) -> (u32, u32) {
        let yes: u32 = env.storage().instance().get(&DataKey::YesVotes).unwrap_or(0);
        let no: u32 = env.storage().instance().get(&DataKey::NoVotes).unwrap_or(0);
        (yes, no)
    }
}