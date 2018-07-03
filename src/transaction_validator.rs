use std::time::{SystemTime, UNIX_EPOCH};
use txhash::TxHash;
use transaction::Transaction;
use transaction::TRINARY_SIZE;
use transaction::SUPPLY;
use sponge::curl::SpongeMode;

const MAX_TIMESTAMP_FUTURE_SEC :u64 = 2*60*60;
const MAX_TIMESTAMP_FUTURE_MS  :u64 = MAX_TIMESTAMP_FUTURE_SEC*1000;
const ERR_INVALID_TRANSACTION_TRITS : &'static str = "Invalid transaction trits";
const ERR_INVALID_TIMESTAMP : &'static str = "Invalid timestamp";
const ERR_INVALID_VALUE : &'static str = "Invalid value";
const ERR_INVALID_TRANSACTION_HASH : &'static str = "Invalid transaction hash";
const ERR_INVALID_ADDRESS : &'static str = "Invalid address";

struct TransactionValidator {
    snapshot_ts: u64,
    mwm: i32,
}

impl TransactionValidator {

    pub fn has_invalid_timestamp(&self, ts: u64, attachment_ts: u64,  h: TxHash) -> bool {
        attachment_ts == 0 && (ts < self.snapshot_ts && !h.is_null_hash() || ts > now_in_ms() + MAX_TIMESTAMP_FUTURE_MS) ||
        attachment_ts != 0 && (attachment_ts < self.snapshot_ts || (attachment_ts > (now_in_ms() + MAX_TIMESTAMP_FUTURE_MS)))
    }

    pub fn run_validation(&self, tx : Transaction) ->Result<(),&'static str> {
        let h= match TxHash::compute_from_bytes(tx.bytes(), TRINARY_SIZE, SpongeMode::CurlP81)  {
            Err(()) => return Err(ERR_INVALID_TRANSACTION_TRITS),
            Ok(x) => x,
        };

        println!("ts={} attch={}",tx.timestamp(),tx.attachment_timestamp());
        if self.has_invalid_timestamp(tx.timestamp(),tx.attachment_timestamp(),h) {
            return Err(ERR_INVALID_TIMESTAMP);
        }
        if !tx.is_value_valid() {
            return Err(ERR_INVALID_VALUE);
        }
        if h.trailing_zeros() < self.mwm {
            return Err(ERR_INVALID_TRANSACTION_HASH);
        }
        if !tx.last_address_trit_is_zero() && tx.value()>0 {
            return Err(ERR_INVALID_ADDRESS);
        }
        Ok(())
    }
}

fn now_in_sec() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

fn now_in_ms() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_invalid_timestamp_test() {
        let transaction_validator = TransactionValidator{snapshot_ts: now_in_ms()-1000, mwm: 15};
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        assert_eq!(transaction_validator.has_invalid_timestamp(now_in_ms()-2000,0,h1),true);
        assert_eq!(transaction_validator.has_invalid_timestamp(now_in_ms()-500,0,h1),false);
        assert_eq!(transaction_validator.has_invalid_timestamp(now_in_ms()+500,0,h1),false);
        assert_eq!(transaction_validator.has_invalid_timestamp(now_in_ms()+500+MAX_TIMESTAMP_FUTURE_MS,0,h1),true);
    }

    #[test]
    fn run_validation_with_invalid_timestamp(){
        let transaction_validator = TransactionValidator{snapshot_ts: now_in_ms()-1000, mwm: 15};
        let tx = Transaction::new(None,
                                  Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()),
                                  None,
                                  None,
                                  Some(now_in_ms()+5000+MAX_TIMESTAMP_FUTURE_MS), None, None,
                                  Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        match transaction_validator.run_validation(tx){
            Ok(()) => panic!("Expecting invalid"),
            Err(ERR_INVALID_TIMESTAMP) => (),
            Err(x) => panic!("Expecting invalid timestamp but got {}",x),
        }
    }

    #[test]
    fn run_validation_with_invalid_value(){
        let transaction_validator = TransactionValidator{snapshot_ts: now_in_ms()-1000, mwm: 15};
        let tx = Transaction::new(None,
                                  Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()),
                                  Some(SUPPLY+1),
                                  None,
                                  Some(now_in_ms()+500), None, None,
                                  Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        match transaction_validator.run_validation(tx){
            Ok(()) => panic!("Expecting invalid"),
            Err(ERR_INVALID_VALUE) => (),
            Err(x) => panic!("Expecting invalid value but got {}",x),
        }
    }

    #[test]
    fn run_validation_with_invalid_hash(){
        let transaction_validator = TransactionValidator{snapshot_ts: 1482522289-1000, mwm: 15};
        let tx = Transaction::new(None,
                                  Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()),
                                  Some(1000),
                                  None,
                                  Some(1482522289+500), None, None,
                                  Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        match transaction_validator.run_validation(tx){
            Ok(()) => panic!("Expecting invalid"),
            Err(ERR_INVALID_TRANSACTION_HASH) => (),
            Err(x) => panic!("Expecting invalid hash but got {}",x),
        }
    }

    #[test]
    fn run_validation_with_valid_hash(){
        let transaction_validator = TransactionValidator{snapshot_ts: 1482522286-1000, mwm: 1};
        let tx = Transaction::new(None,
                                  Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGD9".as_ref()),
                                  Some(1000),
                                  None,
                                  Some(1482522284+500), None, None,
                                  Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        match transaction_validator.run_validation(tx){
            Ok(()) => (),
            Err(x) => panic!("Expecting valid tx but got {}",x),
        }
    }

    #[test]
    fn run_validation_with_invalid_address(){
        let transaction_validator = TransactionValidator{snapshot_ts: 1482522286-1000, mwm: 2};
        let tx = Transaction::new(None,
                                  Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDM".as_ref()),
                                  Some(1000),
                                  None,
                                  Some(1482522284+500), None, None,
                                  Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        match transaction_validator.run_validation(tx){
            Ok(()) => panic!("Expecting invalid"),
            Err(ERR_INVALID_ADDRESS) => (),
            Err(x) => panic!("Expecting invalid hash but got {}",x),
        }
    }
}