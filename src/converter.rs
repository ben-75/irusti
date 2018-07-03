pub fn trits_from_string(trytes: &str) -> Result<Vec<i8>,&str>{
    let size_in_trits = trytes.len() * 3;
    let mut response = Vec::with_capacity(size_in_trits);
    for c in trytes.to_string().chars() {
        match c {
            '9' => response.extend_from_slice(&[0, 0, 0]),
            'A' => response.extend_from_slice(&[1, 0, 0]),
            'B' => response.extend_from_slice(&[-1, 1, 0]),
            'C' => response.extend_from_slice(&[0, 1, 0]),
            'D' => response.extend_from_slice(&[1, 1, 0]),
            'E' => response.extend_from_slice(&[-1, -1, 1]),
            'F' => response.extend_from_slice(&[0, -1, 1]),
            'G' => response.extend_from_slice(&[1, -1, 1]),
            'H' => response.extend_from_slice(&[-1, 0, 1]),
            'I' => response.extend_from_slice(&[0, 0, 1]),
            'J' => response.extend_from_slice(&[1, 0, 1]),
            'K' => response.extend_from_slice(&[-1, 1, 1]),
            'L' => response.extend_from_slice(&[0, 1, 1]),
            'M' => response.extend_from_slice(&[1, 1, 1]),
            'N' => response.extend_from_slice(&[-1, -1, -1]),
            'O' => response.extend_from_slice(&[0, -1, -1]),
            'P' => response.extend_from_slice(&[1, -1, -1]),
            'Q' => response.extend_from_slice(&[-1, 0, -1]),
            'R' => response.extend_from_slice(&[0, 0, -1]),
            'S' => response.extend_from_slice(&[1, 0, -1]),
            'T' => response.extend_from_slice(&[-1, 1, -1]),
            'U' => response.extend_from_slice(&[0, 1, -1]),
            'V' => response.extend_from_slice(&[1, 1, -1]),
            'W' => response.extend_from_slice(&[-1, -1, 0]),
            'X' => response.extend_from_slice(&[0, -1, 0]),
            'Y' => response.extend_from_slice(&[1, -1, 0]),
            'Z' => response.extend_from_slice(&[-1, 0, 0]),
            _ => return Err("Invalid tryte"),
        }
    }
    Ok(response)
}

pub fn trits_to_string(t: &[i8]) -> Option<String> {
    if t.len() % 3 != 0 {
        return None;
    }
    let mut s = String::with_capacity(t.len()/3);
    for i in 0..t.len()/3 {
        let k = i*3;
        s.push(tuple_2_char((t[k],t[k+1],t[k+2])));
    }
    Some(s)
}

pub fn to_bytes(trytes: &str) -> Result<Vec<i8>,&str>{
    let size_in_byte = if (trytes.len() * 3) % 5 >0 {((trytes.len()*3)/5)+1} else {(trytes.len()*3)/5};
    let mut response = Vec::with_capacity(size_in_byte);
    for _i in 0..size_in_byte {
        response.push(0);
    }
    let mut index_in_byte = 0;
    let mut byte_index = 0;
    for c in trytes.to_string().chars() {
        match c {
            '9' => add_trits((0,0,0),byte_index,index_in_byte,&mut response),
            'A' => add_trits((1,0,0),byte_index,index_in_byte,&mut response),
            'B' => add_trits((-1,1,0),byte_index,index_in_byte,&mut response),
            'C' => add_trits((0,1,0),byte_index,index_in_byte,&mut response),
            'D' => add_trits((1,1,0),byte_index,index_in_byte,&mut response),
            'E' => add_trits((-1,-1,1),byte_index,index_in_byte,&mut response),
            'F' => add_trits((0,-1,1),byte_index,index_in_byte,&mut response),
            'G' => add_trits((1,-1,1),byte_index,index_in_byte,&mut response),
            'H' => add_trits((-1,0,1),byte_index,index_in_byte,&mut response),
            'I' => add_trits((0,0,1),byte_index,index_in_byte,&mut response),
            'J' => add_trits((1,0,1),byte_index,index_in_byte,&mut response),
            'K' => add_trits((-1,1,1),byte_index,index_in_byte,&mut response),
            'L' => add_trits((0,1,1),byte_index,index_in_byte,&mut response),
            'M' => add_trits((1,1,1),byte_index,index_in_byte,&mut response),
            'N' => add_trits((-1,-1,-1),byte_index,index_in_byte,&mut response),
            'O' => add_trits((0,-1,-1),byte_index,index_in_byte,&mut response),
            'P' => add_trits((1,-1,-1),byte_index,index_in_byte,&mut response),
            'Q' => add_trits((-1,0,-1),byte_index,index_in_byte,&mut response),
            'R' => add_trits((0,0,-1),byte_index,index_in_byte,&mut response),
            'S' => add_trits((1,0,-1),byte_index,index_in_byte,&mut response),
            'T' => add_trits((-1,1,-1),byte_index,index_in_byte,&mut response),
            'U' => add_trits((0,1,-1),byte_index,index_in_byte,&mut response),
            'V' => add_trits((1,1,-1),byte_index,index_in_byte,&mut response),
            'W' => add_trits((-1,-1,0),byte_index,index_in_byte,&mut response),
            'X' => add_trits((0,-1,0),byte_index,index_in_byte,&mut response),
            'Y' => add_trits((1,-1,0),byte_index,index_in_byte,&mut response),
            'Z' => add_trits((-1,0,0),byte_index,index_in_byte,&mut response),
            _ => return Err("Invalid tryte"),
        }
        if index_in_byte > 1 {byte_index +=1;}
        index_in_byte = (index_in_byte+3)%5;
    }
    Ok(response)

}

fn add_trits((t0,t1,t2) :(i8,i8,i8),byte_index :usize, index_in_byte :u32, byte_array :&mut Vec<i8>)->(){
    let factor = 3_i8.pow(index_in_byte);
    if index_in_byte<=2 {
        byte_array[byte_index] += t0 * factor + t1 * 3 * factor + t2 * 9 *factor;
    }
    if index_in_byte==3 {
        byte_array[byte_index] += t0 * factor + t1 * 3 * factor;
        byte_array[byte_index+1] = t2;
    }
    if index_in_byte==4 {
        byte_array[byte_index] += t0 * factor;
        byte_array[byte_index+1] = t1+ 3*t2;
    }
    ()
}

pub fn i8_to_trits(value :i8) -> [i8;5]{
    let mut v = value;
    let mut t5 :i8= 0;
    let mut t4 :i8 = 0;
    let mut t3 :i8= 0;
    let mut t2 :i8= 0;
    if v > 40 {  //13+27
        t5 =1;
        v += -81;
    }
    if v < -40 {  //13+27+1
        t5 = -1;
        v += 81;
    }
    if v > 13 {
        t4 =1;
        v += -27;
    }
    if v < -13 {
        t4 = -1;
        v += 27;
    }
    if v > 4 {
        t3 =1;
        v += -9;
    }
    if v < -4 {
        t3 = -1;
        v += 9;
    }
    if v > 1 {
        t2 =1;
        v += -3;
    }
    if v < -1 {
        t2 = -1;
        v += 3;
    }
    if v==-2 {
        println!("v==-2 value={}",value);
    }
    [v,t2,t3,t4,t5]
}

pub fn u64_to_trits(value :i64) -> [i8;40]{
    i64_to_trits(value as i64)
}

pub fn u64_from_trits(trits :&[i8])->u64 {
    let mut response :i64=0;
    let mut factor :i64 = 1;
    for i in 0..39 {
        response += trits[i] as i64 *factor ;
        factor *=3;
    }
    response as u64
}
pub fn i64_from_trits(trits :&[i8])->i64 {
    let mut response :i64=0;
    let mut factor :i64 = 1;
    for i in 0..39 {
        response += trits[i] as i64 *factor;
        factor *=3;
    }
    response
}

pub fn u64_from_bytes(bytes :&[i8], byte_idx :usize, trit_offset :u8)->u64{
    let mut trits :[i8;40] = [0;40];
    u64_from_trits(trits_from_bytes(bytes,&mut trits,byte_idx,trit_offset))
}
pub fn i64_from_bytes(bytes :&[i8], byte_idx :usize, trit_offset :u8)->i64{
    let mut trits :[i8;40] = [0;40];
    i64_from_trits(trits_from_bytes(bytes,&mut trits,byte_idx,trit_offset))
}
pub fn i64_to_trits(value :i64) -> [i8;40]{
    let mut v = value;
    let mut r = [0_i8;40];

    if v > 2_026_277_576_509_488_133 { r[39] = 1; v += -4_052_555_153_018_976_267; }
    if v < -2_026_277_576_509_488_133 { r[39] = -1; v += 4_052_555_153_018_976_267; }
    if v > 675_425_858_836_496_044 { r[38] = 1; v += -1_350_851_717_672_992_089; }
    if v < -675_425_858_836_496_044 { r[38] = -1; v += 1_350_851_717_672_992_089; }
    if v > 225_141_952_945_498_681 { r[37] = 1; v += -450_283_905_890_997_363; }
    if v < -225_141_952_945_498_681 { r[37] = -1; v += 450_283_905_890_997_363; }
    if v > 75_047_317_648_499_560 { r[36] = 1; v += -150094635296999121; }
    if v < -75_047_317_648_499_560 { r[36] = -1; v += 150094635296999121; }
    if v > 25_015_772_549_499_853 { r[35] = 1; v += -50031545098999707; }
    if v < -25_015_772_549_499_853 { r[35] = -1; v += 50031545098999707; }
    if v > 8_338_590_849_833_284 { r[34] = 1; v += -16677181699666569; }
    if v < -8_338_590_849_833_284 { r[34] = -1; v += 16677181699666569; }
    if v > 2_779_530_283_277_761 { r[33] = 1; v += -5559060566555523; }
    if v < -2_779_530_283_277_761 { r[33] = -1; v += 5559060566555523; }
    if v > 926_510_094_425_920 { r[32] = 1; v += -1853020188851841; }
    if v < -926_510_094_425_920 { r[32] = -1; v += 1853020188851841; }
    if v > 308_836_698_141_973 { r[31] = 1; v += -617673396283947; }
    if v < -308_836_698_141_973 { r[31] = -1; v += 617673396283947; }
    if v > 102_945_566_047_324 { r[30] = 1; v += -205891132094649; }
    if v < -102_945_566_047_324 { r[30] = -1; v += 205891132094649; }
    if v > 34_315_188_682_441 { r[29] = 1; v += -68630377364883; }
    if v < -34_315_188_682_441 { r[29] = -1; v += 68630377364883; }
    if v > 11_438_396_227_480 { r[28] = 1; v += -22876792454961; }
    if v < -11_438_396_227_480 { r[28] = -1; v += 22876792454961; }
    if v > 3_812_798_742_493 { r[27] = 1; v += -7625597484987; }
    if v < -3_812_798_742_493 { r[27] = -1; v += 7625597484987; }
    if v > 1_270_932_914_164 { r[26] = 1; v += -2541865828329; }
    if v < -1_270_932_914_164 { r[26] = -1; v += 2541865828329; }
    if v > 423_644_304_721 { r[25] = 1; v += -847288609443; }
    if v < -423_644_304_721 { r[25] = -1; v += 847288609443; }
    if v > 141_214_768_240 { r[24] = 1; v += -282429536481; }
    if v < -141_214_768_240 { r[24] = -1; v += 282429536481; }
    if v > 47_071_589_413 { r[23] = 1; v += -94143178827; }
    if v < -47_071_589_413 { r[23] = -1; v += 94143178827; }
    if v > 15_690_529_804 { r[22] = 1; v += -31381059609; }
    if v < -15_690_529_804 { r[22] = -1; v += 31381059609; }
    if v > 5_230_176_601 { r[21] = 1; v += -10460353203; }
    if v < -5_230_176_601 { r[21] = -1; v += 10460353203; }
    if v > 1_743_392_200 { r[20] = 1; v += -3486784401; }
    if v < -1_743_392_200 { r[20] = -1; v += 3486784401; }
    if v > 581_130_733 { r[19] = 1; v += -1162261467; }
    if v < -581_130_733 { r[19] = -1; v += 1162261467; }
    if v > 193_710_244 { r[18] = 1; v += -387420489; }
    if v < -193_710_244 { r[18] = -1; v += 387420489; }
    if v > 64_570_081 { r[17] = 1; v += -129140163; }
    if v < -64_570_081 { r[17] = -1; v += 129140163; }
    if v > 21_523_360 { r[16] = 1; v += -43046721; }
    if v < -21_523_360 { r[16] = -1; v += 43046721; }
    if v > 7_174_453 { r[15] = 1; v += -14348907; }
    if v < -7_174_453 { r[15] = -1; v += 14348907; }
    if v > 2_391_484 { r[14] = 1; v += -4782969; }
    if v < -2_391_484 { r[14] = -1; v += 4782969; }
    if v > 797_161 { r[13] = 1; v += -1594323; }
    if v < -797_161 { r[13] = -1; v += 1594323; }
    if v > 265_720 { r[12] = 1; v += -531441; }
    if v < -265_720 { r[12] = -1; v += 531441; }
    if v > 88_573 { r[11] = 1; v += -177147; }
    if v < -88_573 { r[11] = -1; v += 177147; }
    if v > 29_524 { r[10] = 1; v += -59049; }
    if v < -29_524 { r[10] = -1; v += 59049; }
    if v > 9_841 { r[9] = 1; v += -19683; }
    if v < -9_841 { r[9] = -1; v += 19683; }
    if v > 3_280 { r[8] = 1; v += -6561; }
    if v < -3_280 { r[8] = -1; v += 6561; }
    if v > 1_093 { r[7] = 1; v += -2187; }
    if v < -1_093 { r[7] = -1; v += 2187; }
    if v > 364 { r[6] = 1; v += -729; }
    if v < -364 { r[6] = -1; v += 729; }
    if v > 121 { r[5] = 1; v += -243; }
    if v < -121 { r[5] = -1; v += 243; }
    if v > 40 { r[4] = 1; v += -81; }
    if v < -40 { r[4] = -1; v += 81; }
    if v > 13 { r[3] = 1; v += -27; }
    if v < -13 { r[3] = -1; v += 27; }
    if v > 4 {    r[2] =  1;  v += -9;  }
    if v < -4 {   r[2] = -1;  v +=  9;  }
    if v > 1 {    r[1] =  1;  v += -3;  }
    if v < -1 {   r[1] = -1;  v +=  3;  }
    r[0] = v as i8;
    r
}

pub fn trailing_zeros(bytes :&Vec<i8>) ->i32 {
    let i8_arr = i8_to_trits(bytes[48]);
    match  (i8_arr[0],i8_arr[1],i8_arr[2],i8_arr[3],i8_arr[4]) {
        (0,0,0,0,0) => 3+internal_trailing_zeros(bytes,47),
        (_,0,0,0,0) => 2,
        (_,_,0,0,0) => 1,
        _ => 0,
    }
}


pub fn internal_trailing_zeros(bytes :&Vec<i8>, index :usize) ->i32 {
    let i8_arr = i8_to_trits(bytes[index]);
    match  (i8_arr[0],i8_arr[1],i8_arr[2],i8_arr[3],i8_arr[4]) {
        (0,0,0,0,0) => if index>0 {5+internal_trailing_zeros(bytes,index-1)} else {5},
        (_,0,0,0,0) => 4,
        (_,_,0,0,0) => 3,
        (_,_,_,0,0) => 2,
        (_,_,_,_,0) => 1,
        _ => 0,
    }
}

pub fn get_trit(bytes :&Vec<i8>,byte_index :usize, bit_offset :u8)->i8 {
    let [t0,t1,t2,t3,t4] = i8_to_trits(bytes[byte_index]);
    match bit_offset {
        0 => t0,
        1 => t1,
        2 => t2,
        3 => t3,
        4 => t4,
        _ => panic!("Invalid bit_offset"),
    }
}

pub fn to_string(bytes :&Vec<i8>, mut tryte_count :i32) -> String {
    let mut response:String = "".to_string();
    let mut remaining_count = 0;
    let mut b0 :i8 = 0;
    let mut b1 :i8 = 0;
    for byte_index in 0..bytes.len() {
        if tryte_count == 0 {break;}
        let [t0,t1,t2,t3,t4] = i8_to_trits(bytes[byte_index]);
        match remaining_count {
            0 => {
                response.push(tuple_2_char((t0, t1, t2)));
                tryte_count -=1;
                //(b0,b1) =(t3,t4);
                b0 = t3;
                b1 = t4;
                remaining_count = 2;
            }
            1 => {
                response.push(tuple_2_char((b0, t0, t1)));
                tryte_count -=1;
                if tryte_count == 0 {break;}
                response.push(tuple_2_char((t2, t3, t4)));
                tryte_count -=1;
                remaining_count = 0;
            }
            2 => {
                response.push(tuple_2_char((b0, b1, t0)));
                tryte_count -=1;
                if tryte_count == 0 {break;}
                response.push(tuple_2_char((t1, t2, t3)));
                tryte_count -=1;
                b0 = t4;
                remaining_count = 1;
            }
            _ => panic!("cannot append. remainig count = {}",remaining_count),

        }
    }
    response
}

pub fn tuple_2_char((t0,t1,t2) :(i8,i8,i8)) -> char {
    match (t0,t1,t2) {
        (0,0,0) => '9',
        (1,0,0) => 'A',
        (-1,1,0) => 'B',
        (0,1,0) => 'C',
        (1,1,0) => 'D',
        (-1,-1,1) => 'E',
        (0,-1,1) => 'F',
        (1,-1,1) => 'G',
        (-1,0,1) => 'H',
        (0,0,1) => 'I',
        (1,0,1) => 'J',
        (-1,1,1) => 'K',
        (0,1,1) => 'L',
        (1,1,1) => 'M',
        (-1,-1,-1) => 'N',
        (0,-1,-1) => 'O',
        (1,-1,-1) => 'P',
        (-1,0,-1) => 'Q',
        (0,0,-1) => 'R',
        (1,0,-1) => 'S',
        (-1,1,-1) => 'T',
        (0,1,-1) => 'U',
        (1,1,-1) => 'V',
        (-1,-1,0) => 'W',
        (0,-1,0) => 'X',
        (1,-1,0) => 'Y',
        (-1,0,0) => 'Z',
        _ => panic!("...euh ({},{},{})",t0,t1,t2),
    }
}

pub  fn trits_from_bytes<'a>(bytes :&[i8], trits: & 'a mut[i8], byte_idx :usize, trit_offset :u8) -> &'a[i8]{
    let trits_count = trits.len();
    let mut response:Vec<i8> = Vec::with_capacity(trits_count);
    let mut trit_index = 0;
    let mut byte_index = byte_idx;
    let mut offset = trit_offset;
    while trit_index<trits_count {
        let [t0,t1,t2,t3,t4] = i8_to_trits(bytes[byte_index]);
        if offset==0 && (trit_index+5 <= trits_count) {
            response.extend_from_slice(&[t0,t1,t2,t3,t4]);
            byte_index +=1;
            trit_index +=5;
        }else{
            if offset>0 {
                if offset==1 {
                    response.extend_from_slice(&[t1,t2,t3,t4]);
                    byte_index +=1;
                    trit_index +=4;
                }
                if offset==2 {
                    response.extend_from_slice(&[t2,t3,t4]);
                    byte_index +=1;
                    trit_index +=3;
                }
                if offset==3 {
                    response.extend_from_slice(&[t3,t4]);
                    byte_index +=1;
                    trit_index +=2;
                }
                if offset==4 {
                    response.extend_from_slice(&[t4]);
                    byte_index +=1;
                    trit_index +=1;
                }
                offset=0;
            }else{
                match trits_count-trit_index {
                    4 => {
                        response.extend_from_slice(&[t0,t1,t2,t3]);
                    }
                    3 => {
                        response.extend_from_slice(&[t0,t1,t2]);
                    }
                    2 => {
                        response.extend_from_slice(&[t0,t1]);
                    }
                    1 => {
                        response.extend_from_slice(&[t0]);
                    }
                    _ => panic!("euh... trits_count-trit_index={}",trits_count-trit_index)
                }
                trit_index = trits_count;
            }
        }
    }
    trits.copy_from_slice(response.as_ref());
    trits

}

pub fn bytes_to_trits(bytes :&[i8], tryte_count :usize) -> Vec<i8> {
    let mut trits_count = tryte_count*3;
    let mut response:Vec<i8> = Vec::with_capacity(trits_count);
    for item in bytes {
        if trits_count == 0 {break;}
        let [t0,t1,t2,t3,t4] = i8_to_trits(*item);
        if trits_count>0 {
            response.push(t0);
            trits_count -=1;
        }
        if trits_count>0 {
            response.push(t1);
            trits_count -=1;
        }
        if trits_count>0 {
            response.push(t2);
            trits_count -=1;
        }
        if trits_count>0 {
            response.push(t3);
            trits_count -=1;
        }
        if trits_count>0 {
            response.push(t4);
            trits_count -=1;
        }
    }
    response
}

pub fn trits_to_bytes(trits :&Vec<i8>) -> (Vec<i8>, usize) {
    let trits_count = trits.len();
    let byte_count = if trits_count % 3 > 0 {trits_count/3 +1} else {trits_count/3};
    let mut response :Vec<i8> = Vec::with_capacity(byte_count);
    let mut trit_index = 0;
    while trit_index < trits_count {
        if trits_count-trit_index >= 5 {
            trit_index +=5;
            response.push(trit_to_i8((trits[trit_index],trits[trit_index+1],trits[trit_index+2],trits[trit_index+3],trits[trit_index+4])))
        }else{
            match trits_count-trit_index {
                4 => {
                    response.push(trit_to_i8((trits[trit_index],trits[trit_index+1],trits[trit_index+2],trits[trit_index+3],0)));
                },
                3 => {
                    response.push(trit_to_i8((trits[trit_index],trits[trit_index+1],trits[trit_index+2],0,0)));
                },
                2 => {
                    response.push(trit_to_i8((trits[trit_index],trits[trit_index+1],0,0,0)));
                },
                1 => {
                    response.push(trit_to_i8((trits[trit_index],0,0,0,0)));
                },
                _ => panic!("impossible")
            }
            trit_index = trits_count;
        }
    }
    (response,trits_count)
}

fn trit_to_i8((t0,t1,t2,t3,t4) :(i8,i8,i8,i8,i8))->i8{
    t0+t1*3+t2*9+t3*27+t4*81
}

pub fn trytes_to_trits(trytes :String) -> Vec<i8> {
    let sz = 3*trytes.len();
    let mut integers :Vec<i8> = Vec::with_capacity(sz);
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

pub fn hash_trits_to_bytes(trits :[i8;243]) -> [i8;49] {
    let mut response = [0;49];
    let mut trit_index = 0;
    let mut byte_index = 0;
    while trit_index < 243 {
        if 243-trit_index >= 5 {
            response[byte_index] = trit_to_i8((trits[trit_index],trits[trit_index+1],trits[trit_index+2],trits[trit_index+3],trits[trit_index+4]));
            trit_index +=5;
        }else{
            match 243-trit_index {
                4 => {
                    response[byte_index] = trit_to_i8((trits[trit_index],trits[trit_index+1],trits[trit_index+2],trits[trit_index+3],0));
                },
                3 => {
                    response[byte_index] = trit_to_i8((trits[trit_index],trits[trit_index+1],trits[trit_index+2],0,0));
                },
                2 => {
                    response[byte_index] = trit_to_i8((trits[trit_index],trits[trit_index+1],0,0,0));
                },
                1 => {
                    response[byte_index] = trit_to_i8((trits[trit_index],0,0,0,0));
                },
                _ => panic!("impossible")
            }
            trit_index = 243;
        }
        byte_index +=1;

    }
    response
}
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn to_trits_test() {
        assert_eq!(bytes_to_trits(&vec![1], 1), vec![1, 0, 0]);
    }

        #[test]
    fn to_string_test(){
        assert_eq!(to_string(&to_bytes("A").unwrap(),1),"A".to_string());
        assert_eq!(to_string(&to_bytes("9").unwrap(),1),"9".to_string());
        assert_eq!(to_string(&to_bytes("ABC").unwrap(),3),"ABC".to_string());
        assert_eq!(to_string(&to_bytes("AB").unwrap(),2),"AB".to_string());
    }
    #[test]
    fn trailing_zeros_test(){
        assert_eq!(trailing_zeros(&to_bytes("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM").unwrap()),0);
        assert_eq!(trailing_zeros(&to_bytes("999999999999999999999999999999999999999999999999999999999999999999999999999999999").unwrap()),243);
        assert_eq!(trailing_zeros(&to_bytes("99999999999999999999999999999999999999999999999999999999999999999999999999999999A").unwrap()),2);
        assert_eq!(trailing_zeros(&to_bytes("99999999999999999999999999999999999999999999999999999999999999999999999999999999B").unwrap()),1);
        assert_eq!(trailing_zeros(&to_bytes("9999999999999999999999999999999999999999999999999999999999999999999999999999999Z9").unwrap()),5);
        assert_eq!(trailing_zeros(&to_bytes("999999999999999999999999999999999999999999999999999999999999999999999999999999Z99").unwrap()),8);
    }

    #[test]
    fn i8_to_trits_test() {
        assert_eq!(i8_to_trits(1),[1,0,0,0,0]);
        assert_eq!(i8_to_trits(4),[1,1,0,0,0]);
        assert_eq!(i8_to_trits(2),[-1,1,0,0,0]);
        assert_eq!(i8_to_trits(121),[1,1,1,1,1]);
        assert_eq!(i8_to_trits(-121),[-1,-1,-1,-1,-1]);
        assert_eq!(i8_to_trits(-40),[-1,-1,-1,-1,0]);
        assert_eq!(i8_to_trits(41),[-1,-1,-1,-1,1]);
        assert_eq!(i8_to_trits(0),[0,0,0,0,0]);
    }

    #[test]
    fn i64_to_trits_test() {
        let v :[i8;40] = [1,-1,0,1,-1,1,1,-1,0,-1,0,-1,0,0,1,1,1,-1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        let c =i64_to_trits(1482522289);
        for i in 0..40 {
            assert_eq!(c[i],v[i]);
        }
        let true_trits = trytes_to_trits("YGYQIVD".to_string());
        for i in 0..21 {
            assert_eq!(true_trits[i],v[i]);
        }
        assert_eq!(true_trits.len(),21);
    }

    #[test]
    fn to_bytes_test() {
        assert_eq!(to_bytes("9").unwrap(),[0]);
        assert_eq!(to_bytes("A").unwrap(),[1]);
        assert_eq!(to_bytes("AA").unwrap(),[28,0]);
        assert_eq!(to_bytes("99").unwrap(),[0,0]);
        assert_eq!(to_bytes("M").unwrap(),[13]);
        assert_eq!(to_bytes("Z").unwrap(),[-1]);
        assert_eq!(to_bytes("MDIY9").unwrap(),[121,108,-1]);
        let mut v =Vec::with_capacity(49);
        for _ in 0..48 {v.push(121)}
        v.push(13);
        assert_eq!(to_bytes("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM").unwrap(),v);
        let mut v =Vec::with_capacity(49);
        for _ in 0..49 {v.push(0)}
        assert_eq!(to_bytes("999999999999999999999999999999999999999999999999999999999999999999999999999999999").unwrap(),v);
    }


}