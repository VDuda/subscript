"use client";

import { useState } from "react";
import { Button } from "@repo/ui/button";

export const SubscribeForm = () => {
  const [merchantAddress, setMerchantAddress] = useState("");
  const [depositAmount, setDepositAmount] = useState("");

  const handleSubscribe = async () => {
    // TODO: This is a placeholder. Replace with actual charms-sdk and wallet integration.

    // 1. Get user's wallet and address (mocked for now)
    const userWallet = {
      address: "tb1q...", // mock user address
      // function to sign a transaction
      signTransaction: async (tx: any) => {
        console.log("Signing transaction:", tx);
        return "signed_transaction_hex";
      },
    };

    // 2. Construct the transaction using charms-sdk (mocked for now)
    // This would involve creating a spell or using the sdk's transaction builder.
    console.log("Constructing transaction with charms-sdk...");
    const transaction = {
      inputs: [
        // User's UTXOs to fund the subscription
      ],
      outputs: [
        {
          // The new SubscriptionCharm UTXO
          address: "charm_address", // The address of the charm
          charms: {
            // The SubscriptionCharm data
          },
        },
        {
          // Change back to the user
          address: userWallet.address,
        },
      ],
    };

    // 3. Prompt user to sign the transaction
    try {
      const signedTx = await userWallet.signTransaction(transaction);
      console.log("Transaction signed:", signedTx);

      // 4. Broadcast the transaction (mocked for now)
      console.log("Broadcasting transaction...");
      alert(`Subscription to ${merchantAddress} with ${depositAmount} BTC created successfully!`);
    } catch (error) {
      console.error("Subscription failed:", error);
      alert("Subscription failed. See console for details.");
    }
  };

  return (
    <div className="flex flex-col gap-4">
      <h2 className="text-2xl font-bold">New Subscription</h2>
      <div className="flex flex-col gap-2">
        <label htmlFor="merchant-address">Merchant Address</label>
        <input
          id="merchant-address"
          type="text"
          value={merchantAddress}
          onChange={(e) => setMerchantAddress(e.target.value)}
          className="rounded-md border border-gray-300 p-2 text-black"
          placeholder="Enter merchant address"
        />
      </div>
      <div className="flex flex-col gap-2">
        <label htmlFor="deposit-amount">Deposit Amount (BTC)</label>
        <input
          id="deposit-amount"
          type="text"
          value={depositAmount}
          onChange={(e) => setDepositAmount(e.target.value)}
          className="rounded-md border border-gray-300 p-2 text-black"
          placeholder="Enter deposit amount"
        />
      </div>
      <Button appName="web" onClick={handleSubscribe}>
        Subscribe
      </Button>
    </div>
  );
};