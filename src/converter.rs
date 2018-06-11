
pub fn to_bytes(trytes: &str) -> Vec<i8>{
    let size_in_byte = if (trytes.len() * 3) % 5 >0 {((trytes.len()*3)/5)+1} else {(trytes.len()*3)/5};
    let mut response = Vec::with_capacity(size_in_byte);
    for i in 0..size_in_byte {
        response.push(0);
    }
    let mut index_in_byte = 0;
    let mut factor = 1;
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
            _ => (),
        }
        if(index_in_byte > 1){byte_index +=1;}
        index_in_byte = (index_in_byte+3)%5;
    }
    response

}

fn add_trits((t0,t1,t2) :(i8,i8,i8),byte_index :usize, index_in_byte :u32, byte_array :&mut Vec<i8>)->(){
    let mut factor = 3_i8.pow(index_in_byte);
    if(index_in_byte<=2) {
        byte_array[byte_index] += (t0 * factor + t1 * 3 * factor + t2 * 9 *factor);
    }
    if(index_in_byte==3) {
        byte_array[byte_index] += (t0 * factor + t1 * 3 * factor);
        byte_array[byte_index+1] = t2;
    }
    if(index_in_byte==4) {
        byte_array[byte_index] += (t0 * factor);
        byte_array[byte_index+1] = t1+ 3*t2;
    }
    ()
}

fn i8_to_trits(value :i8) -> [i8;5]{
    let mut v = value;
    let mut t5 :i8= 0;
    let mut t4 :i8 = 0;
    let mut t3 :i8= 0;
    let mut t2 :i8= 0;
    if(v>40){
        t5 =1;
        v += -81;
    }
    if(v<=-41){
        t5 = -1;
        v += 81;
    }
    if(v>14){
        t4 =1;
        v += -27;
    }
    if(v<=-15){
        t4 = -1;
        v += 27;
    }
    if(v>4){
        t3 =1;
        v += -9;
    }
    if(v<=-5){
        t3 = -1;
        v += 9;
    }
    if(v>1){
        t2 =1;
        v += -3;
    }
    if(v <= -2){
        t2 = -1;
        v += 3;
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
    let mut byte_index = 0;
    let mut response:String = "".to_string();
    let mut remaining_count = 0;
    let mut b0 :i8 = 0;
    let mut b1 :i8 = 0;
    for byte_index in 0..bytes.len() {
        if tryte_count == 0 {break;}
        let [t0,t1,t2,t3,t4] = i8_to_trits(bytes[byte_index]);
        match(remaining_count) {
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

fn tuple_2_char((t0,t1,t2) :(i8,i8,i8)) -> char {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_test(){
        assert_eq!(to_string(&to_bytes("A"),1),"A".to_string());
        assert_eq!(to_string(&to_bytes("9"),1),"9".to_string());
        assert_eq!(to_string(&to_bytes("ABC"),3),"ABC".to_string());
        assert_eq!(to_string(&to_bytes("AB"),2),"AB".to_string());
    }
    #[test]
    fn trailing_zeros_test(){
        assert_eq!(trailing_zeros(&to_bytes("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM")),0);
        assert_eq!(trailing_zeros(&to_bytes("999999999999999999999999999999999999999999999999999999999999999999999999999999999")),243);
        assert_eq!(trailing_zeros(&to_bytes("99999999999999999999999999999999999999999999999999999999999999999999999999999999A")),2);
        assert_eq!(trailing_zeros(&to_bytes("99999999999999999999999999999999999999999999999999999999999999999999999999999999B")),1);
        assert_eq!(trailing_zeros(&to_bytes("9999999999999999999999999999999999999999999999999999999999999999999999999999999Z9")),5);
        assert_eq!(trailing_zeros(&to_bytes("999999999999999999999999999999999999999999999999999999999999999999999999999999Z99")),8);
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
        assert_eq!(to_bytes("9"),[0]);
        assert_eq!(to_bytes("A"),[1]);
        assert_eq!(to_bytes("AA"),[28,0]);
        assert_eq!(to_bytes("99"),[0,0]);
        assert_eq!(to_bytes("M"),[13]);
        assert_eq!(to_bytes("Z"),[-1]);
        assert_eq!(to_bytes("MDIY9"),[121,108,-1]);
        let mut v =Vec::with_capacity(49);
        for _ in 0..48 {v.push(121)}
        v.push(13);
        assert_eq!(to_bytes("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"),v);
        let mut v =Vec::with_capacity(49);
        for _ in 0..49 {v.push(0)}
        assert_eq!(to_bytes("999999999999999999999999999999999999999999999999999999999999999999999999999999999"),v);
    }


}