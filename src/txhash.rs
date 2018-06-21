use std::str::FromStr;
use sponge::curl::SpongeMode;
use sponge::curl::Curl;
use sponge::curl::Sponge;
use sponge::curl::HASH_LENGTH;
use std::hash::{Hash, self};
use std::fmt;
use converter::i8_to_trits;
use converter::hash_trits_to_bytes;
use converter::tuple_2_char;
use converter::trytes_to_trits;
use converter::bytes_to_trits;

const SIZE_IN_TRITS :usize = 243;
const SIZE_IN_BYTES :usize = 49;
const NULL_HASH :TxHash = TxHash{arr:[0_i8;49]};

#[derive(Copy,Clone)]
pub struct TxHash{
    arr :[i8;SIZE_IN_BYTES],
}

impl TxHash {

    pub fn new(trytes :&str) -> TxHash {
        TxHash::from_str(trytes).unwrap()
    }

    pub fn as_u8_array(&self) -> &[u8;SIZE_IN_BYTES] {
        unsafe { &*(&self.arr as *const _  as *const [u8;SIZE_IN_BYTES]) }
    }

    pub fn trailing_zeros(&self) -> i32 {
        let i8_arr = i8_to_trits(self.arr[48]);
        match  (i8_arr[0],i8_arr[1],i8_arr[2],i8_arr[3],i8_arr[4]) {
            (0,0,0,0,0) => 3+self.internal_trailing_zeros(47),
            (_,0,0,0,0) => 2,
            (_,_,0,0,0) => 1,
            _ => 0,
        }
    }

    fn internal_trailing_zeros(&self, index :usize) ->i32 {
        let i8_arr = i8_to_trits(self.arr[index]);
        match  (i8_arr[0],i8_arr[1],i8_arr[2],i8_arr[3],i8_arr[4]) {
            (0,0,0,0,0) => if index>0 {5+self.internal_trailing_zeros(index-1)} else {5},
            (_,0,0,0,0) => 4,
            (_,_,0,0,0) => 3,
            (_,_,_,0,0) => 2,
            (_,_,_,_,0) => 1,
            _ => 0,
        }
    }

    pub fn compute_from_trytes(trytes : String, mode :SpongeMode) -> Result<TxHash,()> {
        let sz = 3*trytes.len();
        let integers = trytes_to_trits(trytes);

        let mut curl = match mode {
            SpongeMode::CurlP27 => {Curl::new_curl_p27()}
            _ => Curl::new_curl_p81(),
        };
        curl.reset();
        curl.absorb(integers);

        let mut out :[i8;HASH_LENGTH] = [0;HASH_LENGTH];
        curl.squeeze(&mut out);
        Ok(TxHash {
            arr: hash_trits_to_bytes(out),
        })
    }

    pub fn compute_from_bytes(bytes : &[i8], trit_count :usize, mode :SpongeMode) -> Result<TxHash,()> {
        let sz = trit_count;
        let integers = bytes_to_trits(&bytes.to_vec(), trit_count);

        let mut curl = match mode {
            SpongeMode::CurlP27 => {Curl::new_curl_p27()}
            _ => Curl::new_curl_p81(),
        };
        curl.reset();
        curl.absorb(integers);
        let mut out :[i8;HASH_LENGTH] = [0;HASH_LENGTH];
        curl.squeeze(&mut out);
        Ok(TxHash {
            arr: hash_trits_to_bytes(out),
        })
    }

    pub fn is_null_hash(&self) -> bool{
        self.eq(&NULL_HASH)
    }

}

impl fmt::Debug for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trytes: {}", self.to_string())
    }
}

impl PartialEq for TxHash {
    fn eq(&self, other: &TxHash) -> bool {
        for i in 0..SIZE_IN_BYTES {
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
        let mut tryte_count = SIZE_IN_TRITS;
        let mut response:String = "".to_string();
        let mut remaining_count = 0;
        let mut b0 :i8 = 0;
        let mut b1 :i8 = 0;
        for byte_index in 0..49 {
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

impl FromStr for TxHash {
    type Err = ();

    fn from_str(trytes: &str) -> Result<TxHash, ()> {
        let mut response: [i8;49] = [0;49];

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
        Ok(TxHash{arr:response})
    }

}


fn add_trits((t0,t1,t2) :(i8,i8,i8),byte_index :usize, index_in_byte :u32, byte_array :&mut [i8;49])->(){
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

impl Hash for TxHash {
    fn hash<S: hash::Hasher>(&self, state: &mut S) {
        for i in 0..49 {
            state.write_i8(self.arr[i]);
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Instant};

    #[test]
    fn curl_p81() {
        let now = Instant::now();
        let h3 = TxHash::compute_from_trytes("RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQIQQF9RCSJJCVZG9ZWITXNCSBBDHEEKDRBHVTWCZ9SZOOZHVBPCQNPKTWFNZAWGCZ9QDIMKRVINMIRZBPKRKQAIPGOHBTHTGYXTBJLSURDSPEOJ9UKJECUKCCPVIQQHDUYKVKISCEIEGVOQWRBAYXWGSJUTEVG9RPQLPTKYCRAJ9YNCUMDVDYDQCKRJOAPXCSUDAJGETALJINHEVNAARIPONBWXUOQUFGNOCUSSLYWKOZMZUKLNITZIFXFWQAYVJCVMDTRSHORGNSTKX9Z9DLWNHZSMNOYTU9AUCGYBVIITEPEKIXBCOFCMQPBGXYJKSHPXNUKFTXIJVYRFILAVXEWTUICZCYYPCEHNTK9SLGVL9RLAMYTAEPONCBHDXSEQZOXO9XCFUCPPMKEBR9IEJGQOPPILHFXHMIULJYXZJASQEGCQDVYFOM9ETXAGVMSCHHQLFPATWOSMZIDL9AHMSDCE9UENACG9OVFAEIPPQYBCLXDMXXA9UBJFQQBCYKETPNKHNOUKCSSYLWZDLKUARXNVKKKHNRBVSTVKQCZL9RY9BDTDTPUTFUBGRMSTOTXLWUHDMSGYRDSZLIPGQXIDMNCNBOAOI9WFUCXSRLJFIVTIPIAZUK9EDUJJ9B9YCJEZQQELLHVCWDNRH9FUXDGZRGOVXGOKORTCQQA9JXNROLETYCNLRMBGXBL9DQKMOAZCBJGWLNJLGRSTYBKLGFVRUF9QOPZVQFGMDJA9TBVGFJDBAHEVOLW9GNU9NICLCQJBOAJBAHHBZJGOFUCQMBGYQLCWNKSZPPBQMSJTJLM9GXOZHTNDLGIRCSIJAZTENQVQDHFSOQM9WVNWQQJNOPZMEISSCLOADMRNWALBBSLSWNCTOSNHNLWZBVCFIOGFPCPRKQSRGKFXGTWUSCPZSKQNLQJGKDLOXSBJMEHQPDZGSENUKWAHRNONDTBLHNAKGLOMCFYRCGMDOVANPFHMQRFCZIQHCGVORJJNYMTORDKPJPLA9LWAKAWXLIFEVLKHRKCDG9QPQCPGVKIVBENQJTJGZKFTNZHIMQISVBNLHAYSSVJKTIELGTETKPVRQXNAPWOBGQGFRMMK9UQDWJHSQMYQQTCBMVQKUVGJEAGTEQDN9TCRRAZHDPSPIYVNKPGJSJZASZQBM9WXEDWGAOQPPZFLAMZLEZGXPYSOJRWL9ZH9NOJTUKXNTCRRDO9GKULXBAVDRIZBOKJYVJUSHIX9F9O9ACYCAHUKBIEPVZWVJAJGSDQNZNWLIWVSKFJUMOYDMVUFLUXT9CEQEVRFBJVPCTJQCORM9JHLYFSMUVMFDXZFNCUFZZIKREIUIHUSHRPPOUKGFKWX9COXBAZMQBBFRFIBGEAVKBWKNTBMLPHLOUYOXPIQIZQWGOVUWQABTJT9ZZPNBABQFYRCQLXDHDEX9PULVTCQLWPTJLRSVZQEEYVBVY9KCNEZXQLEGADSTJBYOXEVGVTUFKNCNWMEDKDUMTKCMRPGKDCCBDHDVVSMPOPUBZOMZTXJSQNVVGXNPPBVSBL9WWXWQNMHRMQFEQYKWNCSW9URI9FYPT9UZMAFMMGUKFYTWPCQKVJ9DIHRJFMXRZUGI9TMTFUQHGXNBITDSORZORQIAMKY9VRYKLEHNRNFSEFBHF9KXIQAEZEJNQOENJVMWLMHI9GNZPXYUIFAJIVCLAGKUZIKTJKGNQVTXJORWIQDHUPBBPPYOUPFAABBVMMYATXERQHPECDVYGWDGXFJKOMOBXKRZD9MCQ9LGDGGGMYGUAFGMQTUHZOAPLKPNPCIKUNEMQIZOCM9COAOMZSJ9GVWZBZYXMCNALENZ9PRYMHENPWGKX9ULUIGJUJRKFJPBTTHCRZQKEAHT9DC9GSWQEGDTZFHACZMLFYDVOWZADBNMEM9XXEOMHCNJMDSUAJRQTBUWKJF9RZHK9ACGUNI9URFIHLXBXCEODONPXBSCWP9WNAEYNALKQHGULUQGAFL9LB9NBLLCACLQFGQMXRHGBTMI9YKAJKVELRWWKJAPKMSYMJTDYMZ9PJEEYIRXRMMFLRSFSHIXUL9NEJABLRUGHJFL9RASMSKOI9VCFRZ9GWTMODUUESIJBHWWHZYCLDENBFSJQPIOYC9MBGOOXSWEMLVU9L9WJXKZKVDBDMFSVHHISSSNILUMWULMVMESQUIHDGBDXROXGH9MTNFSLWJZRAPOKKRGXAAQBFPYPAAXLSTMNSNDTTJQSDQORNJS9BBGQ9KQJZYPAQ9JYQZJ9B9KQDAXUACZWRUNGMBOQLQZUHFNCKVQGORRZGAHES9PWJUKZWUJSBMNZFILBNBQQKLXITCTQDDBV9UDAOQOUPWMXTXWFWVMCXIXLRMRWMAYYQJPCEAAOFEOGZQMEDAGYGCTKUJBS9AGEXJAFHWWDZRYEN9DN9HVCMLFURISLYSWKXHJKXMHUWZXUQARMYPGKRKQMHVR9JEYXJRPNZINYNCGZHHUNHBAIJHLYZIZGGIDFWVNXZQADLEDJFTIUTQWCQSX9QNGUZXGXJYUUTFSZPQKXBA9DFRQRLTLUJENKESDGTZRGRSLTNYTITXRXRGVLWBTEWPJXZYLGHLQBAVYVOSABIVTQYQM9FIQKCBRRUEMVVTMERLWOK".to_string(), SpongeMode::CurlP81);
        assert_eq!(h3.unwrap().to_string(), "TIXEPIEYMGURTQ9ABVYVQSWMNGCVQFASMFAEQWUZCLIWLCDIGYVXOEJBBEMZOIHAYSUQMEFOGZBXUMHQW".to_string());
        println!("{:?}", now.elapsed());

    }

    #[test]
    fn curl_p81_bis() {
        let now = Instant::now();
        let h3 = TxHash::compute_from_trytes("999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999A9RGRKVGWMWMKOLVMDFWJUHNUNYWZTJADGGPZGXNLERLXYWJE9WQHWWBMCPZMVVMJUMWWBLZLNMLDCGDJ999999999999999999999999999999999999999999999999999999YGYQIVD99999999999999999999TXEFLKNPJRBYZPORHZU9CEMFIFVVQBUSTDGSJCZMBTZCDTTJVUFPTCCVHHORPMGCURKTH9VGJIXUQJVHK999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999".to_string(), SpongeMode::CurlP81);
        println!("{:?}", now.elapsed());
        assert_eq!(h3.unwrap().to_string(),"WZWSSZGBNFPYFYVHQQUKFJYBIEEPOXMPVOWYPRMRJBQVXJOSBXCAWJTXSCGRZX9VTFJJZVXTWWYDWONQQ".to_string());
    }

    #[test]
    fn trailing_zeros_test() {
        let h1 = TxHash::new("999999999999999999999999999999999999999999999999999999999999999999999999999999999");
        assert_eq!(h1.trailing_zeros(),243);
        let h1 = TxHash::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999A");
        assert_eq!(h1.trailing_zeros(),2);
        let h1 = TxHash::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999B");
        assert_eq!(h1.trailing_zeros(),1);
        let h1 = TxHash::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999C");
        assert_eq!(h1.trailing_zeros(),1);
        let h1 = TxHash::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999D");
        assert_eq!(h1.trailing_zeros(),1);
        let h1 = TxHash::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999E");
        assert_eq!(h1.trailing_zeros(),0);
        let h1 = TxHash::new("9999999999999999999999999999999999999999999999999999999999999999999999999999999Z9");
        assert_eq!(h1.trailing_zeros(),5);
    }
}