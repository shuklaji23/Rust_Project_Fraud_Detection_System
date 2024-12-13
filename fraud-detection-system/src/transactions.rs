use crate::functions::*;
use crate::{flag_criteria::get_flag_type, flagged_transactions::FlaggedTransaction};
use chrono::{NaiveDateTime, TimeZone, Utc};

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: u64,
    pub account_id: u32,
    pub amount: f64,
    pub timestamp: u64,
    pub origin_country: String,
    pub ip_address: String,
    pub destination_country: String,
}

impl Transaction {
    pub fn new(
        id: u64,
        account_id: u32,
        amount: f64,
        timestamp: String,
        origin_country: String,
        ip_address: String,
        destination_country: String,
    ) -> Transaction {
        let transaction = Transaction {
            id,
            account_id,
            amount,
            timestamp: Utc
                .from_utc_datetime(
                    &NaiveDateTime::parse_from_str(timestamp.as_str(), "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                )
                .timestamp() as u64,
            origin_country,
            ip_address,
            destination_country,
        };

        let flag = get_flag_type(&transaction);
        if let Some(flagtype) = flag {
            let _flagged_transaction = FlaggedTransaction::new(&transaction, flagtype);
        } else {
            increase_count("UNFLAGGED_COUNTER");
        }
        return transaction;
    }
}