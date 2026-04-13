#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_vote_yes() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.vote(&user, &true);

    let (yes, no) = client.result();
    assert_eq!(yes, 1);
    assert_eq!(no, 0);
}

#[test]
fn test_vote_no() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.vote(&user, &false);

    let (yes, no) = client.result();
    assert_eq!(yes, 0);
    assert_eq!(no, 1);
}

#[test]
#[should_panic(expected = "already voted")]
fn test_double_vote_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.vote(&user, &true);
    client.vote(&user, &false); // This should panic
}

#[test]
fn test_multiple_voters() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);

    client.vote(&user1, &true);
    client.vote(&user2, &true);
    client.vote(&user3, &false);

    let (yes, no) = client.result();
    assert_eq!(yes, 2);
    assert_eq!(no, 1);
}

#[test]
fn test_initial_result_is_zero() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let (yes, no) = client.result();
    assert_eq!(yes, 0);
    assert_eq!(no, 0);
}
