# Plan: Build the Core Subscription Charm (Rust)

This plan details the phases and tasks required to implement the core `SubscriptionCharm` in Rust.

---

## Phase 1: Environment Setup & Charm Definition

### Tasks
- [x] Task: Set up the Rust development environment for BitcoinOS and Charms development. (08abd36)
- [x] Task: Initialize a new Rust project for the SubScript Charm. (08abd36)
- [x] Task: Define the `SubscriptionCharm` struct in the Rust project according to the spec. (08abd36)

---

## Phase 2: Implementation of Spending Paths

### Tasks
- [x] Task: Write unit tests for the "Merchant Pull" spending path. (77547fb)
    - [ ] Sub-task: Test successful pull after the interval has passed.
    - [ ] Sub-task: Test failed pull before the interval has passed.
    - [ ] Sub-task: Test for correct output generation.
- [x] Task: Implement the "Merchant Pull" logic within the Charm's `verify` function. (14db625)
- [x] Task: Write unit tests for the "Subscriber Unsubscribe" spending path. (1389b6e)
    - [ ] Sub-task: Test for successful withdrawal of all funds.
    - [ ] Sub-task: Test that only the subscriber can trigger this path.
- [ ] Task: Implement the "Subscriber Unsubscribe" logic within the Charm's `verify` function.
- [ ] Task: Conductor - User Manual Verification 'Implementation of Spending Paths' (Protocol in workflow.md)

---

## Phase 3: Integration and Deployment

### Tasks
- [ ] Task: Write integration tests to simulate the full lifecycle of a subscription.
- [ ] Task: Compile the Charm to its final on-chain representation.
- [ ] Task: Deploy the Charm to a testnet environment for initial validation.
- [ ] Task: Conductor - User Manual Verification 'Integration and Deployment' (Protocol in workflow.md)
