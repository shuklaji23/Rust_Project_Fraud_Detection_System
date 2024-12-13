use crate::{flag_criteria::FlagType, functions::FLAGGED_TRANSACTIONS_LIST, transactions::Transaction};

#[derive(Clone, Debug)]
pub struct FlaggedTransaction{
    transaction: Transaction,
    reason: String,
}

impl FlaggedTransaction{
    pub fn new(transaction: &Transaction, flag_type: FlagType)-> FlaggedTransaction{
        let reason: String = match flag_type{
            FlagType::UnusualAmount => String::from("Unusual amount"),
            FlagType::CrossBorder => String::from("Cross Border Transaction"),
            FlagType::FlaggedIp => String::from("IP is flagged"),
            FlagType::HighFrequency => String::from("High Frequency"),
        };

        let flagged_transaction = FlaggedTransaction{
            transaction: transaction.clone(),
            reason,
        };

        let mut flagged_transaction_list = FLAGGED_TRANSACTIONS_LIST.lock().unwrap();
        flagged_transaction_list.push(flagged_transaction.clone());
        return flagged_transaction;
    }
}