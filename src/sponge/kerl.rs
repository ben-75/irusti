//! This module currently uses a custom implementation of bigint
//! Due to num-bigint being significantly slower, the plan is to
//! use this until ApInt or ramp are good enough, then use those
//! instead.

use sponge::sponge::Sponge;
use sponge::keccak::Keccak;
const BIT_HASH_LENGTH: usize = 384;
const BYTE_HASH_LENGTH: usize = BIT_HASH_LENGTH / 8;

const RADIX: i32 = 3;
const MAX_TRIT_VALUE: i32 = (RADIX - 1) / 2;
const MIN_TRIT_VALUE: i32 = -MAX_TRIT_VALUE;

const HASH_LENGTH: usize = 243;
const BYTE_LENGTH: usize = 48;
const INT_LENGTH: usize = BYTE_LENGTH / 4;

const HALF_3: [u32; 12] = [
    0xa5ce_8964,
    0x9f00_7669,
    0x1484_504f,
    0x3ade_00d9,
    0x0c24_486e,
    0x5097_9d57,
    0x79a4_c702,
    0x48bb_ae36,
    0xa9f6_808b,
    0xaa06_a805,
    0xa87f_abdf,
    0x5e69_ebef,
];

#[derive(Clone)]
pub struct Kerl {
    keccak: Keccak,
    byte_state: [u8; BYTE_HASH_LENGTH],
    trit_state: [i8; HASH_LENGTH],
}

impl Default for Kerl {
    fn default() -> Kerl {
        Kerl {
            keccak: Keccak::new_keccak384(),
            byte_state: [0; BYTE_HASH_LENGTH],
            trit_state: [0; HASH_LENGTH],
        }
    }
}

impl Sponge for Kerl {
    fn absorb(&mut self, trits: &mut [i8]) {
        assert_eq!(trits.len() % HASH_LENGTH, 0);
        let mut bytes = [0; BYTE_LENGTH];
        for chunk in trits.chunks(HASH_LENGTH) {
            self.trit_state.copy_from_slice(chunk);
            self.trit_state[HASH_LENGTH - 1] = 0;
            trits_to_bytes(chunk, &mut bytes);
            self.keccak.update(&bytes);
        }
    }

    fn squeeze(&mut self, trits: &mut [i8]) {
        assert_eq!(trits.len() % HASH_LENGTH, 0);
        for chunk in trits.chunks_mut(HASH_LENGTH) {
            self.keccak.pad();
            self.keccak.fill_block();
            self.keccak.squeeze(&mut self.byte_state);
            self.keccak = Keccak::new_keccak384();
            bytes_to_trits(&mut self.byte_state, &mut self.trit_state);
            self.trit_state[HASH_LENGTH - 1] = 0;
            chunk.copy_from_slice(&self.trit_state[0..HASH_LENGTH]);
            for b in self.byte_state.iter_mut() {
                *b ^= 0xFF;
            }
            self.keccak.update(&self.byte_state);
        }
    }
}

impl Kerl {
    pub fn reset(&mut self) {
        self.keccak = Keccak::new_keccak384();
    }
    fn trit_state(&self) -> &[i8] {
        &self.trit_state
    }
    fn trit_state_mut(&mut self) -> &mut [i8] {
        &mut self.trit_state
    }
}

pub fn trits_to_bytes(trits: &[i8], bytes: &mut [u8]) {
    assert_eq!(trits.len(), HASH_LENGTH);
    assert_eq!(bytes.len(), BYTE_LENGTH);

    let mut base = [0; INT_LENGTH];

    let mut size = 1;
    let mut all_minus_1 = true;

    for t in trits[0..HASH_LENGTH - 1].iter() {
        if *t != -1 {
            all_minus_1 = false;
            break;
        }
    }

    if all_minus_1 {
        base.clone_from_slice(&HALF_3);
        bigint_not(&mut base);
        bigint_add_base(&mut base, 1_u32);
    } else {
        for t in trits[0..HASH_LENGTH - 1].iter().rev() {
            {
                let sz = size;
                let mut carry: u32 = 0;

                for b in base.iter_mut().take(sz) {
                    let v = u64::from(*b) * (RADIX as u64) + u64::from(carry);
                    let (newcarry, newbase) = ((v >> 32) as u32, v as u32);
                    carry = newcarry;
                    *b = newbase;
                }

                if carry > 0 {
                    base[sz] = carry;
                    size += 1;
                }
            }

            let trit = (t + 1) as u32;
            {
                let sz = bigint_add_base(&mut base, trit) as usize;
                if sz > size {
                    size = sz;
                }
            }
        }

        if !is_null(&base) {
            if bigint_cmp(&HALF_3, &base) <= 0 {
                bigint_sub(&mut base, &HALF_3);
            } else {
                let mut tmp = HALF_3;
                bigint_sub(&mut tmp, &base);
                bigint_not(&mut tmp);
                bigint_add_base(&mut tmp, 1);
                base.copy_from_slice(&tmp);
            }
        }
    }

    let mut out = vec![0; BYTE_LENGTH];
    for i in 0..INT_LENGTH {
        out[i * 4] = ((base[INT_LENGTH - 1 - i] & 0xFF00_0000) >> 24) as u8;
        out[i * 4 + 1] = ((base[INT_LENGTH - 1 - i] & 0x00FF_0000) >> 16) as u8;
        out[i * 4 + 2] = ((base[INT_LENGTH - 1 - i] & 0x0000_FF00) >> 8) as u8;
        out[i * 4 + 3] = (base[INT_LENGTH - 1 - i] & 0x0000_00FF) as u8;
    }
    bytes.copy_from_slice(&out);
}

pub fn bytes_to_trits(bytes: &mut [u8], trits: &mut [i8]) {
    assert_eq!(bytes.len(), BYTE_LENGTH);
    assert_eq!(trits.len(), HASH_LENGTH);

    let mut base = vec![0; INT_LENGTH];
    trits[HASH_LENGTH - 1] = 0;

    for i in 0..INT_LENGTH {
        base[INT_LENGTH - 1 - i] = u32::from(bytes[i * 4]) << 24;
        base[INT_LENGTH - 1 - i] |= u32::from(bytes[i * 4 + 1]) << 16;
        base[INT_LENGTH - 1 - i] |= u32::from(bytes[i * 4 + 2]) << 8;
        base[INT_LENGTH - 1 - i] |= u32::from(bytes[i * 4 + 3]);
    }

    let mut flip_trits = false;

    if base[INT_LENGTH - 1] >> 31 == 0 {
        bigint_add(&mut base, &HALF_3);
    } else {
        bigint_not(&mut base);
        if bigint_cmp(&base, &HALF_3) > 0 {
            bigint_sub(&mut base, &HALF_3);
            flip_trits = true;
        } else {
            bigint_add_base(&mut base, 1);
            let mut tmp = HALF_3;
            bigint_sub(&mut tmp, &base);
            base.copy_from_slice(&tmp);
        }
    }

    let mut rem;
    for trit in trits.iter_mut().take(HASH_LENGTH - 1) {
        rem = 0;
        for j in (0..INT_LENGTH).rev() {
            let lhs = (u64::from(rem) << 32) | (u64::from(base[j]));
            let rhs = RADIX as u64;
            let q = (lhs / rhs) as u32;
            let r = (lhs % rhs) as u32;

            base[j] = q;
            rem = r;
        }
        *trit = rem as i8 - 1;
    }

    if flip_trits {
        for v in trits.iter_mut() {
            *v = -*v;
        }
    }
}

fn bigint_not(base: &mut [u32]) {
    for i in base.iter_mut() {
        *i = !*i;
    }
}

fn bigint_add_base(base: &mut [u32], rh: u32) -> u32 {
    let mut res = full_add(base[0], rh, false);
    base[0] = res.0;
    let mut j = 0;
    while res.1 {
        res = full_add(base[j], 0, true);
        base[j] = res.0;
        j += 1;
    }
    j as u32
}

fn bigint_add(base: &mut [u32], rh: &[u32]) {
    let mut carry = false;

    for (a, b) in base.iter_mut().zip(rh.iter()) {
        let (v, c) = full_add(*a, *b, carry);
        *a = v;
        carry = c;
    }
}

fn bigint_cmp(lh: &[u32], rh: &[u32]) -> i8 {
    for (a, b) in lh.iter().rev().zip(rh.iter().rev()) {
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
    }
    0
}

fn bigint_sub(base: &mut [u32], rh: &[u32]) {
    let mut noborrow = true;
    for (a, b) in base.iter_mut().zip(rh) {
        let (v, c) = full_add(*a, !*b, noborrow);
        *a = v;
        noborrow = c;
    }
    assert!(noborrow);
}

fn is_null(base: &[u32]) -> bool {
    for b in base.iter() {
        if *b != 0 {
            return false;
        }
    }
    true
}

fn full_add(ia: u32, ib: u32, carry: bool) -> (u32, bool) {
    let a = u64::from(ia);
    let b = u64::from(ib);

    let mut v = a + b;
    let mut l = v >> 32;
    let mut r = v & 0xFFFF_FFFF;

    let carry1 = l != 0;

    if carry {
        v = r + 1;
    }
    l = (v >> 32) & 0xFFFF_FFFF;
    r = v & 0xFFFF_FFFF;
    let carry2 = l != 0;
    (r as u32, carry1 || carry2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use converter::*;

    #[test]
    fn kerl_one_absorb() {
        let mut trits: Vec<i8> = trits_from_string(
            "GYOMKVTSNHVJNCNFBBAH9AAMXLPLLLROQY99QN9DLSJUHDPBLCFFAIQXZA9BKMBJCYSFHFPXAHDWZFEIZ",
        ).unwrap();
        let mut kerl = Kerl::default();
        kerl.absorb( trits.as_mut());
        let mut out = [0; 243];
        kerl.squeeze(&mut out);
        assert_eq!(
            trits_to_string(&out).unwrap(),
            "OXJCNFHUNAHWDLKKPELTBFUCVW9KLXKOGWERKTJXQMXTKFKNWNNXYD9DMJJABSEIONOSJTTEVKVDQEWTW"
        );
    }

    #[test]
    fn kerl_multi_squeeze_multi_absorb() {
        let mut trits = trytes_to_trits("G9JYBOMPUXHYHKSNRNMMSSZCSHOFYOYNZRSZMAAYWDYEIMVVOGKPJBVBM9TD\
PULSFUNMTVXRKFIDOHUXXVYDLFSZYZTWQYTE9SPYYWYTXJYQ9IFGYOLZXWZBKWZN9QOOTBQMWMUBLEWUEEASRHRTNIQW\
JQNDWRYLCA");

        let mut kerl = Kerl::default();
        kerl.absorb(trits.as_mut());

        let mut out = [0; 486];

        kerl.squeeze(&mut out);
        assert_eq!(
            trits_to_string(&out).unwrap(),
            "LUCKQVACOGBFYSPPVSSOXJEKNSQQRQKPZC9NXFSMQNRQCGGUL9OHVVKBDSKEQEBKXRNUJSRXYVHJTXBPD\
             WQGNSCDCBAIRHAQCOWZEBSNHIJIGPZQITIBJQ9LNTDIBTCQ9EUWKHFLGFUVGGUWJONK9GBCDUIMAYMMQX"
        );
    }

    #[test]
    fn kerl_multi_squeeze() {
        let mut trits: Vec<i8> = trytes_to_trits("9MIDYNHBWMBCXVDEFOFWINXTERALUKYYPPHKP9JJFGJEIUY9MUDVNFZHMMWZUYUSWAIOWEVTHNWMHANBH");
//             "9MIDYNHBWMBCXVDEFOFWINXTERALUKYYPPHKP9JJFGJEIUY9MUDVNFZHMMWZUYUSWAIOWEVTHNWMHANBH"
//                .chars()
//                .flat_map(char_to_trits)
//                .cloned()
//                .collect();
        let mut kerl = Kerl::default();
        kerl.absorb(trits.as_mut());

        let mut out = [0; 486];
        kerl.squeeze(&mut out);
        assert_eq!(
            trits_to_string(&out).unwrap(),
            "G9JYBOMPUXHYHKSNRNMMSSZCSHOFYOYNZRSZMAAYWDYEIMVVOGKPJBVBM9TDPULSFUNMTVXRKFIDOHUXX\
             VYDLFSZYZTWQYTE9SPYYWYTXJYQ9IFGYOLZXWZBKWZN9QOOTBQMWMUBLEWUEEASRHRTNIQWJQNDWRYLCA"
        );
    }

}
