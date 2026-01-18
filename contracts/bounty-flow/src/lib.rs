#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, Address, Env, String, Symbol, Vec};


#[contract]
pub struct BountyContract;


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    TaskNotFound = 1,      // Task ID doesn't exist
    AlreadyCompleted = 2,  // Work already submitted for this task
    NotCompleted = 3,      // Trying to claim before work is done
    AlreadyPaid = 4,       // Trying to claim twice for the same task
    Unauthorized = 5,      // Caller is not authorized for this action
    InvalidAmount = 6,     // Amount must be greater than zero
}


#[derive(Clone)]
pub struct Task {
    pub id: u32,              // Unique task identifier
    pub creator: Address,     // Person who posted the task
    pub freelancer: Address,  // Person who completes the task
    pub title: String,        // Description of what needs to be done
    pub amount: i128,         // Payment amount locked for this task
    pub is_completed: bool,   // Has the freelancer submitted work?
    pub is_paid: bool,        // Has the creator released payment?
}


#[contractimpl]
impl BountyContract {
    // Creates a new bounty task - This function allows someone to post a task they need done.
    // Returns: The unique task ID, or an error
    pub fn create_task(env: Env, creator: Address, title: String, amount: i128) -> Result<u32, ContractError> {
        creator.require_auth();

        // Validation: Amount must be positive
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let storage = env.storage().persistent();
        
        // Get the current task count (starts at 0 if none exist)
        let task_count_key = Symbol::new(&env, "task_count");
        let mut task_count: u32 = storage.get(&task_count_key).unwrap_or(0);
        
        // Increment to get the new task ID
        task_count += 1;
        let new_task_id = task_count;
        
        let task = (new_task_id, creator.clone(), creator.clone(), title, amount, false, false);
        
        let key = Symbol::new(&env, "task");
        storage.set(&(key, new_task_id), &task);
        
        storage.set(&task_count_key, &task_count);
        
        Ok(new_task_id)
    }

    // Submits completed work for a task - A freelancer calls this when they've finished the work.
    // Returns: true if successful, or an error
    pub fn submit_work(env: Env, task_id: u32, freelancer: Address) -> Result<bool, ContractError> {
        // Security: Verify the caller is who they claim to be
        freelancer.require_auth();
        
        let storage = env.storage().persistent();
        
        let task_data: Option<(u32, Address, Address, String, i128, bool, bool)> = 
            storage.get(&(Symbol::new(&env, "task"), task_id));
        
        match task_data {
            Some((id, creator, _old_freelancer, title, amount, is_completed, is_paid)) => {
                // Error Handling: Can't submit work twice
                if is_completed {
                    return Err(ContractError::AlreadyCompleted);
                }
                
                let updated_task = (id, creator, freelancer, title, amount, true, is_paid);
                storage.set(&(Symbol::new(&env, "task"), task_id), &updated_task);
                Ok(true)
            }
            // Error Handling: Task doesn't exist
            None => Err(ContractError::TaskNotFound),
        }
    }

    // Releases payment to the freelancer - The task creator calls this after reviewing the submitted work.
    // Returns: true if successful, or an error
    pub fn release_funds(env: Env, task_id: u32, creator: Address) -> Result<bool, ContractError> {
        // Security: Verify the caller is who they claim to be
        creator.require_auth();

        let storage = env.storage().persistent();
        
        let task_data: Option<(u32, Address, Address, String, i128, bool, bool)> = 
            storage.get(&(Symbol::new(&env, "task"), task_id));
        
        match task_data {
            Some((id, task_creator, freelancer, title, amount, is_completed, is_paid)) => {
                // Security: Only the task creator can release funds
                if task_creator != creator {
                    return Err(ContractError::Unauthorized);
                }
                
                // Error Handling: Work must be submitted first
                if !is_completed {
                    return Err(ContractError::NotCompleted);
                }
                
                // Error Handling: Prevent double payment
                if is_paid {
                    return Err(ContractError::AlreadyPaid);
                }
                
                // Update the task: mark as paid
                let updated_task = (id, task_creator, freelancer, title, amount, is_completed, true);
                storage.set(&(Symbol::new(&env, "task"), task_id), &updated_task);
                Ok(true)
            }
            // Error Handling: Task doesn't exist
            None => Err(ContractError::TaskNotFound),
        }
    }

    // Retrieves task information - Returns basic information about a task including its title and status.
    // Returns: A vector containing [title, status] or empty if not found
    pub fn get_task(env: Env, task_id: u32) -> Vec<String> {
        let storage = env.storage().persistent();
        
        // Try to retrieve the task
        let key = Symbol::new(&env, "task");
        let task_data: Option<(u32, Address, Address, String, i128, bool, bool)> = 
            storage.get(&(key, task_id));
        
        if let Some((_id, _creator, _freelancer, title, _amount, _is_completed, is_paid)) = task_data {
            // Determine the status based on task state
            let status = if is_paid {
                "paid"          // Task completed and paid
            } else if _is_completed {
                "completed"     // Work submitted, awaiting payment
            } else {
                "pending"       // Task created, waiting for freelancer
            };
            
            // Build result vector with title and status
            let mut result = Vec::new(&env);
            result.push_back(title);
            result.push_back(String::from_str(&env, status));
            
            result
        } else {
            // Task not found, return empty vector
            Vec::new(&env)
        }
    }
}

mod test;
