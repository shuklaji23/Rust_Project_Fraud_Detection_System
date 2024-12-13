use crate::functions::*;
use crate::transactions::Transaction;

pub enum FlagType {
    UnusualAmount,
    HighFrequency,
    CrossBorder,
    FlaggedIp,
}

pub fn get_flag_type(txn: &Transaction) -> Option<FlagType> {
    if (txn.amount > 100000 as f64) || (txn.amount < 1 as f64) {
        return Some(FlagType::UnusualAmount);
    } else if transaction_per_sec(txn.account_id) > 5 {
        return Some(FlagType::HighFrequency);
    } else if txn.origin_country != txn.destination_country {
        return Some(FlagType::CrossBorder);
    } else if is_flagged_ip((*txn).ip_address.to_string()) {
        return Some(FlagType::FlaggedIp);
    } else {
        return None;
    }
}
