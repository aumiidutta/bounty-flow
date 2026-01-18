#![cfg(test)]

use super::*;
use soroban_sdk::{Address, Env, String, testutils::Address as _};

#[test]
fn test_create_task() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(BountyContract, ());
    let client = BountyContractClient::new(&env, &contract_id);

    // Create a task creator address
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Build a website");
    let amount = 1000i128;

    // Create a new task
    let task_id = client.create_task(&creator, &title, &amount);
    
    // Verify task was created with ID
    assert_eq!(task_id, 1u32);
}

#[test]
fn test_submit_work() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(BountyContract, ());
    let client = BountyContractClient::new(&env, &contract_id);

    // Create addresses
    let creator = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let title = String::from_str(&env, "Design a logo");
    let amount = 500i128;

    // Create a task
    let task_id = client.create_task(&creator, &title, &amount);

    // Freelancer submits work
    let submitted = client.submit_work(&task_id, &freelancer);
    
    // Verify work was submitted
    assert_eq!(submitted, true);
}

#[test]
fn test_release_funds() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(BountyContract, ());
    let client = BountyContractClient::new(&env, &contract_id);

    // Create addresses
    let creator = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let title = String::from_str(&env, "Write documentation");
    let amount = 750i128;

    // Create task and submit work
    let task_id = client.create_task(&creator, &title, &amount);
    client.submit_work(&task_id, &freelancer);

    // Creator releases funds
    let paid = client.release_funds(&task_id, &creator);
    
    // Verify funds were released
    assert_eq!(paid, true);
}
