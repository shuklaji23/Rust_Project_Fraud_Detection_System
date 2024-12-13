pub mod flag_criteria;
pub mod flagged_transactions;
pub mod functions;
pub mod threadpool;
pub mod transactions;
pub mod console;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use console::start_console;
use functions::FLAGGED_TRANSACTIONS_LIST;
use transactions::Transaction;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    if let Ok(transactions_record) = read_lines("./src/transactions.txt") {
        for line in transactions_record.flatten() {
            let transaction_fields: Vec<&str> = line.split(", ").collect();
            if let [id, account_id, amount, timestamp, origin_country, ip_address, destination_country] =
                &transaction_fields[..]
            {
                let (
                    id,
                    account_id,
                    amount,
                    timestamp,
                    origin_country,
                    ip_address,
                    destination_country,
                ) = (
                    id.parse::<u64>().unwrap_or_default(),
                    account_id.parse::<u32>().unwrap_or_default(),
                    amount.parse::<f64>().unwrap_or_default(),
                    timestamp.to_string(),
                    origin_country.to_string(),
                    ip_address.to_string(),
                    destination_country.to_string(),
                );
                let transaction = Transaction::new(id, account_id, amount, timestamp, origin_country, ip_address, destination_country);
            }
        }
        
        let fl = FLAGGED_TRANSACTIONS_LIST.lock().unwrap();
        println!("{:?}", fl);
    }
    start_console();
}
