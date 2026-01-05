# Technology Stack

This document outlines the core technologies and tools used in the SubScript project.

## 1. Programming Languages

*   **Rust:** Used for implementing the core logic of the "Charm" and its zero-knowledge constraints, interacting with BitcoinOS/Charms at a low level.
*   **JavaScript/TypeScript:** The primary language for the frontend development, particularly with Next.js. TypeScript will be used for type safety and improved developer experience.
*   **Node.js/Python:** Used for the "Gatekeeper" off-chain verification backend, providing flexibility for API development.

## 2. Frameworks & Libraries

*   **Next.js 16.1.1:** The React framework for building the frontend user interface, supporting both the Subscriber and Merchant views.
*   **Charms SDK:** Essential for interacting with the BitcoinOS/Charms protocol, enabling the creation, spending, and querying of Charms (UTXOs).

## 3. Development Tools

*   **Bun:** The chosen JavaScript runtime and package manager, favored for its speed and efficiency in development workflows.
*   **Turbo (Turborepo):** Used as the build system for managing a monorepo structure, optimizing build times, and streamlining development across different project components.