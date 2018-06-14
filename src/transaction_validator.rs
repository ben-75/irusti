use std::time::{SystemTime, UNIX_EPOCH};
use txhash::TxHash;

const MAX_TIMESTAMP_FUTURE_SEC :u64 = 2*60*60;
const MAX_TIMESTAMP_FUTURE_MS  :u64 = MAX_TIMESTAMP_FUTURE_SEC*1000;

struct TransactionValidator {
    snapshot_ts: u64,
}

impl TransactionValidator {
    pub fn has_invalid_timestamp(&self, attachment_ts: u64, ts: u64, h: TxHash) -> bool {
        attachment_ts == 0 && (ts < self.snapshot_ts && !h.is_null_hash() || ts > now_in_sec() + MAX_TIMESTAMP_FUTURE_SEC) ||
            (attachment_ts < self.snapshot_ts || attachment_ts > now_in_ms() + MAX_TIMESTAMP_FUTURE_MS)
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