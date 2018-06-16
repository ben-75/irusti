use converter::to_bytes;
use std::hash::{Hash, self};
use std::fmt;
use std::str::FromStr;
use converter::i8_to_trits;
use converter::tuple_2_char;

const SIZE :usize = 1604;
const TAG_SIZE_IN_BYTES :usize = 17; // = ceil(81 TRITS / 5 TRITS_PER_BYTE)

const SUPPLY :i64 = 2779530283277761; // = (3^33 - 1) / 2

const SIGNATURE_MESSAGE_FRAGMENT_TRINARY_OFFSET :usize = 0;
const SIGNATURE_MESSAGE_FRAGMENT_TRINARY_SIZE   :usize = 6561;
const ADDRESS_TRINARY_OFFSET                    :usize = SIGNATURE_MESSAGE_FRAGMENT_TRINARY_OFFSET + SIGNATURE_MESSAGE_FRAGMENT_TRINARY_SIZE;
const ADDRESS_TRINARY_SIZE                      :usize = 243;
const VALUE_TRINARY_OFFSET                      :usize = ADDRESS_TRINARY_OFFSET + ADDRESS_TRINARY_SIZE;
const VALUE_TRINARY_SIZE                        :usize = 81;
const VALUE_USABLE_TRINARY_SIZE                 :usize = 33;
const OBSOLETE_TAG_TRINARY_OFFSET               :usize = VALUE_TRINARY_OFFSET + VALUE_TRINARY_SIZE;
const OBSOLETE_TAG_TRINARY_SIZE                 :usize = 81;
const TIMESTAMP_TRINARY_OFFSET                  :usize = OBSOLETE_TAG_TRINARY_OFFSET + OBSOLETE_TAG_TRINARY_SIZE;
const TIMESTAMP_TRINARY_SIZE                    :usize = 27;
const CURRENT_INDEX_TRINARY_OFFSET              :usize = TIMESTAMP_TRINARY_OFFSET + TIMESTAMP_TRINARY_SIZE;
const CURRENT_INDEX_TRINARY_SIZE                :usize = 27;
const LAST_INDEX_TRINARY_OFFSET                 :usize = CURRENT_INDEX_TRINARY_OFFSET + CURRENT_INDEX_TRINARY_SIZE;
const LAST_INDEX_TRINARY_SIZE                   :usize = 27;
const BUNDLE_TRINARY_OFFSET                     :usize = LAST_INDEX_TRINARY_OFFSET + LAST_INDEX_TRINARY_SIZE;
const BUNDLE_TRINARY_SIZE                       :usize = 243;
const TRUNK_TRANSACTION_TRINARY_OFFSET          :usize = BUNDLE_TRINARY_OFFSET + BUNDLE_TRINARY_SIZE;
const TRUNK_TRANSACTION_TRINARY_SIZE            :usize = 243;
const BRANCH_TRANSACTION_TRINARY_OFFSET         :usize = TRUNK_TRANSACTION_TRINARY_OFFSET + TRUNK_TRANSACTION_TRINARY_SIZE;
const BRANCH_TRANSACTION_TRINARY_SIZE           :usize = 243;

const TAG_TRINARY_OFFSET                        :usize = BRANCH_TRANSACTION_TRINARY_OFFSET + BRANCH_TRANSACTION_TRINARY_SIZE;
const TAG_TRINARY_SIZE                          :usize = 81;
const ATTACHMENT_TIMESTAMP_TRINARY_OFFSET       :usize = TAG_TRINARY_OFFSET + TAG_TRINARY_SIZE;
const ATTACHMENT_TIMESTAMP_TRINARY_SIZE         :usize = 27;
const ATTACHMENT_TIMESTAMP_LOWER_BOUND_TRINARY_OFFSET :usize = ATTACHMENT_TIMESTAMP_TRINARY_OFFSET + ATTACHMENT_TIMESTAMP_TRINARY_SIZE;
const ATTACHMENT_TIMESTAMP_LOWER_BOUND_TRINARY_SIZE   :usize = 27;
const ATTACHMENT_TIMESTAMP_UPPER_BOUND_TRINARY_OFFSET :usize = ATTACHMENT_TIMESTAMP_LOWER_BOUND_TRINARY_OFFSET + ATTACHMENT_TIMESTAMP_LOWER_BOUND_TRINARY_SIZE;
const ATTACHMENT_TIMESTAMP_UPPER_BOUND_TRINARY_SIZE   :usize = 27;
const NONCE_TRINARY_OFFSET                      :usize = ATTACHMENT_TIMESTAMP_UPPER_BOUND_TRINARY_OFFSET + ATTACHMENT_TIMESTAMP_UPPER_BOUND_TRINARY_SIZE;
const NONCE_TRINARY_SIZE                        :usize = 81;

const TRINARY_SIZE                              :usize = NONCE_TRINARY_OFFSET + NONCE_TRINARY_SIZE;

const ESSENCE_TRINARY_OFFSET                    :usize = ADDRESS_TRINARY_OFFSET;
const ESSENCE_TRINARY_SIZE                      :usize = ADDRESS_TRINARY_SIZE + VALUE_TRINARY_SIZE + OBSOLETE_TAG_TRINARY_SIZE + TIMESTAMP_TRINARY_SIZE + CURRENT_INDEX_TRINARY_SIZE + LAST_INDEX_TRINARY_SIZE;

struct Transaction {
    arr :[i8;SIZE],
}

impl Transaction {

    pub fn new(signature_message_fragment :&str) -> Result<Transaction,String>{
        let mut arr = [0_i8;SIZE];
        let msg = to_bytes(signature_message_fragment);
        match msg {
            Err(x) => return Err(x.to_string()),
            Ok(x) => {
                arr[0..x.len()].copy_from_slice(x.as_ref());
            }
        }
        Ok(Transaction{arr})
    }
    pub fn is_value_valid(&self) -> bool {
        let first_byte_index = 1367;//(VALUE_TRINARY_OFFSET + VALUE_USABLE_TRINARY_SIZE)/5;
        let last_byte_index = 1377;//(VALUE_TRINARY_OFFSET + VALUE_TRINARY_SIZE)/5;
        for i in (1367_usize..1377_usize).rev() {
            if self.arr[i]!=0 {return false;}
        }
        self.arr[1367]<5 && self.arr[1367]>-5
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Transaction) -> bool {
        for i in 0..SIZE {
            if self.arr[i]!=other.arr[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Transaction {}

impl fmt::Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction-Trytes: {}", self.to_string())
    }
}

impl ToString for Transaction{

    fn to_string(&self) -> String {
        let mut tryte_count = TRINARY_SIZE;
        let mut response:String = "".to_string();
        let mut remaining_count = 0;
        let mut b0 :i8 = 0;
        let mut b1 :i8 = 0;
        for byte_index in 0..SIZE {
            if tryte_count == 0 {break;}
            let [t0,t1,t2,t3,t4] = i8_to_trits(self.arr[byte_index]);
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
}

impl FromStr for Transaction {
    type Err = ();

    fn from_str(trytes: &str) -> Result<Transaction, ()> {
        let mut response: [i8;SIZE] = [0;SIZE];

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
                _ => return Err(()),
            }
            if index_in_byte > 1 {byte_index +=1;}
            index_in_byte = (index_in_byte+3)%5;
        }
        Ok(Transaction{arr:response})
    }

}


fn add_trits((t0,t1,t2) :(i8,i8,i8),byte_index :usize, index_in_byte :u32, byte_array :&mut [i8;SIZE])->(){
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let tx = Transaction::new("A".as_ref()).unwrap();
        let mut array = [0_i8;SIZE];
        array[0] = 1_i8;
        let tx2 = Transaction{arr: array};
        assert_eq!(tx,tx2);
    }
}