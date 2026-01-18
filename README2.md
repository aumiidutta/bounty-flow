# Soroban Project

## About

**Bounty Flow** is a decentralized freelancing platform built on Soroban (Stellar smart contracts). It enables direct task-based payments between clients and freelancers without any middleman or platform fees.

### What the Contract Does

The bounty contract facilitates a simple three-step workflow:

1. **Task Creation** - A client posts a task with a description and locked payment amount
2. **Work Submission** - A freelancer completes the task and submits their work
3. **Fund Release** - The client reviews and releases payment directly to the freelancer

This creates a trustless environment where both parties are protected: clients don't pay until work is submitted, and freelancers are guaranteed payment for completed work.

### Why This Design Was Chosen

**Simplicity First**: The design prioritizes clarity and ease of understanding for beginners learning smart contract development. We use:
- Direct state mutations instead of complex state machines
- Tuple storage instead of complex serialization (beginner-friendly)
- Clear function names that describe exactly what they do
- Explicit error types instead of generic failures

**Security by Design**: Every function includes authentication checks (`require_auth()`) and validates state transitions before allowing changes.

**No Middleman**: Unlike traditional platforms (Upwork, Fiverr), funds flow directly from client to freelancer, eliminating platform fees and delays.

### How State Changes Work

The contract follows a linear state machine for each task:


┌─────────────┐    submit_work()    ┌─────────────┐   release_funds()   ┌─────────────┐
│   PENDING   │ ──────────────────> │  COMPLETED  │ ──────────────────> │     PAID    │
│  (created)  │                     │ (work done) │                     │  (finished) │
└─────────────┘                     └─────────────┘                     └─────────────┘


**State Transitions:**

1. **PENDING** (Initial State)
   - Task is created via `create_task()`
   - `is_completed = false`, `is_paid = false`
   - Waiting for a freelancer to claim and complete it

2. **COMPLETED** (Work Submitted)
   - Freelancer calls `submit_work()`
   - `is_completed = true`, `is_paid = false`
   - Freelancer address is recorded
   - Client can now review and release payment

3. **PAID** (Final State)
   - Client calls `release_funds()` after reviewing
   - `is_completed = true`, `is_paid = true`
   - Transaction is complete and immutable

**Key Rules:**
- Tasks can only move forward, never backward
- Each transition requires authentication
- Multiple checks prevent invalid state changes

### Security Checks Implemented

The contract implements multiple security layers:

#### 1. **Access Control**
- `require_auth()` on every state-changing function
- Verifies caller identity before allowing actions
- Prevents impersonation attacks

#### 2. **Authorization Validation**
```rust
// Only task creator can release funds
if task_creator != creator {
    return Err(ContractError::Unauthorized);
}
```

#### 3. **State Validation**
- **No Double Submission**: Prevents submitting work twice
- **No Premature Payment**: Can't pay before work is completed
- **No Double Payment**: Prevents paying the same task twice

#### 4. **Input Validation**
```rust
// Amount must be positive
if amount <= 0 {
    return Err(ContractError::InvalidAmount);
}
```

#### 5. **Error Handling**
- Comprehensive error types (`TaskNotFound`, `AlreadyCompleted`, etc.)
- Explicit errors instead of silent failures
- Clear feedback for debugging and user experience

#### 6. **Immutable Records**
- Persistent storage ensures task history can't be altered
- All state changes are recorded on-chain

### Deployed Contract

**Status**: Not yet deployed

**Deployment Instructions**:
```bash
# Build the contract
cd contracts/bounty-flow
make build

# Deploy to testnet (requires stellar CLI configured)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/bounty_flow.wasm \
  --source YOUR_ACCOUNT \
  --network testnet

# The command will output your contract ID
```

**Once deployed, the contract address will be added here.**

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
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
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `bounty_flow` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
