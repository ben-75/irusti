use std::str::FromStr;
use curl::SpongeMode;
use curl::Curl;
use curl::Sponge;



const RADIX :u8 = 3;
const MAX_TRIT_VALUE :u8 = (RADIX - 1) / 2;
const NUMBER_OF_TRITS_IN_A_BYTE :u8 = 5;
const NUMBER_OF_TRITS_IN_A_TRYTE :u8 = 3;
const SIZE_IN_TRITS :usize = 243;

pub struct TxHash{
    arr :[bool;405],
}

impl TxHash {



    pub fn trailing_zeros(&self) -> i32 {
        let mut zeros = 0;
        for i in (1..82).rev() {
            let start_index = (i*5)-1;
            match (self.arr[start_index - 4], self.arr[start_index - 3], self.arr[start_index - 2], self.arr[start_index-1], self.arr[start_index]){
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

    pub fn compute(trytes : String, mode :SpongeMode) -> Result<TxHash,()> {
        let sz = 3*trytes.len();
        let mut integers :Vec<i8> = Vec::with_capacity(sz);
        integers = TxHash::to_i8(trytes,integers);
        let mut curl = match mode {
            SpongeMode::CurlP27 => {Curl::new_curl_p27()}
            _ => Curl::new_curl_p81(),
        };
        curl.reset();
        curl.absorb(integers,0,sz);
        TxHash::from_i8(curl.squeeze(0,243))
    }

    /*
000		    9  (false,false,false,false,false,false)
100		    A  (true,true,false,false,false,false)
-110		B  (true,false,true,true,false,false)
010		    C  (false,false,true,true,false,false)
110		    D  (true,true,true,true,false,false)
-1-11		E  (true,false,true,false,true,true)
0-11		F  (false,false,true,false,true,true)
1-11		G  (true,true,true,false,true,true)
-101		H  (true,false,false,false,true,true)
001		    I  (false,false,false,false,true,true)
101		    J  (true,true,false,false,true,true)
-111		K  (true,false,true,true,true,true)
011		    L  (false,false,true,true,true,true)
111		    M  (true,true,true,true,true,true)
-1-1-1	    N  (true,false,true,false,true,false)
0-1-1		O  (false,false,true,false,true,false)
1-1-1		P  (true,true,true,false,true,false)
-10-1		Q  (true,false,false,false,true,false)
00-1		R  (false,false,false,false,true,false)
10-1		S  (true,true,false,false,true,false)
-11-1		T  (true,false,true,true,true,false)
01-1		U  (false,false,true,true,true,false)
11-1		V  (true,true,true,true,true,false)
-1-10		W  (true,false,true,false,false,false)
0-10		X  (false,false,true,false,false,false)
1-10		Y  (true,true,true,false,false,false)
-100		Z  (true,false,false,false,false,false)
*/

    pub fn to_bits(trytes :String, mut bits :Vec<bool>) {
        for c in trytes.chars() {
            match c {
                        '9' => bits.extend_from_slice(&[false,false,false,false,false,false]),
                        'A' => bits.extend_from_slice(&[true,true,false,false,false,false]),
                        'B' => bits.extend_from_slice(&[true,false,true,true,false,false]),
                        'C' => bits.extend_from_slice(&[false,false,true,true,false,false]),
                        'D' => bits.extend_from_slice(&[true,true,true,true,false,false]),
                        'E' => bits.extend_from_slice(&[true,false,true,false,true,true]),
                    	'F' => bits.extend_from_slice(&[false,false,true,false,true,true]),
                        'G' => bits.extend_from_slice(&[true,true,true,false,true,true]),
                        'H' => bits.extend_from_slice(&[true,false,false,false,true,true]),
                        'I' => bits.extend_from_slice(&[false,false,false,false,true,true]),
                        'J' => bits.extend_from_slice(&[true,true,false,false,true,true]),
                        'K' => bits.extend_from_slice(&[true,false,true,true,true,true]),
                        'L' => bits.extend_from_slice(&[false,false,true,true,true,true]),
                        'M' => bits.extend_from_slice(&[true,true,true,true,true,true]),
                        'N' => bits.extend_from_slice(&[true,false,true,false,true,false]),
                        'O' => bits.extend_from_slice(&[false,false,true,false,true,false]),
                        'P' => bits.extend_from_slice(&[true,true,true,false,true,false]),
                        'Q' => bits.extend_from_slice(&[true,false,false,false,true,false]),
                        'R' => bits.extend_from_slice(&[false,false,false,false,true,false]),
                        'S' => bits.extend_from_slice(&[true,true,false,false,true,false]),
                        'T' => bits.extend_from_slice(&[true,false,true,true,true,false]),
                    	'U' => bits.extend_from_slice(&[false,false,true,true,true,false]),
                        'V' => bits.extend_from_slice(&[true,true,true,true,true,false]),
                        'W' => bits.extend_from_slice(&[true,false,true,false,false,false]),
                        'X' => bits.extend_from_slice(&[false,false,true,false,false,false]),
                        'Y' => bits.extend_from_slice(&[true,true,true,false,false,false]),
                        'Z' => bits.extend_from_slice(&[true,false,false,false,false,false]),
                        _ => (),
            }
        }
    }

    pub fn to_2bits(trytes :String, mut bits :Vec<(bool,bool)>) {
        for c in trytes.chars() {
            match c {
                '9' => bits.extend_from_slice(&[(false,false),(false,false),(false,false)]),
                'A' => bits.extend_from_slice(&[(true,true),(false,false),(false,false)]),
                'B' => bits.extend_from_slice(&[(true,false),(true,true),(false,false)]),
                'C' => bits.extend_from_slice(&[(false,false),(true,true),(false,false)]),
                'D' => bits.extend_from_slice(&[(true,true),(true,true),(false,false)]),
                'E' => bits.extend_from_slice(&[(true,false),(true,false),(true,true)]),
                'F' => bits.extend_from_slice(&[(false,false),(true,false),(true,true)]),
                'G' => bits.extend_from_slice(&[(true,true),(true,false),(true,true)]),
                'H' => bits.extend_from_slice(&[(true,false),(false,false),(true,true)]),
                'I' => bits.extend_from_slice(&[(false,false),(false,false),(true,true)]),
                'J' => bits.extend_from_slice(&[(true,true),(false,false),(true,true)]),
                'K' => bits.extend_from_slice(&[(true,false),(true,true),(true,true)]),
                'L' => bits.extend_from_slice(&[(false,false),(true,true),(true,true)]),
                'M' => bits.extend_from_slice(&[(true,true),(true,true),(true,true)]),
                'N' => bits.extend_from_slice(&[(true,false),(true,false),(true,false)]),
                'O' => bits.extend_from_slice(&[(false,false),(true,false),(true,false)]),
                'P' => bits.extend_from_slice(&[(true,true),(true,false),(true,false)]),
                'Q' => bits.extend_from_slice(&[(true,false),(false,false),(true,false)]),
                'R' => bits.extend_from_slice(&[(false,false),(false,false),(true,false)]),
                'S' => bits.extend_from_slice(&[(true,true),(false,false),(true,false)]),
                'T' => bits.extend_from_slice(&[(true,false),(true,true),(true,false)]),
                'U' => bits.extend_from_slice(&[(false,false),(true,true),(true,false)]),
                'V' => bits.extend_from_slice(&[(true,true),(true,true),(true,false)]),
                'W' => bits.extend_from_slice(&[(true,false),(true,false),(false,false)]),
                'X' => bits.extend_from_slice(&[(false,false),(true,false),(false,false)]),
                'Y' => bits.extend_from_slice(&[(true,true),(true,false),(false,false)]),
                'Z' => bits.extend_from_slice(&[(true,false),(false,false),(false,false)]),
                _ => (),
            }
        }
    }


    pub fn to_i8(trytes :String, mut integers :Vec<i8>) -> Vec<i8> {
        for c in trytes.chars() {
            match c {
                '9' => integers.extend_from_slice(&[0,0,0]),
                'A' => integers.extend_from_slice(&[1,0,0]),
                'B' => integers.extend_from_slice(&[-1,1,0]),
                'C' => integers.extend_from_slice(&[0,1,0]),
                'D' => integers.extend_from_slice(&[1,1,0]),
                'E' => integers.extend_from_slice(&[-1,-1,1]),
                'F' => integers.extend_from_slice(&[0,-1,1]),
                'G' => integers.extend_from_slice(&[1,-1,1]),
                'H' => integers.extend_from_slice(&[-1,0,1]),
                'I' => integers.extend_from_slice(&[0,0,1]),
                'J' => integers.extend_from_slice(&[1,0,1]),
                'K' => integers.extend_from_slice(&[-1,1,1]),
                'L' => integers.extend_from_slice(&[0,1,1]),
                'M' => integers.extend_from_slice(&[1,1,1]),
                'N' => integers.extend_from_slice(&[-1,-1,-1]),
                'O' => integers.extend_from_slice(&[0,-1,-1]),
                'P' => integers.extend_from_slice(&[1,-1,-1]),
                'Q' => integers.extend_from_slice(&[-1,0,-1]),
                'R' => integers.extend_from_slice(&[0,0,-1]),
                'S' => integers.extend_from_slice(&[1,0,-1]),
                'T' => integers.extend_from_slice(&[-1,1,-1]),
                'U' => integers.extend_from_slice(&[0,1,-1]),
                'V' => integers.extend_from_slice(&[1,1,-1]),
                'W' => integers.extend_from_slice(&[-1,-1,0]),
                'X' => integers.extend_from_slice(&[0,-1,0]),
                'Y' => integers.extend_from_slice(&[1,-1,0]),
                'Z' => integers.extend_from_slice(&[-1,0,0]),
                _ => (),
            }
        };
        integers
    }

    pub fn from_i8(integers :[i8;243]) -> Result<TxHash, ()> {
        let mut arr :[bool;405] = [false;405];
        for i in 0..81 {
            match (integers[i*3],integers[i*3+1],integers[i*3+2]) {
                (0,0,0) => {arr[i*5]=false;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=false}, //3
                (1,0,0) => {arr[i*5]=true;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=false},  //2
                (-1,1,0) => {arr[i*5]=false;arr[i*5+1]=true;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=false},  //1
                (0,1,0) => {arr[i*5]=true;arr[i*5+1]=true;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=false},   //1
                (1,1,0) => {arr[i*5]=false;arr[i*5+1]=false;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=false},  //1
                (-1,-1,1) => {arr[i*5]=true;arr[i*5+1]=false;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=false},   //0
                (0,-1,1) => {arr[i*5]=false;arr[i*5+1]=true;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=false},   //0
                (1,-1,1) => {arr[i*5]=true;arr[i*5+1]=true;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=false},    //0
                (-1,0,1) => {arr[i*5]=false;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=true;arr[i*5+4]=false},  //0
                (0,0,1) => {arr[i*5]=true;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=true;arr[i*5+4]=false},   //0
                (1,0,1) => {arr[i*5]=false;arr[i*5+1]=true;arr[i*5+2]=false;arr[i*5+3]=true;arr[i*5+4]=false},   //0
                (-1,1,1) => {arr[i*5]=true;arr[i*5+1]=true;arr[i*5+2]=false;arr[i*5+3]=true;arr[i*5+4]=false},    //0
                (0,1,1) => {arr[i*5]=false;arr[i*5+1]=false;arr[i*5+2]=true;arr[i*5+3]=true;arr[i*5+4]=false},   //0
                (1,1,1) => {arr[i*5]=true;arr[i*5+1]=false;arr[i*5+2]=true;arr[i*5+3]=true;arr[i*5+4]=false},    //0
                (-1,-1,-1) => {arr[i*5]=false;arr[i*5+1]=true;arr[i*5+2]=true;arr[i*5+3]=true;arr[i*5+4]=false},    //0
                (0,-1,-1) => {arr[i*5]=true;arr[i*5+1]=true;arr[i*5+2]=true;arr[i*5+3]=true;arr[i*5+4]=false},     //0
                (1,-1,-1) => {arr[i*5]=false;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=true},  //0
                (-1,0,-1) => {arr[i*5]=true;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=true},   //0
                (0,0,-1) => {arr[i*5]=false;arr[i*5+1]=true;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=true},   //0
                (1,0,-1) => {arr[i*5]=true;arr[i*5+1]=true;arr[i*5+2]=false;arr[i*5+3]=false;arr[i*5+4]=true},    //0
                (-1,1,-1) => {arr[i*5]=false;arr[i*5+1]=false;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=true},   //0
                (0,1,-1) => {arr[i*5]=true;arr[i*5+1]=false;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=true},    //0
                (1,1,-1) => {arr[i*5]=false;arr[i*5+1]=true;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=true},    //0
                (-1,-1,0) => {arr[i*5]=true;arr[i*5+1]=true;arr[i*5+2]=true;arr[i*5+3]=false;arr[i*5+4]=true},     //1
                (0,-1,0) => {arr[i*5]=false;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=true;arr[i*5+4]=true},   //1
                (1,-1,0) => {arr[i*5]=true;arr[i*5+1]=false;arr[i*5+2]=false;arr[i*5+3]=true;arr[i*5+4]=true},    //1
                (-1,0,0) => {arr[i*5]=false;arr[i*5+1]=true;arr[i*5+2]=false;arr[i*5+3]=true;arr[i*5+4]=true},    //2
                _ => {error!("i={} integers[i*3]={},integers[i*3+1]={},integers[i*3+2]={}",i,integers[i*3],integers[i*3+1],integers[i*3+2]);return Err(())},
            }
        }
        Ok(TxHash{arr})
    }
}

impl PartialEq for TxHash {
    fn eq(&self, other: &TxHash) -> bool {
        for i in 0..405 {
            if self.arr[i]!=other.arr[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for TxHash {}

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


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime};

    #[test]
    fn curl_p81() {
        let start = SystemTime::now();
        let h3 = TxHash::compute("RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQIQQF9RCSJJCVZG9ZWITXNCSBBDHEEKDRBHVTWCZ9SZOOZHVBPCQNPKTWFNZAWGCZ9QDIMKRVINMIRZBPKRKQAIPGOHBTHTGYXTBJLSURDSPEOJ9UKJECUKCCPVIQQHDUYKVKISCEIEGVOQWRBAYXWGSJUTEVG9RPQLPTKYCRAJ9YNCUMDVDYDQCKRJOAPXCSUDAJGETALJINHEVNAARIPONBWXUOQUFGNOCUSSLYWKOZMZUKLNITZIFXFWQAYVJCVMDTRSHORGNSTKX9Z9DLWNHZSMNOYTU9AUCGYBVIITEPEKIXBCOFCMQPBGXYJKSHPXNUKFTXIJVYRFILAVXEWTUICZCYYPCEHNTK9SLGVL9RLAMYTAEPONCBHDXSEQZOXO9XCFUCPPMKEBR9IEJGQOPPILHFXHMIULJYXZJASQEGCQDVYFOM9ETXAGVMSCHHQLFPATWOSMZIDL9AHMSDCE9UENACG9OVFAEIPPQYBCLXDMXXA9UBJFQQBCYKETPNKHNOUKCSSYLWZDLKUARXNVKKKHNRBVSTVKQCZL9RY9BDTDTPUTFUBGRMSTOTXLWUHDMSGYRDSZLIPGQXIDMNCNBOAOI9WFUCXSRLJFIVTIPIAZUK9EDUJJ9B9YCJEZQQELLHVCWDNRH9FUXDGZRGOVXGOKORTCQQA9JXNROLETYCNLRMBGXBL9DQKMOAZCBJGWLNJLGRSTYBKLGFVRUF9QOPZVQFGMDJA9TBVGFJDBAHEVOLW9GNU9NICLCQJBOAJBAHHBZJGOFUCQMBGYQLCWNKSZPPBQMSJTJLM9GXOZHTNDLGIRCSIJAZTENQVQDHFSOQM9WVNWQQJNOPZMEISSCLOADMRNWALBBSLSWNCTOSNHNLWZBVCFIOGFPCPRKQSRGKFXGTWUSCPZSKQNLQJGKDLOXSBJMEHQPDZGSENUKWAHRNONDTBLHNAKGLOMCFYRCGMDOVANPFHMQRFCZIQHCGVORJJNYMTORDKPJPLA9LWAKAWXLIFEVLKHRKCDG9QPQCPGVKIVBENQJTJGZKFTNZHIMQISVBNLHAYSSVJKTIELGTETKPVRQXNAPWOBGQGFRMMK9UQDWJHSQMYQQTCBMVQKUVGJEAGTEQDN9TCRRAZHDPSPIYVNKPGJSJZASZQBM9WXEDWGAOQPPZFLAMZLEZGXPYSOJRWL9ZH9NOJTUKXNTCRRDO9GKULXBAVDRIZBOKJYVJUSHIX9F9O9ACYCAHUKBIEPVZWVJAJGSDQNZNWLIWVSKFJUMOYDMVUFLUXT9CEQEVRFBJVPCTJQCORM9JHLYFSMUVMFDXZFNCUFZZIKREIUIHUSHRPPOUKGFKWX9COXBAZMQBBFRFIBGEAVKBWKNTBMLPHLOUYOXPIQIZQWGOVUWQABTJT9ZZPNBABQFYRCQLXDHDEX9PULVTCQLWPTJLRSVZQEEYVBVY9KCNEZXQLEGADSTJBYOXEVGVTUFKNCNWMEDKDUMTKCMRPGKDCCBDHDVVSMPOPUBZOMZTXJSQNVVGXNPPBVSBL9WWXWQNMHRMQFEQYKWNCSW9URI9FYPT9UZMAFMMGUKFYTWPCQKVJ9DIHRJFMXRZUGI9TMTFUQHGXNBITDSORZORQIAMKY9VRYKLEHNRNFSEFBHF9KXIQAEZEJNQOENJVMWLMHI9GNZPXYUIFAJIVCLAGKUZIKTJKGNQVTXJORWIQDHUPBBPPYOUPFAABBVMMYATXERQHPECDVYGWDGXFJKOMOBXKRZD9MCQ9LGDGGGMYGUAFGMQTUHZOAPLKPNPCIKUNEMQIZOCM9COAOMZSJ9GVWZBZYXMCNALENZ9PRYMHENPWGKX9ULUIGJUJRKFJPBTTHCRZQKEAHT9DC9GSWQEGDTZFHACZMLFYDVOWZADBNMEM9XXEOMHCNJMDSUAJRQTBUWKJF9RZHK9ACGUNI9URFIHLXBXCEODONPXBSCWP9WNAEYNALKQHGULUQGAFL9LB9NBLLCACLQFGQMXRHGBTMI9YKAJKVELRWWKJAPKMSYMJTDYMZ9PJEEYIRXRMMFLRSFSHIXUL9NEJABLRUGHJFL9RASMSKOI9VCFRZ9GWTMODUUESIJBHWWHZYCLDENBFSJQPIOYC9MBGOOXSWEMLVU9L9WJXKZKVDBDMFSVHHISSSNILUMWULMVMESQUIHDGBDXROXGH9MTNFSLWJZRAPOKKRGXAAQBFPYPAAXLSTMNSNDTTJQSDQORNJS9BBGQ9KQJZYPAQ9JYQZJ9B9KQDAXUACZWRUNGMBOQLQZUHFNCKVQGORRZGAHES9PWJUKZWUJSBMNZFILBNBQQKLXITCTQDDBV9UDAOQOUPWMXTXWFWVMCXIXLRMRWMAYYQJPCEAAOFEOGZQMEDAGYGCTKUJBS9AGEXJAFHWWDZRYEN9DN9HVCMLFURISLYSWKXHJKXMHUWZXUQARMYPGKRKQMHVR9JEYXJRPNZINYNCGZHHUNHBAIJHLYZIZGGIDFWVNXZQADLEDJFTIUTQWCQSX9QNGUZXGXJYUUTFSZPQKXBA9DFRQRLTLUJENKESDGTZRGRSLTNYTITXRXRGVLWBTEWPJXZYLGHLQBAVYVOSABIVTQYQM9FIQKCBRRUEMVVTMERLWOK".to_string(), SpongeMode::CurlP81);
        let since_the_start = SystemTime::now().duration_since(start)
            .expect("Time went backwards");
        info!("Time:{:?}",since_the_start);
        assert_eq!(h3.unwrap().to_string(),"TIXEPIEYMGURTQ9ABVYVQSWMNGCVQFASMFAEQWUZCLIWLCDIGYVXOEJBBEMZOIHAYSUQMEFOGZBXUMHQW".to_string());
    }
}