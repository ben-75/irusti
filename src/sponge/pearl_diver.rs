use crossbeam;
use num_cpus;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Running,
    Cancelled,
    Completed,
}

const TRANSACTION_LENGTH: usize = 8019;
const CURL_HASH_LENGTH: usize = 243;
const CURL_STATE_LENGTH: usize = CURL_HASH_LENGTH * 3;
const HIGH_BITS: u64 =
    0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
const LOW_BITS: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

pub struct PearlDiver {
    running: AtomicBool,
}

impl Default for PearlDiver {
    fn default() -> Self {
        PearlDiver {
            running: AtomicBool::new(false),
        }
    }
}

impl PearlDiver {
    pub fn new() -> PearlDiver {
        PearlDiver::default()
    }

    pub fn cancel(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

pub fn search(transaction_trits: [i8; 8019], min_weight_magnitude: usize) -> (bool, Vec<i8>) {
    let state = AtomicBool::new(true);
    validate_parameters(&transaction_trits, min_weight_magnitude);
    let mut mid_state_low = vec![0; CURL_STATE_LENGTH];
    let mut mid_state_high = vec![0; CURL_STATE_LENGTH];
    initialize_mid_curl_states(&transaction_trits, &mut mid_state_low, &mut mid_state_high);
    let state_arc = Arc::new(state);
    let transaction_trits_arc = Arc::new(Mutex::new(transaction_trits));
    crossbeam::scope(|scope| {
        for i in (0..num_cpus::get()).rev() {
            let mut mid_state_copy_low = mid_state_low.to_vec();
            let mut mid_state_copy_high = mid_state_high.to_vec();
            let local_state_arc = Arc::clone(&state_arc);
            let local_transaction_trits_arc = Arc::clone(&transaction_trits_arc);
            scope.spawn(move || {
                get_runnable(
                    &local_state_arc,
                    i,
                    &local_transaction_trits_arc,
                    min_weight_magnitude,
                    &mut mid_state_copy_low,
                    &mut mid_state_copy_high,
                );
            });
        }
    });
    let result = (*(transaction_trits_arc.lock().unwrap())).to_vec();
    (!state_arc.load(Ordering::SeqCst), result)
}

pub fn get_runnable(
    state: &Arc<AtomicBool>,
    thread_index: usize,
    transaction_trits: &Arc<Mutex<[i8; 8019]>>,
    min_weight_magnitude: usize,
    mid_state_copy_low: &mut [u64],
    mid_state_copy_high: &mut [u64],
) {
    for _ in 0..thread_index {
        increment(
            mid_state_copy_low,
            mid_state_copy_high,
            162 + CURL_HASH_LENGTH / 9,
            162 + (CURL_HASH_LENGTH / 9) * 2,
        );
    }
    let mut state_low = [0; CURL_STATE_LENGTH];
    let mut state_high = [0; CURL_STATE_LENGTH];

    let mut scratchpad_low = [0; CURL_STATE_LENGTH];
    let mut scratchpad_high = [0; CURL_STATE_LENGTH];

    let mask_start_index = CURL_HASH_LENGTH - min_weight_magnitude;
    let mut mask = 0;

    while mask == 0 && state.load(Ordering::SeqCst) {
        increment(
            mid_state_copy_low,
            mid_state_copy_high,
            162 + (CURL_HASH_LENGTH / 9) * 2,
            CURL_HASH_LENGTH,
        );
        copy(
            mid_state_copy_low,
            mid_state_copy_high,
            &mut state_low,
            &mut state_high,
        );
        transform(
            &mut state_low,
            &mut state_high,
            &mut scratchpad_low,
            &mut scratchpad_high,
        );

        mask = HIGH_BITS;
        for i in mask_start_index..CURL_HASH_LENGTH {
            if mask == 0 {
                break;
            }
            mask &= !(state_low[i] ^ state_high[i]);
        }
    }

    if mask != 0 && state.load(Ordering::SeqCst) {
        state.store(false, Ordering::SeqCst);
        let mut out_mask = 1;
        while (out_mask & mask) == 0 {
            out_mask <<= 1;
        }
        for i in 0..CURL_HASH_LENGTH {
            let mut locked_transaction_trits = transaction_trits.lock().unwrap();
            locked_transaction_trits[TRANSACTION_LENGTH - CURL_HASH_LENGTH + i] =
                if (mid_state_copy_low[i] & out_mask) == 0 {
                    1
                } else if (mid_state_copy_high[i] & out_mask) == 0 {
                    -1
                } else {
                    0
                };
        }
    }
}

fn validate_parameters(transaction_trits: &[i8], min_weight_magnitude: usize) {
    assert_eq!(transaction_trits.len(), TRANSACTION_LENGTH);
    assert!(min_weight_magnitude <= CURL_HASH_LENGTH);
}

fn copy(src_low: &[u64], src_high: &[u64], dest_low: &mut [u64], dest_high: &mut [u64]) {
    //array_copy(src_low, 0, dest_low, 0, CURL_STATE_LENGTH);
    dest_low[0..CURL_STATE_LENGTH].copy_from_slice(&src_low[0..CURL_STATE_LENGTH]);

    //array_copy(src_high, 0, dest_high, 0, CURL_STATE_LENGTH);
    dest_high[0..CURL_STATE_LENGTH].copy_from_slice(&src_high[0..CURL_STATE_LENGTH]);
}

fn initialize_mid_curl_states(
    transaction_trits: &[i8],
    mid_state_low: &mut [u64],
    mid_state_high: &mut [u64],
) {
    for i in CURL_HASH_LENGTH..CURL_STATE_LENGTH {
        mid_state_low[i] = HIGH_BITS;
        mid_state_high[i] = HIGH_BITS;
    }

    let mut offset = 0;
    let mut curl_scratchpad_low = vec![0; CURL_STATE_LENGTH];
    let mut curl_scratchpad_high = vec![0; CURL_STATE_LENGTH];
    for _ in (0..(TRANSACTION_LENGTH - CURL_HASH_LENGTH) / CURL_HASH_LENGTH).rev() {
        for j in 0..CURL_HASH_LENGTH {
            match transaction_trits[offset] {
                0 => {
                    mid_state_low[j] = HIGH_BITS;
                    mid_state_high[j] = HIGH_BITS;
                }
                1 => {
                    mid_state_low[j] = LOW_BITS;
                    mid_state_high[j] = HIGH_BITS;
                }
                _ => {
                    mid_state_low[j] = HIGH_BITS;
                    mid_state_high[j] = LOW_BITS;
                }
            }
            offset += 1;
        }
        transform(
            mid_state_low,
            mid_state_high,
            &mut curl_scratchpad_low,
            &mut curl_scratchpad_high,
        );
    }
    for i in 0..162 {
        match transaction_trits[offset] {
            0 => {
                mid_state_low[i] = HIGH_BITS;
                mid_state_high[i] = HIGH_BITS;
            }
            1 => {
                mid_state_low[i] = LOW_BITS;
                mid_state_high[i] = HIGH_BITS;
            }
            _ => {
                mid_state_low[i] = HIGH_BITS;
                mid_state_high[i] = LOW_BITS;
            }
        }
        offset += 1;
    }
    mid_state_low[162] =
        0b1101_1011_0110_1101_1011_0110_1101_1011_0110_1101_1011_0110_1101_1011_0110_1101;
    mid_state_high[162] =
        0b1011_0110_1101_1011_0110_1101_1011_0110_1101_1011_0110_1101_1011_0110_1101_1011;
    mid_state_low[162 + 1] =
        0b1111_0001_1111_1000_1111_1100_0111_1110_0011_1111_0001_1111_1000_1111_1100_0111;
    mid_state_high[162 + 1] =
        0b1000_1111_1100_0111_1110_0011_1111_0001_1111_1000_1111_1100_0111_1110_0011_1111;
    mid_state_low[162 + 2] =
        0b0111_1111_1111_1111_1110_0000_0000_1111_1111_1111_1111_1100_0000_0001_1111_1111;
    mid_state_high[162 + 2] =
        0b1111_1111_1100_0000_0001_1111_1111_1111_1111_1000_0000_0011_1111_1111_1111_1111;
    mid_state_low[162 + 3] =
        0b1111_1111_1100_0000_0000_0000_0000_0000_0000_0111_1111_1111_1111_1111_1111_1111;
    mid_state_high[162 + 3] =
        0b0000_0000_0011_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
}

fn transform(
    state_low: &mut [u64],
    state_high: &mut [u64],
    scratchpad_low: &mut [u64],
    scratchpad_high: &mut [u64],
) {
    let mut scratch_index = 0;
    for _ in 0..81 {
        copy(state_low, state_high, scratchpad_low, scratchpad_high);
        for state_index in 0..CURL_STATE_LENGTH {
            let alpha = scratchpad_low[scratch_index];
            let beta = scratchpad_high[scratch_index];
            if scratch_index < 365 {
                scratch_index += 364;
            } else {
                scratch_index -= 365;
            }
            let gamma = scratchpad_high[scratch_index];
            let delta = (alpha | (!gamma)) & (scratchpad_low[scratch_index] ^ beta);

            state_low[state_index] = !delta;
            state_high[state_index] = (alpha ^ gamma) | delta;
        }
    }
}

fn increment(mid_low: &mut [u64], mid_high: &mut [u64], from_index: usize, to_index: usize) {
    let mut carry = 1;
    let mut low: u64;
    let mut hi: u64;
    let mut i = from_index;
    while i < to_index && carry != 0 {
        low = mid_low[i];
        hi = mid_high[i];
        mid_low[i] = hi ^ low;
        mid_high[i] = low;
        carry = hi & (!low);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::{thread_rng, Rng};
    use std::time::{Duration, Instant};
    use sponge::curl::*;

    const HASH_SIZE: usize = 243;
    const MIN_WEIGHT_MAGNITUDE: usize = 9;

    #[test]
    fn test_cancel() {
        let mut pearl_diver = PearlDiver::new();
        pearl_diver.cancel();
    }

    #[test]
    fn test_pow_works() {
        let mut rng = thread_rng();
        let mut curl = Curl::new_curl_p81();
        let vec: Vec<i8> = (0..8019).map(|_| rng.gen_range(-1, 2)).collect();
        let mut trits = [0; 8019];
        trits.copy_from_slice(&vec);
        let (_b, mut t) = search(trits, MIN_WEIGHT_MAGNITUDE);
        let mut hash_trits = [0; HASH_SIZE];
        curl.reset();
        curl.absorb(t);
        curl.squeeze(&mut hash_trits);
        for j in (HASH_SIZE - MIN_WEIGHT_MAGNITUDE..HASH_SIZE - 1).rev() {
            assert_eq!(hash_trits[j], 0);
        }
    }

    // Recommended to run this with --release
    #[test]
    fn test_no_random_fail() {
        let mut rng = thread_rng();
        let mut curl = Curl::new_curl_p81();
        let now = Instant::now();
        let count = 100;
        for i in 0..count {
            let vec: Vec<i8> = (0..8019).map(|_| rng.gen_range(-1, 2)).collect();
            let mut trits = [0; 8019];
            trits.copy_from_slice(&vec);
            let (_b, mut t) = search(trits, MIN_WEIGHT_MAGNITUDE);
            let mut hash_trits = [0; HASH_SIZE];
            curl.reset();
            curl.absorb(t);
            curl.squeeze(&mut hash_trits);
            for j in (HASH_SIZE - MIN_WEIGHT_MAGNITUDE..HASH_SIZE - 1).rev() {
                assert_eq!(hash_trits[j], 0);
            }
            if i % (count/10) == 0 {
                println!("{} successful hashes.", i);
            }
        }
        println!("{:?}", now.elapsed()/count);
        println!("{:?} pow/sec", (count as u64/now.elapsed().as_secs()));

    }
}