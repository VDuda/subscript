# SubScript

**Bitcoin's Native Protocol for SaaS.**

SubScript is a project that aims to implement "On-Chain SaaS Subscriptions on Bitcoin." It provides a "Direct Debit" protocol for Bitcoin, allowing for recurring payments without the need for traditional financial intermediaries.

This repository contains the implementation of the SubScript protocol, including the on-chain "Charm" (the core logic) and the frontend application for interacting with the protocol.

## Future Roadmap
*   **Dynamic Pricing:** Allow the merchant to update the price (requires user signature).
*   **Top-Ups:** Allow users to add more BTC to an existing subscription.
*   **Fiat-Pegging:** Use a Price Oracle to peg the pull amount to a fiat value.
