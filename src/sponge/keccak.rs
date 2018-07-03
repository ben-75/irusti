//! An implementation of the FIPS-202-defined SHA-3 and SHAKE functions.
//!
//! The `Keccak-f[1600]` permutation is fully unrolled; it's nearly as fast
//! as the Keccak team's optimized permutation.
//!
//! ## Building
//!
//! ```bash
//! cargo build
//! ```
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tiny-keccak = "1.0"
//! ```
//!
//! and this to your crate root:
//!
//! Original implemntation in C:
//! https://github.com/coruus/keccak-tiny
//!
//! Implementor: David Leon Gil
//!
//! Port to rust:
//! Marek Kotewicz (marek.kotewicz@gmail.com)
//!
//! License: CC0, attribution kindly requested. Blame taken too,
//! but not liability.

const RHO: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
];

const PI: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];

const RC: [u64; 24] = [
    1u64,
    0x8082u64,
    0x8000_0000_0000_808au64,
    0x8000_0000_8000_8000u64,
    0x808bu64,
    0x8000_0001u64,
    0x8000_0000_8000_8081u64,
    0x8000_0000_0000_8009u64,
    0x8au64,
    0x88u64,
    0x8000_8009u64,
    0x8000_000au64,
    0x8000_808bu64,
    0x8000_0000_0000_008bu64,
    0x8000_0000_0000_8089u64,
    0x8000_0000_0000_8003u64,
    0x8000_0000_0000_8002u64,
    0x8000_0000_0000_0080u64,
    0x800au64,
    0x8000_0000_8000_000au64,
    0x8000_0000_8000_8081u64,
    0x8000_0000_0000_8080u64,
    0x8000_0001u64,
    0x8000_0000_8000_8008u64,
];

#[allow(unused_assignments)]
/// keccak-f[1600]
pub fn keccakf(a: &mut [u64; PLEN]) {
    for rc in RC.iter().take(24) {
        let mut array: [u64; 5] = [0; 5];

        // Theta



        array[0] ^= a[0];
        array[0] ^= a[5];
        array[0] ^= a[10];
        array[0] ^= a[15];
        array[0] ^= a[20];

        array[1] ^= a[1];
        array[1] ^= a[6];
        array[1] ^= a[11];
        array[1] ^= a[16];
        array[1] ^= a[21];

        array[2] ^= a[2];
        array[2] ^= a[7];
        array[2] ^= a[12];
        array[2] ^= a[17];
        array[2] ^= a[22];

        array[3] ^= a[3];
        array[3] ^= a[8];
        array[3] ^= a[13];
        array[3] ^= a[18];
        array[3] ^= a[23];

        array[4] ^= a[4];
        array[4] ^= a[9];
        array[4] ^= a[14];
        array[4] ^= a[19];
        array[4] ^= a[24];





        a[0] ^= array[(4) % 5] ^ array[(1) % 5].rotate_left(1);
        a[5 + 0] ^= array[(4) % 5] ^ array[(1) % 5].rotate_left(1);
        a[10] ^= array[(4) % 5] ^ array[(1) % 5].rotate_left(1);
        a[15 + 0] ^= array[(4) % 5] ^ array[(1) % 5].rotate_left(1);
        a[20] ^= array[(4) % 5] ^ array[(1) % 5].rotate_left(1);

        a[1] ^= array[(1 + 4) % 5] ^ array[(1 + 1) % 5].rotate_left(1);
        a[5 + 1] ^= array[(1 + 4) % 5] ^ array[(1 + 1) % 5].rotate_left(1);
        a[11] ^= array[(1 + 4) % 5] ^ array[(1 + 1) % 5].rotate_left(1);
        a[15 + 1] ^= array[(1 + 4) % 5] ^ array[(1 + 1) % 5].rotate_left(1);
        a[21] ^= array[(1 + 4) % 5] ^ array[(1 + 1) % 5].rotate_left(1);

        a[2] ^= array[(2 + 4) % 5] ^ array[(2 + 1) % 5].rotate_left(1);
        a[5 + 2] ^= array[(2 + 4) % 5] ^ array[(2 + 1) % 5].rotate_left(1);
        a[12] ^= array[(2 + 4) % 5] ^ array[(2 + 1) % 5].rotate_left(1);
        a[15 + 2] ^= array[(2 + 4) % 5] ^ array[(2 + 1) % 5].rotate_left(1);
        a[22] ^= array[(2 + 4) % 5] ^ array[(2 + 1) % 5].rotate_left(1);

        a[3] ^= array[(3 + 4) % 5] ^ array[(3 + 1) % 5].rotate_left(1);
        a[5 + 3] ^= array[(3 + 4) % 5] ^ array[(3 + 1) % 5].rotate_left(1);
        a[13] ^= array[(3 + 4) % 5] ^ array[(3 + 1) % 5].rotate_left(1);
        a[15 + 3] ^= array[(3 + 4) % 5] ^ array[(3 + 1) % 5].rotate_left(1);
        a[23] ^= array[(3 + 4) % 5] ^ array[(3 + 1) % 5].rotate_left(1);

        a[4] ^= array[(4 + 4) % 5] ^ array[(4 + 1) % 5].rotate_left(1);
        a[5 + 4] ^= array[(4 + 4) % 5] ^ array[(4 + 1) % 5].rotate_left(1);
        a[14] ^= array[(4 + 4) % 5] ^ array[(4 + 1) % 5].rotate_left(1);
        a[15 + 4] ^= array[(4 + 4) % 5] ^ array[(4 + 1) % 5].rotate_left(1);
        a[24] ^= array[(4 + 4) % 5] ^ array[(4 + 1) % 5].rotate_left(1);



        // Rho and pi
        let mut last = a[1];

        array[0] = a[PI[0]];
        a[PI[0]] = last.rotate_left(RHO[0]);
        last = array[0];

        array[0] = a[PI[1]];
        a[PI[1]] = last.rotate_left(RHO[1]);
        last = array[0];

        array[0] = a[PI[2]];
        a[PI[2]] = last.rotate_left(RHO[2]);
        last = array[0];

        array[0] = a[PI[3]];
        a[PI[3]] = last.rotate_left(RHO[3]);
        last = array[0];

        array[0] = a[PI[4]];
        a[PI[4]] = last.rotate_left(RHO[4]);
        last = array[0];

        array[0] = a[PI[5]];
        a[PI[5]] = last.rotate_left(RHO[5]);
        last = array[0];

        array[0] = a[PI[6]];
        a[PI[6]] = last.rotate_left(RHO[6]);
        last = array[0];

        array[0] = a[PI[7]];
        a[PI[7]] = last.rotate_left(RHO[7]);
        last = array[0];

        array[0] = a[PI[8]];
        a[PI[8]] = last.rotate_left(RHO[8]);
        last = array[0];

        array[0] = a[PI[9]];
        a[PI[9]] = last.rotate_left(RHO[9]);
        last = array[0];

        array[0] = a[PI[10]];
        a[PI[10]] = last.rotate_left(RHO[10]);
        last = array[0];

        array[0] = a[PI[11]];
        a[PI[11]] = last.rotate_left(RHO[11]);
        last = array[0];

        array[0] = a[PI[12]];
        a[PI[12]] = last.rotate_left(RHO[12]);
        last = array[0];

        array[0] = a[PI[13]];
        a[PI[13]] = last.rotate_left(RHO[13]);
        last = array[0];

        array[0] = a[PI[14]];
        a[PI[14]] = last.rotate_left(RHO[14]);
        last = array[0];

        array[0] = a[PI[15]];
        a[PI[15]] = last.rotate_left(RHO[15]);
        last = array[0];

        array[0] = a[PI[16]];
        a[PI[16]] = last.rotate_left(RHO[16]);
        last = array[0];

        array[0] = a[PI[17]];
        a[PI[17]] = last.rotate_left(RHO[17]);
        last = array[0];

        array[0] = a[PI[18]];
        a[PI[18]] = last.rotate_left(RHO[18]);
        last = array[0];

        array[0] = a[PI[19]];
        a[PI[19]] = last.rotate_left(RHO[19]);
        last = array[0];

        array[0] = a[PI[20]];
        a[PI[20]] = last.rotate_left(RHO[20]);
        last = array[0];

        array[0] = a[PI[21]];
        a[PI[21]] = last.rotate_left(RHO[21]);
        last = array[0];

        array[0] = a[PI[22]];
        a[PI[22]] = last.rotate_left(RHO[22]);
        last = array[0];

        array[0] = a[PI[23]];
        a[PI[23]] = last.rotate_left(RHO[23]);
        last = array[0];


        // Chi


        array[0] = a[0];
        array[1] = a[1];
        array[2] = a[2];
        array[3] = a[3];
        array[4] = a[4];

        a[0] = array[0] ^ ((!array[(1) % 5]) & (array[(2) % 5]));
        a[1] = array[1] ^ ((!array[(1 + 1) % 5]) & (array[(1 + 2) % 5]));
        a[2] = array[2] ^ ((!array[(2 + 1) % 5]) & (array[(2 + 2) % 5]));
        a[3] = array[3] ^ ((!array[(3 + 1) % 5]) & (array[(3 + 2) % 5]));
        a[4] = array[4] ^ ((!array[(4 + 1) % 5]) & (array[(4 + 2) % 5]));


        array[0] = a[5];
        array[1] = a[5 + 1];
        array[2] = a[5 + 2];
        array[3] = a[5 + 3];
        array[4] = a[5 + 4];

        a[5 + 0] = array[0] ^ ((!array[(1) % 5]) & (array[(2) % 5]));
        a[5 + 1] = array[1] ^ ((!array[(1 + 1) % 5]) & (array[(1 + 2) % 5]));
        a[5 + 2] = array[2] ^ ((!array[(2 + 1) % 5]) & (array[(2 + 2) % 5]));
        a[5 + 3] = array[3] ^ ((!array[(3 + 1) % 5]) & (array[(3 + 2) % 5]));
        a[5 + 4] = array[4] ^ ((!array[(4 + 1) % 5]) & (array[(4 + 2) % 5]));


        array[0] = a[10];
        array[1] = a[11];
        array[2] = a[12];
        array[3] = a[13];
        array[4] = a[14];

        a[10] = array[0] ^ ((!array[(1) % 5]) & (array[(2) % 5]));
        a[11] = array[1] ^ ((!array[(1 + 1) % 5]) & (array[(1 + 2) % 5]));
        a[12] = array[2] ^ ((!array[(2 + 1) % 5]) & (array[(2 + 2) % 5]));
        a[13] = array[3] ^ ((!array[(3 + 1) % 5]) & (array[(3 + 2) % 5]));
        a[14] = array[4] ^ ((!array[(4 + 1) % 5]) & (array[(4 + 2) % 5]));


        array[0] = a[15];
        array[1] = a[15 + 1];
        array[2] = a[15 + 2];
        array[3] = a[15 + 3];
        array[4] = a[15 + 4];

        a[15 + 0] = array[0] ^ ((!array[(1) % 5]) & (array[(2) % 5]));
        a[15 + 1] = array[1] ^ ((!array[(1 + 1) % 5]) & (array[(1 + 2) % 5]));
        a[15 + 2] = array[2] ^ ((!array[(2 + 1) % 5]) & (array[(2 + 2) % 5]));
        a[15 + 3] = array[3] ^ ((!array[(3 + 1) % 5]) & (array[(3 + 2) % 5]));
        a[15 + 4] = array[4] ^ ((!array[(4 + 1) % 5]) & (array[(4 + 2) % 5]));


        array[0] = a[20];
        array[1] = a[21];
        array[2] = a[22];
        array[3] = a[23];
        array[4] = a[24];

        a[20] = array[0] ^ ((!array[(1) % 5]) & (array[(2) % 5]));
        a[21] = array[1] ^ ((!array[(1 + 1) % 5]) & (array[(1 + 2) % 5]));
        a[22] = array[2] ^ ((!array[(2 + 1) % 5]) & (array[(2 + 2) % 5]));
        a[23] = array[3] ^ ((!array[(3 + 1) % 5]) & (array[(3 + 2) % 5]));
        a[24] = array[4] ^ ((!array[(4 + 1) % 5]) & (array[(4 + 2) % 5]));


        // Iota
        a[0] ^= rc;
    }
}

fn setout(src: &[u8], dst: &mut [u8], len: usize) {
    dst[..len].copy_from_slice(&src[..len]);
}

fn xorin(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() <= src.len());
    let len = dst.len();
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    for _ in 0..len {
        unsafe {
            *dst_ptr ^= *src_ptr;
            src_ptr = src_ptr.offset(1);
            dst_ptr = dst_ptr.offset(1);
        }
    }
}

/// Total number of lanes.
const PLEN: usize = 25;

/// This structure should be used to create keccak/sha3 hash.
///
/// ```rust
/// extern crate iota_lib_rs;
/// use iota_lib_rs::pow::keccak::Keccak;
///
/// fn main() {
///     let mut sha3 = Keccak::new_sha3_256();
///     let data: Vec<u8> = From::from("hello");
///     let data2: Vec<u8> = From::from("world");
///
///     sha3.update(&data);
///     sha3.update(&[b' ']);
///     sha3.update(&data2);
///
///     let mut res: [u8; 32] = [0; 32];
///     sha3.finalize(&mut res);
///
///     let expected = vec![
///         0x64, 0x4b, 0xcc, 0x7e, 0x56, 0x43, 0x73, 0x04,
///         0x09, 0x99, 0xaa, 0xc8, 0x9e, 0x76, 0x22, 0xf3,
///         0xca, 0x71, 0xfb, 0xa1, 0xd9, 0x72, 0xfd, 0x94,
///         0xa3, 0x1c, 0x3b, 0xfb, 0xf2, 0x4e, 0x39, 0x38
///     ];
///
///     let ref_ex: &[u8] = &expected;
///     assert_eq!(&res, ref_ex);
/// }
/// ```
#[derive(Clone, Copy)]
pub struct Keccak {
    a: [u64; PLEN],
    offset: usize,
    rate: usize,
    delim: u8,
}

// impl Clone for Keccak {
//     fn clone(&self) -> Self {
//         let mut res = Keccak::new(self.rate, self.delim);
//         res.a.copy_from_slice(&self.a);
//         res.offset = self.offset;
//         res
//     }
// }

macro_rules! impl_constructor {
    ($name:ident, $alias:ident, $bits:expr, $delim:expr) => {
        pub fn $name() -> Keccak {
            Keccak::new(200 - $bits / 4, $delim)
        }

        pub fn $alias(data: &[u8], result: &mut [u8]) {
            let mut keccak = Keccak::$name();
            keccak.update(data);
            keccak.finalize(result);
        }
    };
}

macro_rules! impl_global_alias {
    ($alias:ident, $size:expr) => {
        pub fn $alias(data: &[u8]) -> [u8; $size / 8] {
            let mut result = [0u8; $size / 8];
            Keccak::$alias(data, &mut result);
            result
        }
    };
}

impl_global_alias!(shake128, 128);
impl_global_alias!(shake256, 256);
impl_global_alias!(keccak224, 224);
impl_global_alias!(keccak256, 256);
impl_global_alias!(keccak384, 384);
impl_global_alias!(keccak512, 512);
impl_global_alias!(sha3_224, 224);
impl_global_alias!(sha3_256, 256);
impl_global_alias!(sha3_384, 384);
impl_global_alias!(sha3_512, 512);

impl Keccak {
    pub fn new(rate: usize, delim: u8) -> Keccak {
        Keccak {
            a: [0; PLEN],
            offset: 0,
            rate,
            delim,
        }
    }

    impl_constructor!(new_shake128, shake128, 128, 0x1f);
    impl_constructor!(new_shake256, shake256, 256, 0x1f);
    impl_constructor!(new_keccak224, keccak224, 224, 0x01);
    impl_constructor!(new_keccak256, keccak256, 256, 0x01);
    impl_constructor!(new_keccak384, keccak384, 384, 0x01);
    impl_constructor!(new_keccak512, keccak512, 512, 0x01);
    impl_constructor!(new_sha3_224, sha3_224, 224, 0x06);
    impl_constructor!(new_sha3_256, sha3_256, 256, 0x06);
    impl_constructor!(new_sha3_384, sha3_384, 384, 0x06);
    impl_constructor!(new_sha3_512, sha3_512, 512, 0x06);

    fn a_bytes(&self) -> &[u8; PLEN * 8] {
        unsafe { &*(&self.a as *const [u64; 25] as *const [u8; 200]) }
    }

    fn a_mut_bytes(&mut self) -> &mut [u8; PLEN * 8] {
        unsafe { &mut *(&mut self.a as *mut [u64; 25] as *mut [u8; 200]) }
    }

    pub fn update(&mut self, input: &[u8]) {
        self.absorb(input);
    }

    #[inline]
    pub fn keccakf(&mut self) {
        keccakf(&mut self.a);
    }

    pub fn finalize(mut self, output: &mut [u8]) {
        self.pad();

        // apply keccakf
        keccakf(&mut self.a);

        // squeeze output
        self.squeeze(output);
    }

    // Absorb input
    pub fn absorb(&mut self, input: &[u8]) {
        //first foldp
        let mut ip = 0;
        let mut l = input.len();
        let mut rate = self.rate - self.offset;
        let mut offset = self.offset;
        while l >= rate {
            xorin(&mut self.a_mut_bytes()[offset..][..rate], &input[ip..]);
            keccakf(&mut self.a);
            ip += rate;
            l -= rate;
            rate = self.rate;
            offset = 0;
        }

        // Xor in the last block
        xorin(&mut self.a_mut_bytes()[offset..][..l], &input[ip..]);
        self.offset = offset + l;
    }

    pub fn pad(&mut self) {
        let offset = self.offset;
        let rate = self.rate;
        let delim = self.delim;
        let aa = self.a_mut_bytes();
        aa[offset] ^= delim;
        aa[rate - 1] ^= 0x80;
    }

    pub fn fill_block(&mut self) {
        self.keccakf();
        self.offset = 0;
    }

    // squeeze output
    pub fn squeeze(&mut self, output: &mut [u8]) {
        // second foldp
        let mut op = 0;
        let mut l = output.len();
        while l >= self.rate {
            setout(self.a_bytes(), &mut output[op..], self.rate);
            keccakf(&mut self.a);
            op += self.rate;
            l -= self.rate;
        }

        setout(self.a_bytes(), &mut output[op..], l);
    }

    #[inline]
    pub fn xof(mut self) -> XofReader {
        self.pad();

        keccakf(&mut self.a);

        XofReader {
            keccak: self,
            offset: 0,
        }
    }
}

pub struct XofReader {
    keccak: Keccak,
    offset: usize,
}

impl XofReader {
    pub fn squeeze(&mut self, output: &mut [u8]) {
        // second foldp
        let mut op = 0;
        let mut l = output.len();
        let mut rate = self.keccak.rate - self.offset;
        let mut offset = self.offset;
        while l >= rate {
            setout(&self.keccak.a_bytes()[offset..], &mut output[op..], rate);
            self.keccak.keccakf();
            op += rate;
            l -= rate;
            rate = self.keccak.rate;
            offset = 0;
        }

        setout(&self.keccak.a_bytes()[offset..], &mut output[op..], l);
        self.offset = offset + l;
    }
}
