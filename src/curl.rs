pub trait Sponge {
    fn reset(&mut self);
    fn absorb(&mut self, trites_to_calculate :Vec<i8>, offset :usize, length :usize);
    fn squeeze(&mut self, offset :usize, length :usize) -> [i8;243];
}

#[derive(PartialEq)]
pub enum SpongeMode {
    CurlP27,
    CurlP81,
    KERL
}

pub struct Curl {
    number_of_rounds:i8,
    state :[i8;729]
}

const HASH_LENGTH :usize = 243;
const STATE_LENGTH :usize = 3 * HASH_LENGTH;
const TRUTH_TABLE :[i8;11]= [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

impl Sponge for Curl {
    fn reset(&mut self) {
        self.state = [0;729];
    }

    fn absorb(&mut self, trites_to_calculate :Vec<i8>, mut offset: usize,mut length :usize) {
        while {
            let l = if length < HASH_LENGTH {length} else {HASH_LENGTH};
            self.state[0..l].copy_from_slice(&trites_to_calculate[offset..offset+l]);
            self.transform();
            offset += HASH_LENGTH;

            length -= HASH_LENGTH;
            length > 0
        }{}
    }

    fn squeeze(&mut self, mut offset: usize,mut length: usize) -> [i8;243] {
        let mut trits :[i8;243] = [0;243];
        while {
            let l = if length < HASH_LENGTH {length} else {HASH_LENGTH};

            trits[offset..offset+l].copy_from_slice(&self.state[0..l]);
            self.transform();
            offset += HASH_LENGTH;

            length -= HASH_LENGTH;
            length > 0
        }{}
        trits
    }

}

impl Curl {

    pub fn new_curl_p81() -> Curl {
        Curl {
            number_of_rounds:81,
            state : [0;STATE_LENGTH]
        }
    }

    pub fn new_curl_p27() -> Curl {
        Curl {
            number_of_rounds:27,
            state : [0;STATE_LENGTH]
        }
    }

    fn transform(&mut self) {
        let mut scratchpad_index = 0;
        let mut prev_scratchpad_index = 0;
        let mut scratchpad :[i8;STATE_LENGTH] = [0;STATE_LENGTH];
        for _round in 0..self.number_of_rounds {
            scratchpad[0..STATE_LENGTH].copy_from_slice(&self.state[0..STATE_LENGTH]);
            for state_index in 0..STATE_LENGTH {
                prev_scratchpad_index = scratchpad_index;
                scratchpad_index = if scratchpad_index < 365 { scratchpad_index + 364 } else { scratchpad_index - 365 };
                self.state[state_index] = TRUTH_TABLE[(scratchpad[prev_scratchpad_index] + (scratchpad[scratchpad_index] << 2) + 5) as usize];
            }
        }
    }
}