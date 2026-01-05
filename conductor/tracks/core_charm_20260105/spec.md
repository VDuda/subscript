# Spec: Core Subscription Charm (Rust)

This document outlines the technical specifications for the initial SubScript Charm, which forms the core of the on-chain subscription logic.

## 1. Charm State

The `SubscriptionCharm` will be a Rust struct containing the following fields, which define the rules of the subscription UTXO:

```rust
struct SubscriptionCharm {
    subscriber: PubKey,       // The public key of the user who owns the funds.
    merchant: PubKey,         // The public key of the SaaS provider who can pull funds.
    amount_per_cycle: u64,    // The amount in Satoshis to be transferred per payment cycle.
    interval_blocks: u32,     // The number of Bitcoin blocks that constitute one payment cycle.
    last_payment_block: u32,  // The block height of the most recent payment.
}
```

## 2. Spending Paths (Verification Logic)

The Charm's `verify` function must enforce two exclusive spending paths, implemented as ZK constraints.

### Path A: Merchant "Pull"

This path allows the merchant to withdraw funds for a single payment cycle.

**Conditions:**
1.  The transaction must be signed by the `merchant`'s private key.
2.  The current block height must be greater than or equal to `last_payment_block + interval_blocks`.

**Required Outputs:**
1.  An output sending `amount_per_cycle` Satoshis to the `merchant`'s wallet address.
2.  A new `SubscriptionCharm` UTXO containing the remaining funds. This new Charm must have its `last_payment_block` field updated to the current block height.

### Path B: Subscriber "Unsubscribe"

This path allows the subscriber to terminate the subscription and reclaim all remaining funds.

**Conditions:**
1.  The transaction must be signed by the `subscriber`'s private key.

**Required Outputs:**
1.  A single output sending the entire remaining balance of the UTXO back to the `subscriber`'s wallet address. This action consumes and effectively destroys the Charm.
