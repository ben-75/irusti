use converter::to_bytes;

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