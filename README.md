# Bounty Flow

## About
**Bounty Flow** is a decentralised platform built on Soroban(Stellar smart contracts). It removes the middleman and enables a direct connection between freelancers and clients using payment-based tasks.


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
- AlreadyCompleted: Work is already submitted
- NotCompleted: Funds released before work submission
- AlreadyPaid: Prevent double payment
- Unauthorized: Creator verification failed
- InvalidAmount: Amount must be positive






## Deployed link