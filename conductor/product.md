# Initial Concept
The project aims to implement "On-Chain SaaS Subscriptions on Bitcoin." This project will use Bun, the latest Next.js, and Turbo to create Subscript, a "Direct Debit" protocol for Bitcoin.

# 🚀 Project Name: SubScript
**Tagline:** *Bitcoin's Native Protocol for SaaS.*

## Developer Experience (DevEx)
The developer experience should be a primary focus, aiming for a seamless integration process similar to Stripe.

### 1. The Core Logic (The "Charm")
In the Charms/BitcoinOS model, the core is a **Rule Set** that lives inside a UTXO.

#### **The State (What determines the rules)**
The Charm's specific data structure (State) will look like this in Rust:

```rust
struct SubscriptionCharm {
    subscriber: PubKey,       // The user (owns the funds)
    merchant: PubKey,         // The SaaS (can pull funds)
    amount_per_cycle: u64,    // e.g., 50,000 Sats
    interval_blocks: u32,     // e.g., 144 blocks (~24 hours for demo)
    last_payment_block: u32,  // The block height of the previous payment
}
```

#### **The Spending Paths (The ZK Logic)**
The `verify` function allows the UTXO to be spent in only **two** ways:

**Path A: The "Pull" (Triggered by Merchant)**
*   **Check 1:** Is the transaction signed by the `merchant`?
*   **Check 2:** Is `current_block_height >= last_payment_block + interval_blocks`?
*   **Check 3:** Are the outputs correct?
    *   *Output 0:* Sends `amount_per_cycle` to the Merchant's wallet.
    *   *Output 1:* Creates a **New Charm** with the remainder of the funds, but updates `last_payment_block` to the current height.

**Path B: The "Unsubscribe" (Triggered by Subscriber)**
*   **Check 1:** Is the transaction signed by the `subscriber`?
*   **Check 2:** Are the outputs correct?
    *   *Output 0:* Sends **100% of remaining funds** back to the Subscriber's wallet.
    *   *Result:* The subscription is terminated (UTXO destroyed).

---

### 2. The Architecture (Full Stack)

The project requires a working UI and SDK integration.

#### **A. The Frontend (Next.js + Charms SDK)**
There are two distinct views in the app:

**1. The "Subscriber" View (User)**
*   **One-Click Subscribe:** To achieve a seamless "one-click" feel, the frontend will integrate directly with a user's Bitcoin wallet. Clicking "Subscribe" on a merchant's page will prompt a single transaction confirmation from the user's wallet to deposit the initial funds and create the Subscript UTXO.
*   **My Subs:** Queries the chain for UTXOs containing the user's PubKey in the `subscriber` field.
*   **Visuals:** Show a "Fuel Tank" visual indicating the remaining subscription time.
*   **Action:** A "Cancel" button that triggers Path B.

**2. The "Merchant" View (SaaS Owner)**
*   **Revenue Dashboard:** Scans the chain for all UTXOs where `merchant == MyPubKey`.
*   **"Collect" Button:** This button activates when `current_block > last_payment_block + interval`.
*   **Batching:** The button should construct a transaction to harvest payments from multiple subscribers.

#### **B. The "Gatekeeper" (Off-Chain Verification)**
To grant access to a service, a simple API will be used.

*   **Simple API:** A tiny NodeJS/Python backend.
*   **Endpoint:** `/check-access?user_address=xyz`
*   **Logic:** The API queries the BitcoinOS/Charms indexer.
    *   If a valid SubScript UTXO exists for the user, grant access (200 OK).
    *   If the balance is empty, deny access.

---

### 3. Execution Plan (Hackathon Timeline)

**Day 1: The Charm (Rust)**
*   Set up the Charms environment.
*   Define the `SubscriptionCharm` struct.
*   Write the ZK constraints for the "Pull" logic.
*   **Goal:** Write a test unit simulating a "Pull" before and after the time limit.

**Day 2: The Minting UI**
*   Build the frontend to "Mint" the Charm. This includes the wallet integration for the "One-Click Subscribe" experience.
*   **Goal:** Successfully lock Testnet BTC into a subscription UTXO with a single user confirmation.

**Day 3: The Dashboard & Interaction)**
*   Build the "Merchant Dashboard."
*   Implement the "Collect" function using the Charms SDK.
*   **Goal:** Click "Collect" and see the wallet balance increase.

**Day 4: Polish & Pitch)**
*   Add the "Cancel" button.
*   Create a simple "Demo Newsletter Service" with protected content that only unlocks if the API sees a valid Subscription UTXO.
*   **Narrative:** Focus on how this kills the need for credit cards in Web3.

---

### 4. The "Wow" Factor (How to Demo)

1.  **The Setup:** "Alice wants to subscribe to Bob's Premium Newsletter."
2.  **The Action:** With one click, Alice approves the transaction from her wallet, depositing BTC into the SubScript contract. She then tries to access Bob's premium newsletter content, which is initially blocked.
3.  **The Verification:** The newsletter service's "Gatekeeper" API checks Alice's subscription status. Upon successful verification, Alice gains access to the premium content.
4.  **The Wait:** Show Bob's dashboard with the next payment block.
5.  **The Fast Forward:** Mine blocks rapidly.
6.  **The Collection:** The "Collect" button turns green; click it.
7.  **The Result:** Bob gets paid, and Alice's subscription stays active.
8.  **The Finale:** Alice gets bored. She clicks "Cancel," and the remaining funds return to her wallet instantly. The newsletter content becomes blocked again.

### 5. Future Roadmap
*   **Dynamic Pricing:** Allow the merchant to update the price (requires user signature).
*   **Top-Ups:** Allow users to add more BTC to an existing subscription.
*   **Fiat-Pegging:** Use a Price Oracle to peg the pull amount to a fiat value.

### 6. Resources
*   [BitcoinOS Documentation](https://docs.bitcoinos.build/technical-documentation/grail-pro-charms-zkbtc)
*   [Charms Documentation](https://docs.charms.dev/guides/charms-apps/get-started/)