# Spec: Build the Frontend for Subscript

This document outlines the specifications for the SubScript frontend application, which will provide user interfaces for both Subscribers and Merchants. The frontend will be built using Next.js and integrate with the Charms SDK.

## 1. Frontend Architecture

The application will feature two distinct views:

### 1.1 The "Subscriber" View (User)

This view allows users to manage their subscriptions.

*   **One-Click Subscribe:**
    *   Integrate directly with a user's Bitcoin wallet to facilitate a seamless "one-click" subscription experience.
    *   Clicking "Subscribe" on a merchant's page will prompt a single transaction confirmation from the user's wallet.
    *   This transaction will deposit initial funds and create the Subscript UTXO.
*   **My Subs Page:**
    *   Queries the blockchain for UTXOs where the user's public key is present in the `subscriber` field.
    *   Displays a list of active subscriptions.
*   **Visuals:**
    *   Implement a "Fuel Tank" visual for each subscription, indicating the remaining subscription time or balance.
*   **Action:**
    *   A prominent "Cancel" button to trigger Path B (Unsubscribe) of the Charm protocol.

### 1.2 The "Merchant" View (SaaS Owner)

This view allows SaaS providers to manage their subscriptions and revenue.

*   **Revenue Dashboard:**
    *   Scans the blockchain for all UTXOs where the merchant's public key matches the `merchant` field.
    *   Displays aggregated revenue data and individual subscription statuses.
*   **"Collect" Button:**
    *   This button will activate when `current_block > last_payment_block + interval` for a given subscription.
    *   Triggers Path A (Merchant Pull) of the Charm protocol.
*   **Batching:**
    *   Implement functionality to construct a single transaction that "harvests" payments from multiple subscribers simultaneously (if technically feasible with Charms SDK).

## 2. Technology Stack

*   **Framework:** Next.js 16.1.1
*   **SDK:** Charms SDK
*   **Package Manager:** Bun
*   **Styling:** Adhere to "Bold & Futuristic" visual identity as defined in `product-guidelines.md`.
