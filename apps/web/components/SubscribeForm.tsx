"use client";

import { useState } from "react";
import { Button } from "@repo/ui/button";

export const SubscribeForm = () => {
  const [merchantAddress, setMerchantAddress] = useState("");
  const [depositAmount, setDepositAmount] = useState("");

  const handleSubscribe = () => {
    // TODO: Integrate with Bitcoin wallet
    alert(`Subscribing to ${merchantAddress} with ${depositAmount} BTC`);
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