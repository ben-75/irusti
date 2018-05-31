use std::str::FromStr;

const RADIX :u8 = 3;
const MAX_TRIT_VALUE :u8 = (RADIX - 1) / 2;
const NUMBER_OF_TRITS_IN_A_BYTE :u8 = 5;
const NUMBER_OF_TRITS_IN_A_TRYTE :u8 = 3;

pub struct TxHash{
    arr :[bool;405],
}

impl ToString for TxHash{

    fn to_string(&self) -> String {
        let mut s = String::with_capacity(81);
        let mut i :usize = 0;
        for _ in 0..81 {
            match (self.arr[i], self.arr[i + 1], self.arr[i + 2], self.arr[i + 3], self.arr[i + 4]) {
                (false, false, false, false, false) => s.push('9'),
                (false, false, false, false, true) => s.push('A'),
                (false, false, false, true, false) => s.push('B'),
                (false, false, false, true, true) => s.push('C'),
                (false, false, true, false, false) => s.push('D'),
                (false, false, true, false, true) => s.push('E'),
                (false, false, true, true, false) => s.push('F'),
                (false, false, true, true, true) => s.push('G'),
                (false, true, false, false, false) => s.push('H'),
                (false, true, false, false, true) => s.push('I'),
                (false, true, false, true, false) => s.push('J'),
                (false, true, false, true, true) => s.push('K'),
                (false, true, true, false, false) => s.push('L'),
                (false, true, true, false, true) => s.push('M'),
                (false, true, true, true, false) => s.push('N'),
                (false, true, true, true, true) => s.push('O'),
                (true, false, false, false, false) => s.push('P'),
                (true, false, false, false, true) => s.push('Q'),
                (true, false, false, true, false) => s.push('R'),
                (true, false, false, true, true) => s.push('S'),
                (true, false, true, false, false) => s.push('T'),
                (true, false, true, false, true) => s.push('U'),
                (true, false, true, true, false) => s.push('V'),
                (true, false, true, true, true) => s.push('W'),
                (true, true, false, false, false) => s.push('X'),
                (true, true, false, false, true) => s.push('Y'),
                (true, true, false, true, false) => s.push('Z'),
                (true, true, true, _, _) => panic!(1),
                (true, true, false, true, true) => panic!(1),
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
                        '9' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false},
                        'A' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},
                        'B' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},
                        'C' => {arr[i]=false;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=true},
                        'D' => {arr[i]=false;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=false},
                        'E' => {arr[i]=false;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=true},
                        'F' => {arr[i]=false;arr[i+1]=false;arr[i+2]=true;arr[i+3]=true;arr[i+4]=false},
                        'G' => {arr[i]=false;arr[i+1]=false;arr[i+2]=true;arr[i+3]=true;arr[i+4]=true},
                        'H' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false},
                        'I' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},
                        'J' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},
                        'K' => {arr[i]=false;arr[i+1]=true;arr[i+2]=false;arr[i+3]=true;arr[i+4]=true},
                        'L' => {arr[i]=false;arr[i+1]=true;arr[i+2]=true;arr[i+3]=false;arr[i+4]=false},
                        'M' => {arr[i]=false;arr[i+1]=true;arr[i+2]=true;arr[i+3]=false;arr[i+4]=true},
                        'N' => {arr[i]=false;arr[i+1]=true;arr[i+2]=true;arr[i+3]=true;arr[i+4]=false},
                        'O' => {arr[i]=false;arr[i+1]=true;arr[i+2]=true;arr[i+3]=true;arr[i+4]=true},
                        'P' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false},
                        'Q' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},
                        'R' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},
                        'S' => {arr[i]=true;arr[i+1]=false;arr[i+2]=false;arr[i+3]=true;arr[i+4]=true},
                        'T' => {arr[i]=true;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=false},
                        'U' => {arr[i]=true;arr[i+1]=false;arr[i+2]=true;arr[i+3]=false;arr[i+4]=true},
                        'V' => {arr[i]=true;arr[i+1]=false;arr[i+2]=true;arr[i+3]=true;arr[i+4]=false},
                        'W' => {arr[i]=true;arr[i+1]=false;arr[i+2]=true;arr[i+3]=true;arr[i+4]=true},
                        'X' => {arr[i]=true;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=false},
                        'Y' => {arr[i]=true;arr[i+1]=true;arr[i+2]=false;arr[i+3]=false;arr[i+4]=true},
                        'Z' => {arr[i]=true;arr[i+1]=true;arr[i+2]=false;arr[i+3]=true;arr[i+4]=false},
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