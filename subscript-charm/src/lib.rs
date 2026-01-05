use charms_sdk::data::{App, Data, Transaction, B32};
use serde::{Deserialize, Serialize};

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
    // TODO: logic to check for the two spending paths
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dummy() {}
}