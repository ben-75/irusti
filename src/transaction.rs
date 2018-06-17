use converter::to_bytes;
use std::hash::{Hash, self};
use std::fmt;
use std::str::FromStr;
use converter::i8_to_trits;
use converter::i64_to_trits;
use converter::u64_to_trits;
use converter::tuple_2_char;
use converter::u64_from_bytes;
use converter::i64_from_bytes;
use txhash::TxHash;

const SIZE :usize = 1604;
const TAG_SIZE_IN_BYTES :usize = 17; // = ceil(81 TRITS / 5 TRITS_PER_BYTE)

const SUPPLY :i64 = 2779530283277761; // = (3^33 - 1) / 2

const SIG_MSG_BYTE_OFFSET :usize = 0;
const SIG_MSG_BIT_OFFSET :u8 = 0;
const SIG_MSG_BYTE_SIZE :usize = 1312;
const SIG_MSG_BYTE_OVERFLOW :u8 = 1;

const ADDRESS_BYTE_OFFSET :usize = SIG_MSG_BYTE_OFFSET+SIG_MSG_BYTE_SIZE;      //1312
const ADDRESS_BIT_OFFSET :u8 = (SIG_MSG_BIT_OFFSET+SIG_MSG_BYTE_OVERFLOW) % 5; //1
const ADDRESS_BYTE_SIZE :usize = 48;
const ADDRESS_BYTE_OVERFLOW :u8 = 3;

const VALUE_BYTE_OFFSET :usize = ADDRESS_BYTE_OFFSET+ADDRESS_BYTE_SIZE+((ADDRESS_BIT_OFFSET+ADDRESS_BYTE_OVERFLOW)/5) as usize;  //1328
const VALUE_BIT_OFFSET :u8 = (ADDRESS_BIT_OFFSET+ADDRESS_BYTE_OVERFLOW)%5; //4
const VALUE_BYTE_SIZE :usize = 16;
const VALUE_BYTE_OVERFLOW :u8 = 1;

const OBSOLETE_TAG_BYTE_OFFSET :usize = VALUE_BYTE_OFFSET+VALUE_BYTE_SIZE+((VALUE_BIT_OFFSET+VALUE_BYTE_OVERFLOW)/5) as usize; //1345
const OBSOLETE_TAG_BIT_OFFSET :u8 = (VALUE_BIT_OFFSET+VALUE_BYTE_OVERFLOW)%5; //0
const OBSOLETE_TAG_BYTE_SIZE :usize = 16;
const OBSOLETE_TAG_BYTE_OVERFLOW :u8 = 1;

const TIMESTAMP_BYTE_OFFSET :usize = OBSOLETE_TAG_BYTE_OFFSET+OBSOLETE_TAG_BYTE_SIZE+((OBSOLETE_TAG_BIT_OFFSET+OBSOLETE_TAG_BYTE_OVERFLOW)/5) as usize;
const TIMESTAMP_BIT_OFFSET :u8 = (OBSOLETE_TAG_BIT_OFFSET+OBSOLETE_TAG_BYTE_OVERFLOW)%5;
const TIMESTAMP_BYTE_SIZE :usize = 5;
const TIMESTAMP_BYTE_OVERFLOW :u8 = 2;

const CURRENT_INDEX_BYTE_OFFSET :usize = TIMESTAMP_BYTE_OFFSET+TIMESTAMP_BYTE_SIZE+((TIMESTAMP_BIT_OFFSET+TIMESTAMP_BYTE_OVERFLOW)/5) as usize;
const CURRENT_INDEX_BIT_OFFSET :u8 = (TIMESTAMP_BIT_OFFSET+TIMESTAMP_BYTE_OVERFLOW)%5;
const CURRENT_INDEX_BYTE_SIZE :usize = 5;
const CURRENT_INDEX_BYTE_OVERFLOW :u8 = 2;

const LAST_INDEX_BYTE_OFFSET :usize = CURRENT_INDEX_BYTE_OFFSET+CURRENT_INDEX_BYTE_SIZE+((CURRENT_INDEX_BIT_OFFSET+CURRENT_INDEX_BYTE_OVERFLOW)/5) as usize;
const LAST_INDEX_BIT_OFFSET :u8 = (CURRENT_INDEX_BIT_OFFSET+CURRENT_INDEX_BYTE_OVERFLOW)%5;
const LAST_INDEX_BYTE_SIZE :usize = 5;
const LAST_INDEX_BYTE_OVERFLOW :u8 = 2;

const BUNDLE_BYTE_OFFSET :usize = LAST_INDEX_BYTE_OFFSET+LAST_INDEX_BYTE_SIZE+((LAST_INDEX_BIT_OFFSET+LAST_INDEX_BYTE_OVERFLOW)/5) as usize;
const BUNDLE_BIT_OFFSET :u8 = (LAST_INDEX_BIT_OFFSET+LAST_INDEX_BYTE_OVERFLOW)%5;
const BUNDLE_BYTE_SIZE :usize = 48;
const BUNDLE_BYTE_OVERFLOW :u8 = 3;

const TRUNK_TX_BYTE_OFFSET :usize = BUNDLE_BYTE_OFFSET+BUNDLE_BYTE_SIZE+((BUNDLE_BIT_OFFSET+BUNDLE_BYTE_OVERFLOW)/5) as usize;
const TRUNK_TX_BIT_OFFSET :u8 = (BUNDLE_BIT_OFFSET+BUNDLE_BYTE_OVERFLOW)%5;
const TRUNK_TX_BYTE_SIZE :usize = 48;
const TRUNK_TX_BYTE_OVERFLOW :u8 = 3;

const BRANCH_TX_BYTE_OFFSET :usize = TRUNK_TX_BYTE_OFFSET+TRUNK_TX_BYTE_SIZE+((TRUNK_TX_BIT_OFFSET+TRUNK_TX_BYTE_OVERFLOW)/5) as usize;
const BRANCH_TX_BIT_OFFSET :u8 = (TRUNK_TX_BIT_OFFSET+TRUNK_TX_BYTE_OVERFLOW)%5;
const BRANCH_TX_BYTE_SIZE :usize = 48;
const BRANCH_TX_BYTE_OVERFLOW :u8 = 3;

const TAG_BYTE_OFFSET :usize = BRANCH_TX_BYTE_OFFSET+BRANCH_TX_BYTE_SIZE+((BRANCH_TX_BIT_OFFSET+BRANCH_TX_BYTE_OVERFLOW)/5) as usize;
const TAG_BIT_OFFSET :u8 = (BRANCH_TX_BIT_OFFSET+BRANCH_TX_BYTE_OVERFLOW)%5;
const TAG_BYTE_SIZE :usize = 16;
const TAG_BYTE_OVERFLOW :u8 = 1;

const ATTACHMENT_TS_BYTE_OFFSET :usize = TAG_BYTE_OFFSET+TAG_BYTE_SIZE+((TAG_BIT_OFFSET+TAG_BYTE_OVERFLOW)/5) as usize;
const ATTACHMENT_TS_BIT_OFFSET :u8 = (TAG_BIT_OFFSET+TAG_BYTE_OVERFLOW)%5;
const ATTACHMENT_TS_BYTE_SIZE :usize = 5;
const ATTACHMENT_TS_BYTE_OVERFLOW :u8 = 2;

const ATTACHMENT_TS_LOWER_BOUND_BYTE_OFFSET :usize = ATTACHMENT_TS_BYTE_OFFSET+ATTACHMENT_TS_BYTE_SIZE+((ATTACHMENT_TS_BIT_OFFSET+ATTACHMENT_TS_BYTE_OVERFLOW)/5) as usize;
const ATTACHMENT_TS_LOWER_BOUND_BIT_OFFSET :u8 = (ATTACHMENT_TS_BIT_OFFSET+ATTACHMENT_TS_BYTE_OVERFLOW)%5;
const ATTACHMENT_TS_LOWER_BOUND_BYTE_SIZE :usize = 5;
const ATTACHMENT_TS_LOWER_BOUND_BYTE_OVERFLOW :u8 = 2;

const ATTACHMENT_TS_UPPER_BOUND_BYTE_OFFSET :usize = ATTACHMENT_TS_LOWER_BOUND_BYTE_OFFSET+ATTACHMENT_TS_LOWER_BOUND_BYTE_SIZE+((ATTACHMENT_TS_LOWER_BOUND_BIT_OFFSET+ATTACHMENT_TS_LOWER_BOUND_BYTE_OVERFLOW)/5) as usize;
const ATTACHMENT_TS_UPPER_BOUND_BIT_OFFSET :u8 = (ATTACHMENT_TS_LOWER_BOUND_BIT_OFFSET+ATTACHMENT_TS_LOWER_BOUND_BYTE_OVERFLOW)%5;
const ATTACHMENT_TS_UPPER_BOUND_BYTE_SIZE :usize = 5;
const ATTACHMENT_TS_UPPER_BOUND_BYTE_OVERFLOW :u8 = 2;

const NONCE_BYTE_OFFSET :usize = ATTACHMENT_TS_UPPER_BOUND_BYTE_OFFSET+ATTACHMENT_TS_UPPER_BOUND_BYTE_SIZE+((ATTACHMENT_TS_UPPER_BOUND_BIT_OFFSET+ATTACHMENT_TS_UPPER_BOUND_BYTE_OVERFLOW)/5) as usize;
const NONCE_BIT_OFFSET :u8 = (ATTACHMENT_TS_UPPER_BOUND_BIT_OFFSET+ATTACHMENT_TS_UPPER_BOUND_BYTE_OVERFLOW)%5;
const NONCE_BYTE_SIZE :usize = 16;
const NONCE_BYTE_OVERFLOW :u8 = 1;

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

pub const TRINARY_SIZE                              :usize = NONCE_TRINARY_OFFSET + NONCE_TRINARY_SIZE;

const ESSENCE_TRINARY_OFFSET                    :usize = ADDRESS_TRINARY_OFFSET;
const ESSENCE_TRINARY_SIZE                      :usize = ADDRESS_TRINARY_SIZE + VALUE_TRINARY_SIZE + OBSOLETE_TAG_TRINARY_SIZE + TIMESTAMP_TRINARY_SIZE + CURRENT_INDEX_TRINARY_SIZE + LAST_INDEX_TRINARY_SIZE;

pub struct Transaction {
    arr :[i8;SIZE],
}

impl Transaction {

    pub fn new(signature_message_fragment :Option<&str>, address :Option<&str>, value :Option<i64>, obsolete_tag :Option<&str>,
               timestamp :Option<u64>, current_index :Option<u64>, last_index :Option<u64>, bundle :Option<&str>,
               trunk :Option<&str>,branch :Option<&str>,tag :Option<&str>,
               attachment_timestamp :Option<u64>,attachment_timestamp_lower :Option<u64>,attachment_timestamp_upper :Option<u64>,
               nonce :Option<&str>) -> Result<Transaction,String>{
        let mut arr = [0_i8;SIZE];
        match signature_message_fragment {
            None => (),
            Some(x) => {
                if x.len()>2187 {return Err("Message/Signature fragment is too long".to_string())}
                else { arr = match (register_trytes(x,0,0,&mut arr)) {
                                Err(_) => return Err("Message/Signature fragment contains invalid tryte".to_string()),
                                Ok(x) => x,
                            }
                }
            }
        };

        match address {
            None => (),
            Some(x) => {
                if x.len()>81 {return Err("Address is too long".to_string())}
                else { arr = match register_trytes(x,ADDRESS_BYTE_OFFSET,ADDRESS_BIT_OFFSET,&mut arr) {
                                Err(_) => return Err("Address contains invalid tryte".to_string()),
                                Ok(x) => x,
                            }
                }
            }
        };

        arr = match value {
            None => arr,
            Some(x) => register_i64(x,VALUE_BYTE_OFFSET,VALUE_BIT_OFFSET,&mut arr) ,
        };

        match obsolete_tag {
            None => (),
            Some(x) => {
                if x.len()>27 {return Err("Obsolete tag is too long".to_string())}
                    else { arr = match register_trytes(x,OBSOLETE_TAG_BYTE_OFFSET,OBSOLETE_TAG_BIT_OFFSET,&mut arr) {
                                        Err(_) => return Err("Obsolete tag contains invalid tryte".to_string()),
                                        Ok(x) => x,
                                }
                    }
            }
        };

        arr = match timestamp {
            None => arr,
            Some(x) => register_u64(x,TIMESTAMP_BYTE_OFFSET,TIMESTAMP_BIT_OFFSET,&mut arr) ,
        };

        arr = match current_index {
            None => arr,
            Some(x) => register_u64(x,CURRENT_INDEX_BYTE_OFFSET,CURRENT_INDEX_BIT_OFFSET,&mut arr) ,
        };

        arr = match last_index {
            None => arr,
            Some(x) => register_u64(x,LAST_INDEX_BYTE_OFFSET,LAST_INDEX_BIT_OFFSET,&mut arr) ,
        };

        match bundle {
            None => (),
            Some(x) => {
                if x.len()>81 {return Err("Bundle is too long".to_string())}
                    else { arr = match register_trytes(x,BUNDLE_BYTE_OFFSET,BUNDLE_BIT_OFFSET,&mut arr) {
                        Err(_) => return Err("Bundle contains invalid tryte".to_string()),
                        Ok(x) => x,
                    }
                    }
            }
        };

        match trunk {
            None => (),
            Some(x) => {
                if x.len()>81 {return Err("Trunk hash is too long".to_string())}
                else { arr = match register_trytes(x,TRUNK_TX_BYTE_OFFSET,TRUNK_TX_BIT_OFFSET,&mut arr) {
                        Err(_) => return Err("Trunk hash contains invalid tryte".to_string()),
                        Ok(x) => x,
                    }
                }
            }
        };

        match branch {
            None => (),
            Some(x) => {
                if x.len()>81 {return Err("Branch hash is too long".to_string())}
                else {
                    arr = match register_trytes(x,BRANCH_TX_BYTE_OFFSET,BRANCH_TX_BIT_OFFSET,&mut arr) {
                        Err(_) => return Err("Branch hash contains invalid tryte".to_string()),
                        Ok(x) => x,
                    }
                }
            }
        };

        match tag {
            None => (),
            Some(x) => {
                if x.len()>27 {return Err("Tag is too long".to_string())}
                    else {
                        arr = match register_trytes(x,TAG_BYTE_OFFSET,TAG_BIT_OFFSET,&mut arr) {
                            Err(_) => return Err("Tag contains invalid tryte".to_string()),
                            Ok(x) => x,
                        }
                    }
            }
        };

        arr = match attachment_timestamp {
            None => arr,
            Some(x) => register_u64(x,ATTACHMENT_TS_BYTE_OFFSET,ATTACHMENT_TS_BIT_OFFSET,&mut arr) ,
        };

        arr = match attachment_timestamp_lower {
            None => arr,
            Some(x) => register_u64(x,ATTACHMENT_TS_LOWER_BOUND_BYTE_OFFSET,ATTACHMENT_TS_LOWER_BOUND_BIT_OFFSET,&mut arr) ,
        };

        arr = match attachment_timestamp_upper {
            None => arr,
            Some(x) => register_u64(x,ATTACHMENT_TS_UPPER_BOUND_BYTE_OFFSET,ATTACHMENT_TS_UPPER_BOUND_BIT_OFFSET,&mut arr) ,
        };

        match nonce {
            None => (),
            Some(x) => {
                if x.len()>27 {return Err("Nonce is too long".to_string())}
                    else {
                        arr = match register_trytes(x,NONCE_BYTE_OFFSET,NONCE_BIT_OFFSET,&mut arr) {
                            Err(_) => return Err("Nonce contains invalid tryte".to_string()),
                            Ok(x) => x,
                        }
                    }
            }
        };

        Ok(Transaction{arr})
    }

    pub fn bytes(&self)-> &[i8;SIZE]{
        &self.arr
    }

    pub fn timestamp(&self)->u64{
        u64_from_bytes(&self.arr.to_vec(),40,TIMESTAMP_BYTE_OFFSET,TIMESTAMP_BIT_OFFSET)
    }
    pub fn attachment_timestamp(&self)->u64{
        u64_from_bytes(&self.arr.to_vec(),40,ATTACHMENT_TS_BYTE_OFFSET,ATTACHMENT_TS_BIT_OFFSET)
    }
    pub fn value(&self) -> i64 {
        i64_from_bytes(&self.arr.to_vec(),40,VALUE_BYTE_OFFSET,VALUE_BIT_OFFSET)
    }
    pub fn last_address_trit_is_zero(&self) -> bool {
        false
    }
    pub fn is_value_valid(&self) -> bool {
        let first_mandatory_zero_trit_index = VALUE_BYTE_OFFSET*5+VALUE_BIT_OFFSET as usize+VALUE_USABLE_TRINARY_SIZE;
        let last_mandatory_zero_trit_index = VALUE_BYTE_OFFSET*5+VALUE_BIT_OFFSET as usize+VALUE_TRINARY_SIZE;
        let byte_index = first_mandatory_zero_trit_index/5;
        let bit_offset = first_mandatory_zero_trit_index%5;
        let last_bit_offset = last_mandatory_zero_trit_index%5;
        let last_byte_index = 1377;//(VALUE_TRINARY_OFFSET + VALUE_TRINARY_SIZE)/5;
        for i in (1368_usize..1377_usize) {
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

fn register_i64(value :i64 , byte_idx: usize, idx_in_byte: u8, response: &mut [i8;SIZE])->[i8;SIZE] {
    let mut byte_index = byte_idx;
    let mut index_in_byte = idx_in_byte;
    let trits = i64_to_trits(value);
    for i in 0..13 {
        add_trits((trits[i*3],trits[i*3+1],trits[i*3+2]),byte_index,index_in_byte, response);
        if index_in_byte > 1 {byte_index +=1;}
        index_in_byte = (index_in_byte+3)%5;
    }
    add_trits((trits[39],0,0),byte_index,index_in_byte, response);
    *response
}

fn register_u64(value :u64 , byte_idx: usize, idx_in_byte: u8, response: &mut [i8;SIZE])->[i8;SIZE] {
    let mut byte_index = byte_idx;
    let mut index_in_byte = idx_in_byte;
    let trits = u64_to_trits(value as i64);
    for i in 0..13 {
        add_trits((trits[i*3],trits[i*3+1],trits[i*3+2]),byte_index,index_in_byte, response);
        if index_in_byte > 1 {byte_index +=1;}
        index_in_byte = (index_in_byte+3)%5;
    }
    add_trits((trits[39],0,0),byte_index,index_in_byte, response);
    *response
}

fn register_trytes(trytes: &str , byte_idx: usize, idx_in_byte: u8, response: &mut [i8;SIZE])->Result<[i8;SIZE], String>{
    let mut byte_index = byte_idx;
    let mut index_in_byte = idx_in_byte;
    for c in trytes.to_string().chars() {
        match c {
            '9' => add_trits((0,0,0),byte_index,index_in_byte, response),
            'A' => add_trits((1,0,0),byte_index,index_in_byte, response),
            'B' => add_trits((-1,1,0),byte_index,index_in_byte, response),
            'C' => add_trits((0,1,0),byte_index,index_in_byte, response),
            'D' => add_trits((1,1,0),byte_index,index_in_byte, response),
            'E' => add_trits((-1,-1,1),byte_index,index_in_byte, response),
            'F' => add_trits((0,-1,1),byte_index,index_in_byte, response),
            'G' => add_trits((1,-1,1),byte_index,index_in_byte, response),
            'H' => add_trits((-1,0,1),byte_index,index_in_byte, response),
            'I' => add_trits((0,0,1),byte_index,index_in_byte, response),
            'J' => add_trits((1,0,1),byte_index,index_in_byte, response),
            'K' => add_trits((-1,1,1),byte_index,index_in_byte, response),
            'L' => add_trits((0,1,1),byte_index,index_in_byte, response),
            'M' => add_trits((1,1,1),byte_index,index_in_byte, response),
            'N' => add_trits((-1,-1,-1),byte_index,index_in_byte, response),
            'O' => add_trits((0,-1,-1),byte_index,index_in_byte, response),
            'P' => add_trits((1,-1,-1),byte_index,index_in_byte, response),
            'Q' => add_trits((-1,0,-1),byte_index,index_in_byte, response),
            'R' => add_trits((0,0,-1),byte_index,index_in_byte, response),
            'S' => add_trits((1,0,-1),byte_index,index_in_byte, response),
            'T' => add_trits((-1,1,-1),byte_index,index_in_byte, response),
            'U' => add_trits((0,1,-1),byte_index,index_in_byte, response),
            'V' => add_trits((1,1,-1),byte_index,index_in_byte, response),
            'W' => add_trits((-1,-1,0),byte_index,index_in_byte, response),
            'X' => add_trits((0,-1,0),byte_index,index_in_byte, response),
            'Y' => add_trits((1,-1,0),byte_index,index_in_byte, response),
            'Z' => add_trits((-1,0,0),byte_index,index_in_byte, response),
            c => return Err(format!("Invalid tryte: {}",c)),

            //YGYQIVD
            //1,    -1,0,   1,  -1, 1,      1,      -1,     0,  -1,     0   -1,     0,0,    1,1,1,                      -1,1,1,0
            //1482522289
            //1     -3+0    +27 -81 +243    +729    -2187   +0  -19683  +0  -177147 +0+0    +4782969+14348907+43046721  -129140163 387420489 1162261467
        }
        if index_in_byte > 1 {byte_index +=1;}
        index_in_byte = (index_in_byte+3)%5;
    }
    Ok(*response)
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


fn add_trits((t0,t1,t2) :(i8,i8,i8),byte_index :usize, index_in_byte :u8, byte_array :&mut [i8;SIZE])->(){
    let factor = 3_i8.pow(index_in_byte as u32);
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
        let tx = Transaction::new(Some("A".as_ref()),Some("B".as_ref()), Some(1), None,
        None, None, None,None,None, None, None, None,
        None, None,None).unwrap();
        let mut array = [0_i8;SIZE];
        array[0] = 1_i8;
        array[ADDRESS_BYTE_OFFSET] = 6_i8;
        array[VALUE_BYTE_OFFSET] = 81_i8;
        let tx2 = Transaction{arr: array};
        assert_eq!(tx,tx2);
    }

    #[test]
    fn test_new2(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), None,
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        let ref_tx_trytes = "999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ999999999999999999999999999999999999999999999999999999YGYQIVD99999999999999999999TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";
        assert_eq!(tx.to_string(),ref_tx_trytes);
    }

    #[test]
    fn test_timestamp(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), None,
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        assert_eq!(tx.timestamp(),1482522289);
    }

    #[test]
    fn test_attachment_timestamp(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), None,
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, Some(1482522289),
                                  None, None,None).unwrap();

        assert_eq!(tx.attachment_timestamp(),1482522289);
    }

    #[test]
    fn test_value_1(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(1),
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        assert_eq!(tx.value(),1);
    }
    #[test]
    fn test_value_0(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), None,
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        assert_eq!(tx.value(),0);
    }
    #[test]
    fn test_value_minus_1(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(-1),
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        assert_eq!(tx.value(),-1);
    }
    #[test]
    fn test_value_minus_supply(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(-SUPPLY),
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        assert_eq!(tx.value(),-SUPPLY);
    }
    #[test]
    fn test_value_supply(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(SUPPLY),
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        assert_eq!(tx.value(),SUPPLY);
    }

    #[test]
    fn test_zero_value_is_valid_value(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), None,
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();

        assert_eq!(tx.is_value_valid(),true);
    }

    #[test]
    fn test_1000_value_is_valid_value(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(1000),
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        assert_eq!(tx.is_value_valid(),true);
    }

    #[test]
    fn test_minus_1000_value_is_valid_value(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(-1000),
                                  None,
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        assert_eq!(tx.is_value_valid(),true);
    }

    #[test]
    fn test_max_value_is_valid_value(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(SUPPLY),
                                  Some("MOBSOLETE"),
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        assert_eq!(tx.is_value_valid(),true);
    }

    #[test]
    fn test_min_value_is_valid_value(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(-SUPPLY),
                                  Some("MOBSOLETE"),
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        assert_eq!(tx.is_value_valid(),true);
    }

    #[test]
    fn test_min_value_minus_one_is_invalid_value(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(-SUPPLY-1),
                                  Some("MOBSOLETE"),
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        assert_eq!(tx.is_value_valid(),false);
    }

    #[test]
    fn test_max_value_plus_one_is_invalid_value(){
        let tx = Transaction::new(None,Some("A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ".as_ref()), Some(SUPPLY+1),
                                  Some("MOBSOLETE"),
                                  Some(1482522289), None, None,Some("TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK"),None, None, None, None,
                                  None, None,None).unwrap();
        assert_eq!(tx.is_value_valid(),false);
    }
}