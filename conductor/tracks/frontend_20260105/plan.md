# Plan: Build the Frontend for Subscript

This plan details the phases and tasks required to implement the SubScript frontend application.

---

## Phase 1: Project Setup and Subscriber View (Minting UI)

### Tasks
- [~] Task: Initialize a new Next.js project using Bun and Turborepo.
- [ ] Task: Set up basic project structure for Subscriber and Merchant views.
- [ ] Task: Implement the "Subscribe Button" component for the Subscriber View.
    - [ ] Sub-task: Create a form to input "Merchant Address" and "Deposit Amount".
    - [ ] Sub-task: Integrate with a Bitcoin wallet to prompt transaction confirmation.
- [ ] Task: Implement logic to create a new Subscript UTXO upon successful subscription.
- [ ] Task: Conductor - User Manual Verification 'Project Setup and Subscriber View' (Protocol in workflow.md)

---

## Phase 2: Subscriber View (My Subs & Cancel)

### Tasks
- [ ] Task: Develop the "My Subs" page to query the blockchain for user subscriptions.
- [ ] Task: Implement the "Fuel Tank" visual for displaying remaining subscription time/balance.
- [ ] Task: Create the "Cancel" button to trigger the Unsubscribe (Path B) logic.
- [ ] Task: Conductor - User Manual Verification 'Subscriber View (My Subs & Cancel)' (Protocol in workflow.md)

---

## Phase 3: Merchant View (Revenue Dashboard & Collect)

### Tasks
- [ ] Task: Build the "Revenue Dashboard" to scan the blockchain for merchant UTXOs.
- [ ] Task: Implement the "Collect" button to activate for eligible subscriptions.
- [ ] Task: Develop logic for the "Collect" button to trigger the Merchant Pull (Path A) logic.
- [ ] Task: (Optional) Implement batching functionality for collecting payments from multiple subscribers.
- [ ] Task: Conductor - User Manual Verification 'Merchant View (Revenue Dashboard & Collect)' (Protocol in workflow.md)

---

## Phase 4: Polish and Gatekeeper Integration

### Tasks
- [ ] Task: Integrate the "Gatekeeper" (off-chain verification) into a demo service.
- [ ] Task: Refine UI/UX for both Subscriber and Merchant views based on "Bold & Futuristic" guidelines.
- [ ] Task: Conduct thorough testing and debugging across all frontend components.
- [ ] Task: Conductor - User Manual Verification 'Polish and Gatekeeper Integration' (Protocol in workflow.md)
