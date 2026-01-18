# Bounty Flow

## About
**Bounty Flow** is a decentralised platform built on Soroban(Stellar smart contracts). It removes the middleman and enables a direct connection between freelancers and clients using payment-based tasks.


## Project Structure

├── contracts                        # Directory containing all smart contracts
│   └── bounty_flow                  # Main contract directory
│       ├── src
│       │   ├── lib.rs               # Contract implementation with all business logic
│       │   └── test.rs              # Unit tests for contract functions
│       |── Cargo.toml               # Contract-specific dependencies and metadata
|       └── Makefile                 # Build and test commands
├── .gitignore                       
├── Cargo.toml                       # Workspace-level dependencies shared across contracts
├── Cargo.lock                       # Locked dependency versions for reproducible builds
└── README.md                        


## What the contract does
The contract works in four step:
- *create_task*: Client creates a task
- *get_task*: Freelancer gets a task
- *submit_work*: Freelancer submit the completed task
- *release_funds*: Client releases the payment to freelancer


## Why was this design chosen
- *Keeping it simple*: The concept is easy to understand with the code written in beginner-friendly manner (am a beginner too!)
- *Naming*: All the variables and function has been named so that beginners easily understands what each variables stores and what each functions are supposed to do
- *Error Handling*: For every possible error, it has been handled with proper error name and message.
- *Security by Design*: Every function includes authentication checks 'require_auth()' and validates state transitions before allowing changes.
- *No hidden charges*: There is no middleman thus no reduction of funds as platform fees and other hidden charges.


## How state changes work
┌─────────────┐    submit_work()    ┌─────────────┐   release_funds()   ┌─────────────┐
│   PENDING   │ ──────────────────> │  COMPLETED  │ ──────────────────> │     PAID    │
│  (created)  │                     │ (work done) │                     │  (finished) │
└─────────────┘                     └─────────────┘                     └─────────────┘

**State Transitions**
- *Pending*: Task is created, waiting for a freelancer to claim and complete it
- *Completed*: Freelancer calls submit_work() and their address is recorded, client and reviews the task
- *Paid*: Client releases the payment


## What security checks are implemented
1. **Error Handling**
- TaskNotFound: Task doesn't exist
- AlreadyCompleted: It makes sure that the work is not submitted twice
- NotCompleted: Makes sure funds are not released before work submission
- AlreadyPaid: It prevents double payment for the same task
- Unauthorized: Creator verification failed
- InvalidAmount: It confirms that the payment amount is positive

2. **Access Control**
- `require_auth()` on every state-changing function
- Verifies caller identity before allowing actions
- Prevents impersonation attacks

3. **Authorization Validation**
Only the task creator can release funds
```rust
if task_creator != creator {
    return Err(ContractError::Unauthorized);
}
```

4. **Immutable Records**
- Persistent storage ensures task history can't be altered
- All state changes are recorded on-chain


## Deployed link
https://lab.stellar.org/r/testnet/contract/CAKLCDDE7HC2RPW5R6ZZTTJCLXYJ5VLSYCICFCXM7QBGRI3KXPND2ZIZ