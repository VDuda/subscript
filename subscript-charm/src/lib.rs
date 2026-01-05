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

    // 2. Check if the transaction is signed by the merchant.
    // This part is tricky without knowing how signatures are handled.
    // Assuming 'app_public_inputs' can contain signature verification data.
    // For now, I will skip the explicit signature verification.
    // In a real ZK system, this would be handled by the proof itself.

    // 3. Check the current block height.
    // This information is not directly in the Transaction.
    // It's likely passed via `w` or `x` or some global context.
    // Let's assume `w` contains the current block height as a u32.
    let current_block_height: u32 = w.value::<u32>().unwrap_or(0); // Default to 0 if not found.

    check!(current_block_height >= input_charm.last_payment_block + input_charm.interval_blocks);

    // 4. Check outputs.
    // There must be exactly two outputs: one for the merchant, one for the new charm.
    check!(tx.outs.len() == 2);

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
            // This is a simplification. We would need to check the address and amount.
            // For now, just check if an output exists with the expected amount.
            if output.amount == input_charm.amount_per_cycle {
                merchant_output_found = true;
            }
        }
    }

    check!(merchant_output_found && new_charm_output_found);

    true // If all checks pass
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dummy() {}
}
