use crate::flagged_transactions::FlaggedTransaction;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref FLAGGED_IP: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref UNFLAGGED_TRANSACTIONS: Mutex<i32> = Mutex::new(0);
    pub static ref UNUSUAL_COUNT: Mutex<i32> = Mutex::new(0);
    pub static ref CROSS_BORDER_COUNT: Mutex<i32> = Mutex::new(0);
    pub static ref HIGH_FREQUENCY_COUNT: Mutex<i32> = Mutex::new(0);
    pub static ref FLAGGED_IP_COUNT: Mutex<i32> = Mutex::new(0);
    #[derive(Debug)]
    pub static ref FLAGGED_TRANSACTIONS_LIST: Mutex<Vec<FlaggedTransaction>> =
        Mutex::new(Vec::new());

}

pub fn is_flagged_ip(ip_address: String) -> bool {
    let flagged_ip_list = FLAGGED_IP.lock().unwrap();
    let item = ip_address;
    flagged_ip_list.contains(&item)
}

pub fn add_flagged_ip(ip_address: String) {
    let mut flagged_ip_list = FLAGGED_IP.lock().unwrap();
    flagged_ip_list.push(ip_address);
}

pub fn transaction_per_sec(account_id: u32) -> i32 {
    //to be done
    return 0;
}

pub fn increase_count(counter: &str){
    match counter{
        "UNFLAGGED_COUNTER" => {
            let mut number = UNFLAGGED_TRANSACTIONS.lock().unwrap();
            *number+=1;
        }
        "UNUSUAL_COUNT" => {
            let mut number = UNUSUAL_COUNT.lock().unwrap();
            *number+=1;
        }
        "CROSS_BORDER_COUNT" => {
            let mut number = CROSS_BORDER_COUNT.lock().unwrap();
            *number+=1;
        }
        "HIGH_FREQUENCY_COUNT" => {
            let mut number = HIGH_FREQUENCY_COUNT.lock().unwrap();
            *number+=1;
        }
        "FLAGGED_IP_COUNT" => {
            let mut number = FLAGGED_IP_COUNT.lock().unwrap();
            *number+=1;
        }
        _ => {println!("An error occured!!");}
    }
}

