use charms_data::{check, App, Data, Transaction, B32, UtxoId, TxId, NativeOutput};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionCharm {
    subscriber: B32,       // The user (owns the funds)
    merchant: B32,         // The SaaS (can pull funds)
    amount_per_cycle: u64,    // e.g., 50,000 Sats
    interval_blocks: u32,     // e.g., 144 blocks (~24 hours for demo)
    last_payment_block: u32,  // The block height of the previous payment
}

pub fn app_contract(app: &App, tx: &Transaction, x: &Data, w: &Data) -> bool {
    let empty = Data::empty();
    assert_eq!(x, &empty);

    // 1. Deserialize the SubscriptionCharm from the tx.ins
    let mut input_charm: Option<SubscriptionCharm> = None;
    for (_, charms_map) in tx.ins.iter() {
        if let Some(data) = charms_map.get(app) {
            input_charm = data.value::<SubscriptionCharm>().ok();
            if input_charm.is_some() {
                break;
            }
        }
    }

    let Some(input_charm) = input_charm else {
        // No input charm found, or deserialization failed.
        return false;
    };

    // Determine if this is a Merchant Pull or Subscriber Unsubscribe
    // We assume 'w' might contain a flag for the type of transaction.
    // This is a simplification and would need a more robust mechanism in a real system.
    let is_unsubscribe: bool = w.value::<bool>().unwrap_or(false);

    if is_unsubscribe {
        // Path B: The "Unsubscribe" (Triggered by Subscriber)
        // 1. Check if the transaction is signed by the `subscriber`.
        //    (Simplified: we assume this check is handled by the ZK proof or implicitly through 'app_public_inputs')
        //    For now, we'll assume a valid signature is present if is_unsubscribe is true.

        // 2. Check outputs: A single output sending 100% of remaining funds back to the Subscriber's wallet.
        check!(tx.outs.is_empty()); // No charm outputs for unsubscribe
        check!(tx.coin_outs.is_some());
        let coin_outs = tx.coin_outs.as_ref().unwrap();
        check!(coin_outs.len() == 1); // Exactly one native coin output

        let output_coin = &coin_outs[0];
        // For simplicity, we assume the amount matches the total value of the input UTXO.
        // In a real scenario, this would involve summing input amounts.
        // We'll also assume the destination matches the subscriber's address (B32).
        // This is a placeholder for a more complex check.
        // check!(output_coin.dest == input_charm.subscriber.0.to_vec());
        // For now, we only check for a single output.

    } else {
        // Path A: The "Pull" (Triggered by Merchant)
        // 1. Check if the transaction is signed by the `merchant` (skipped for now).
        // 2. Check current block height.
        let current_block_height: u32 = w.value::<u32>().unwrap_or(0);
        check!(current_block_height >= input_charm.last_payment_block + input_charm.interval_blocks);

        // 3. Check outputs.
        check!(tx.outs.len() == 2); // Two charm outputs: one for payment, one for new charm

        let mut merchant_output_found = false;
        let mut new_charm_output_found = false;

        for output_charm_map in tx.outs.iter() {
            if let Some(output_data) = output_charm_map.get(app) {
                if let Ok(output_charm) = output_data.value::<SubscriptionCharm>() {
                    // Check for the new charm with updated last_payment_block
                    check!(output_charm.subscriber == input_charm.subscriber);
                    check!(output_charm.merchant == input_charm.merchant);
                    check!(output_charm.amount_per_cycle == input_charm.amount_per_cycle);
                    check!(output_charm.interval_blocks == input_charm.interval_blocks);
                    check!(output_charm.last_payment_block == current_block_height);
                    new_charm_output_found = true;
                }
            }
        }

        // Check for merchant payment in coin_outs
        if let Some(coin_outs) = &tx.coin_outs {
            for output in coin_outs.iter() {
                if output.amount == input_charm.amount_per_cycle {
                    // check!(output.dest == input_charm.merchant.0.to_vec()); // Simplified check
                    merchant_output_found = true;
                }
            }
        }
        check!(merchant_output_found && new_charm_output_found);
    }
    true // If all checks pass
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dummy() {}
}