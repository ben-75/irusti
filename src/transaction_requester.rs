extern crate rand;

use tangle::Tangle;
use linked_hash_set::LinkedHashSet;
use txhash::TxHash;
use std::iter::FromIterator;
use std::collections::HashSet;
use zmq_wrapper::MessageQ;

pub struct TransactionRequester<'a,'b>{
    transactions_to_request :LinkedHashSet<TxHash>,
    milestone_transactions_to_request :LinkedHashSet<TxHash>,
    max_size :usize,
    p_remove_transaction :f32,
    tangle :&'a Tangle,
    message_q :&'b MessageQ,
    null_hash :TxHash,
}

impl<'a,'b> TransactionRequester<'a,'b> {

    pub fn new(max_size :usize, p_remove_transaction :f32, tangle :&'a Tangle, message_q :&'b MessageQ) -> TransactionRequester<'a,'b> {
        let transactions_to_request = LinkedHashSet::with_capacity(max_size);
        let milestone_transactions_to_request = LinkedHashSet::with_capacity(500);
        let null_hash = TxHash::new("999999999999999999999999999999999999999999999999999999999999999999999999999999999");
        TransactionRequester{transactions_to_request,milestone_transactions_to_request,max_size,
            p_remove_transaction, tangle, message_q, null_hash}
    }

    pub fn size(&self) -> usize {
        self.transactions_to_request.len() + self.milestone_transactions_to_request.len()
    }

    pub fn get_transactions_to_request(&self) -> HashSet<&TxHash> {
        HashSet::from_iter(self.transactions_to_request.iter().chain(self.milestone_transactions_to_request.iter()))
    }

    fn transaction_to_request_is_full(&self) -> bool{
        self.transactions_to_request.len()>=self.max_size
    }

    pub fn clear_transaction_request(&mut self, hash :TxHash) -> bool {
        let milestone :bool = self.milestone_transactions_to_request.remove(&hash);
        let normal :bool = self.transactions_to_request.remove(&hash);
        return normal || milestone;
    }

    pub fn request_transaction(&mut self, hash :TxHash, milestone :bool) -> &mut TransactionRequester<'a,'b>{
        if !hash.eq(&self.null_hash) && !self.tangle.transaction_exists(&hash) {
            if milestone  {
                self.milestone_transactions_to_request.remove(&hash);
                self.milestone_transactions_to_request.insert(hash);
            } else {
                if !self.milestone_transactions_to_request.contains(&hash) && !self.transaction_to_request_is_full() {
                        self.transactions_to_request.insert(hash);
                }
            }
        }
        self
    }

    pub fn transaction_to_request(&mut self, milestone :bool) -> Option<TxHash> {
        let mut response :Option<TxHash> = None;
        let mut request_set = &mut self.transactions_to_request;
        if (milestone && &self.milestone_transactions_to_request.len()>&0) || request_set.len()==0 {
            request_set = &mut self.milestone_transactions_to_request;
        }
        if request_set.len()==0 {
            return None;
        }

        let mut first_unknown = 0;
        {
            let mut iter = request_set.iter().enumerate();
            let mut found: bool = false;
            while !found {
                let next = iter.next();
                match next {
                    None => break,
                    Some((idx, item)) => {
                        if self.tangle.transaction_exists(item) {
                            first_unknown +=1;
                            self.message_q.publish(&format!("rtl {}", item.to_string()));
                        } else {
                            first_unknown = idx;
                            found = true;
                            response = Some(*item);
                        }
                    }
                }
            }
        }
        while first_unknown > 0 {
            request_set.pop_front();
            first_unknown = first_unknown-1;
        }
        if self.p_remove_transaction > 0_f32 {
            if rand::random::<f32>() < self.p_remove_transaction {
                request_set.pop_front();
            }
        }
        response
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng};

    fn setup<'a,'b>((tg,mq) :(&'a Tangle,&'b MessageQ)) -> TransactionRequester<'a,'b>{
        let test_max_size = 100;
        let p_remove_transaction_test = 0_f32;
        setup_internal((tg,mq), test_max_size, p_remove_transaction_test)
    }

    fn setup_always_remove<'a,'b>((tg,mq) :(&'a Tangle,&'b MessageQ)) -> TransactionRequester<'a,'b>{
        let test_max_size = 100;
        let p_remove_transaction_test = 1_f32;
        setup_internal((tg,mq), test_max_size, p_remove_transaction_test)
    }

    fn setup_internal<'a,'b>((tg,mq) :(&'a Tangle,&'b MessageQ), test_max_size: usize, p_remove_transaction_test: f32) -> TransactionRequester<'a,'b> {
        let transactions_to_request = LinkedHashSet::with_capacity(test_max_size);
        let milestone_transactions_to_request = LinkedHashSet::with_capacity(500);
        let null_hash = TxHash::new("999999999999999999999999999999999999999999999999999999999999999999999999999999999");
        TransactionRequester {
            transactions_to_request,
            milestone_transactions_to_request,
            max_size: test_max_size,
            p_remove_transaction: p_remove_transaction_test,
            tangle :tg,
            message_q :mq,
            null_hash
        }
    }

    fn make_tangle_mq(db_path :&str) -> (Tangle, MessageQ){
        (Tangle::new(db_path.to_string()),MessageQ::new(
            1,
            None,
            5566,
            false))
    }
    #[test]
    fn size_test() {
        {
            let tg_mq = make_tangle_mq("dbtests/unittest1");
            let transaction_requester = setup((&tg_mq.0, &tg_mq.1));
            assert_eq!(transaction_requester.size(),0);
            assert_eq!(transaction_requester.transaction_to_request_is_full(),false);
            assert_eq!(transaction_requester.get_transactions_to_request().len(),0);
        }
        Tangle::shutdown("dbtests/unittest1".to_string());
    }

    #[test]
    fn request_transaction_test() {
        {
            let tg_mq = make_tangle_mq("dbtests/unittest2");
            let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
            let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
            transaction_requester.request_transaction(h1, false);
            assert_eq!(transaction_requester.size(), 1);
            assert_eq!(transaction_requester.transaction_to_request(false).unwrap(), h1);
        };
        Tangle::shutdown("dbtests/unittest2".to_string());
    }

    #[test]
    fn existing_request_transaction_test() {
        {
            let tg_mq = make_tangle_mq("dbtests/unittest6");
            let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
            let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
            &tg_mq.0.transaction_save(&h1, &[1,2,3]);
            transaction_requester.request_transaction(h1,false);
            assert_eq!(transaction_requester.size(),0);
            assert_eq!(transaction_requester.transaction_to_request(false).is_none(),true);
        }
        Tangle::shutdown("dbtests/unittest6".to_string());
    }

    #[test]
    fn existing_request_transaction_test2() {
        {
            let tg_mq = make_tangle_mq("dbtests/unittest7");
            let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
            let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVW9999");
            transaction_requester.request_transaction(h1, false);
            assert_eq!(transaction_requester.size(), 1);
            &tg_mq.0.transaction_save(&h1, &[1, 2, 3]);

            assert_eq!(transaction_requester.transaction_to_request(false).is_none(), true);
            assert_eq!(transaction_requester.size(), 0);
            &tg_mq.0.transaction_delete(&h1);

        }
        Tangle::shutdown("dbtests/unittest7".to_string());

    }

    #[test]
    fn request_milestone_transaction_test() {
        let tg_mq = make_tangle_mq("dbtests/unittest3");
        let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        transaction_requester.request_transaction(h1,true);
        assert_eq!(transaction_requester.size(),1);
        assert_eq!(transaction_requester.transaction_to_request(false).unwrap(),h1);
        assert_eq!(transaction_requester.size(),1);
    }

    #[test]
    fn request_milestone_transaction2_test() {
        let tg_mq = make_tangle_mq("dbtests/unittest4");
        let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        let h2 = TxHash::new("AAADEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        transaction_requester.request_transaction(h1,false)
                             .request_transaction(h2,true);
        assert_eq!(transaction_requester.size(),2);
        assert_eq!(transaction_requester.transaction_to_request(true).unwrap(),h2);
        assert_eq!(transaction_requester.size(),2);
    }

    #[test]
    fn request_milestone_transaction2_remove_test() {
        let tg_mq = make_tangle_mq("dbtests/unittest5");
        let mut transaction_requester = setup_always_remove((&tg_mq.0, &tg_mq.1));
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        let h2 = TxHash::new("AAADEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        transaction_requester.request_transaction(h1,false)
            .request_transaction(h2,true);
        assert_eq!(transaction_requester.size(),2);
        assert_eq!(transaction_requester.transaction_to_request(true).unwrap(),h2);
        assert_eq!(transaction_requester.size(),1);
    }

    #[test]
    fn capacity_limited(){
        let tg_mq = make_tangle_mq("dbtests/unittest8");
        let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
        for _i in 0..(2*transaction_requester.max_size) {
            transaction_requester.request_transaction(TxHash::new(get_random_transaction_hash().as_ref()), false);
        }
        assert_eq!(transaction_requester.size(),transaction_requester.max_size);
    }

    #[test]
    fn milestone_capacity_not_limited(){
        let tg_mq = make_tangle_mq("dbtests/unittest9");
        let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
        for _i in 0..(2*transaction_requester.max_size) {
            transaction_requester.request_transaction(TxHash::new(get_random_transaction_hash().as_ref()), true);
        }
        assert_eq!(transaction_requester.size(),2*transaction_requester.max_size);
    }

    #[test]
    fn mixed_capacity_limited(){
        let tg_mq = make_tangle_mq("dbtests/unittest10");
        let mut transaction_requester = setup((&tg_mq.0, &tg_mq.1));
        for i in 0..(4*transaction_requester.max_size) {
            transaction_requester.request_transaction(TxHash::new(get_random_transaction_hash().as_ref()), i%2==0);
        }
        assert_eq!(transaction_requester.size(),3*transaction_requester.max_size);
    }

    fn get_random_transaction_hash() -> String {
        let mut rng = rand::thread_rng();
        let mut random_h :String = "".to_string();
        for _ in 0..81 {
            let idx :u8 = rng.gen_range(0, 27);
            match idx {
                0 => random_h.push('9'),
                x => random_h.push((x+64) as char),
            }
        }
        random_h
    }

    fn get_random_transaction_trytes() -> String {
        let mut rng = rand::thread_rng();
        let mut random_tx :String = "".to_string();
        for _ in 0..2673 {
            let idx :u8 = rng.gen_range(0, 27);
            match idx {
                0 => random_tx.push('9'),
                x => random_tx.push((x+64) as char),
            }
        }
        random_tx
    }

}