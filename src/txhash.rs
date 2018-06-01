use std::str::FromStr;

const RADIX :u8 = 3;
const MAX_TRIT_VALUE :u8 = (RADIX - 1) / 2;
const NUMBER_OF_TRITS_IN_A_BYTE :u8 = 5;
const NUMBER_OF_TRITS_IN_A_TRYTE :u8 = 3;

pub struct TxHash{
    arr :[bool;405],
}

impl TxHash {

    pub fn trailing_zeros(&self) -> i32 {
        let mut zeros = 0;
        for i in (1..82).rev() {
            let startIndex = (i*5)-1;
            match (self.arr[startIndex - 4], self.arr[startIndex - 3], self.arr[startIndex - 2], self.arr[startIndex-1], self.arr[startIndex]){
                (false, false, false, false, false) => zeros += 3,
                (true, false, false, false, false) => {zeros += 2;break;},
                (false, true, false, false, false) => {zeros += 1;break;},
                (true, true, false, false, false) => {zeros += 1;break;},
                (false, false, true, false, false) => {zeros += 1;break;},

                (true, true, true, false, true) => {zeros += 1;break;},
                (false, false, false, true, true) => {zeros += 1;break;},
                (true, false, false, true, true) => {zeros += 1;break;},
                (false, true, false, true, true) => {zeros += 2;break;},
                _ => {break;},
            }
        };
        zeros
    }
}

impl ToString for TxHash{

    fn to_string(&self) -> String {
        let mut s = String::with_capacity(81);
        let mut i :usize = 0;
        for _ in 0..81 {
            match (self.arr[i], self.arr[i + 1], self.arr[i + 2], self.arr[i + 3], self.arr[i + 4]) {
                (false, false, false, false, false) => s.push('9'),
                (true, false, false, false, false) => s.push('A'),
                (false, true, false, false, false) => s.push('B'),
                (true, true, false, false, false) => s.push('C'),
                (false, false, true, false, false) => s.push('D'),
                (true, false, true, false, false) => s.push('E'),
                (false, true, true, false, false) => s.push('F'),
                (true, true, true, false, false) => s.push('G'),
                (false, false, false, true, false) => s.push('H'),
                (true, false, false, true, false) => s.push('I'),
                (false, true, false, true, false) => s.push('J'),
                (true, true, false, true, false) => s.push('K'),
                (false, false, true, true, false) => s.push('L'),
                (true, false, true, true, false) => s.push('M'),
                (false, true, true, true, false) => s.push('N'),
                (true, true, true, true, false) => s.push('O'),
                (false, false, false, false, true) => s.push('P'),
                (true, false, false, false, true) => s.push('Q'),
                (false, true, false, false, true) => s.push('R'),
                (true, true, false, false, true) => s.push('S'),
                (false, false, true, false, true) => s.push('T'),
                (true, false, true, false, true) => s.push('U'),
                (false, true, true, false, true) => s.push('V'),
                (true, true, true, false, true) => s.push('W'),
                (false, false, false, true, true) => s.push('X'),
                (true, false, false, true, true) => s.push('Y'),
                (false, true, false, true, true) => s.push('Z'),
                _ => panic!(1),
            }
            i += 5;
        }
        s
    }
}

impl FromStr for TxHash {
    type Err = ();

    fn from_str(s: &str) -> Result<TxHash, ()> {
        match s.len() {
            81 => {
                let mut i = 0;
                let mut arr :[bool;405] = [false;405];
                for c in s.to_string().chars() {
                    match c {
                        '9' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false}, //3
                        'A' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false},  //2
                        'B' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false},  //1
                        'C' => {arr[i]=true;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false},   //1
                        'D' => {arr[i]=false;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=false},  //1
                        'E' => {arr[i]=true;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=false},   //0
                        'F' => {arr[i]=false;arr[i+1]=true;arr[i+2]=true;arr[i+3]=false;arr[i+4]=false},   //0
                        'G' => {arr[i]=true;arr[i+1]=true;arr[i+2]=true;arr[i+3]=false;arr[i+4]=false},    //0
                        'H' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},  //0
                        'I' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},   //0
                        'J' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},   //0
                        'K' => {arr[i]=true;arr[i+1]=true;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},    //0
                        'L' => {arr[i]=false;arr[i+1]=false;arr[i+2]=true;arr[i+3]=true;arr[i+4]=false},   //0
                        'M' => {arr[i]=true;arr[i+1]=false;arr[i+2]=true;arr[i+3]=true;arr[i+4]=false},    //0
                        'N' => {arr[i]=false;arr[i+1]=true;arr[i+2]=true;arr[i+3]=true;arr[i+4]=false},    //0
                        'O' => {arr[i]=true;arr[i+1]=true;arr[i+2]=true;arr[i+3]=true;arr[i+4]=false},     //0
                        'P' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},  //0
                        'Q' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},   //0
                        'R' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},   //0
                        'S' => {arr[i]=true;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},    //0
                        'T' => {arr[i]=false;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=true},   //0
                        'U' => {arr[i]=true;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=true},    //0
                        'V' => {arr[i]=false;arr[i+1]=true;arr[i+2]=true;arr[i+3]=false;arr[i+4]=true},    //0
                        'W' => {arr[i]=true;arr[i+1]=true;arr[i+2]=true;arr[i+3]=false;arr[i+4]=true},     //1
                        'X' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=true},   //1
                        'Y' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=true},    //1
                        'Z' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=true;arr[i+4]=true},    //2
                        _ => return Err(()),
                    }
                    i += 5;
                }
                Ok(TxHash{arr})

            }
            _ => Err(())
         }
    }
}