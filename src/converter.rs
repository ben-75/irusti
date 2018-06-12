
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
        if(index_in_byte > 1){byte_index +=1;}
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
    if v>40 {
        t5 =1;
        v += -81;
    }
    if v<=-41 {
        t5 = -1;
        v += 81;
    }
    if v>13 {
        t4 =1;
        v += -27;
    }
    if v<=-14 {
        t4 = -1;
        v += 27;
    }
    if v>4 {
        t3 =1;
        v += -9;
    }
    if v<=-5 {
        t3 = -1;
        v += 9;
    }
    if v>1 {
        t2 =1;
        v += -3;
    }
    if v <= -2 {
        t2 = -1;
        v += 3;
    }
    if v==-2 {
        println!("v==-2 value={}",value);
    }
    [v,t2,t3,t4,t5]
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


pub fn to_trits(bytes :&Vec<i8>, tryte_count :usize) -> Vec<i8> {
    let mut trits_count = tryte_count*3;
    let mut response:Vec<i8> = Vec::with_capacity(trits_count);
    for byte_index in 0..bytes.len() {
        if trits_count == 0 {break;}
        let [t0,t1,t2,t3,t4] = i8_to_trits(bytes[byte_index]);
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

pub fn trytes_to_trites(trytes :String) -> Vec<i8> {
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
        assert_eq!(to_trits(&vec![1],1),vec![1,0,0]);
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