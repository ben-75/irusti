extern crate rand;

use tangle::Tangle;
use tips_view_model::TipsViewModel;
use linked_hash_set::LinkedHashSet;
use txhash::TxHash;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::rc::Rc;
use zmq_wrapper::MessageQ;
use rand::prelude::*;

pub struct TransactionRequester{
    transactions_to_request :LinkedHashSet<TxHash>,
    milestone_transactions_to_request :LinkedHashSet<TxHash>,
    max_size :usize,
    p_remove_transaction :f32,
    tangle :Rc<Tangle>,
    message_q :Rc<MessageQ>,
    null_hash :TxHash,
}

impl TransactionRequester {

    pub fn new(max_size :usize, p_remove_transaction :f32, tangle :Rc<Tangle>, message_q :Rc<MessageQ>) -> TransactionRequester {
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
        self.transactions_to_request.len()>self.max_size
    }

    pub fn clear_transaction_request(&mut self, hash :TxHash) -> bool {
        let milestone :bool = self.milestone_transactions_to_request.remove(&hash);
        let normal :bool = self.transactions_to_request.remove(&hash);
        return normal || milestone;
    }

    pub fn request_transaction(&mut self, hash :TxHash, milestone :bool) -> &mut TransactionRequester{
        if !hash.eq(&self.null_hash) && !self.tangle.exists(&hash) {
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
        let mut requestSet = &mut self.transactions_to_request;
        let mut scan_milestones = false;
        if (milestone && &self.milestone_transactions_to_request.len()>&0) || requestSet.len()==0 {
            requestSet = &mut self.milestone_transactions_to_request;
            scan_milestones = true;
        }
        if requestSet.len()==0 {
            return None;
        }

        let mut first_unknown = 0;
        {
            let mut iter = requestSet.iter().enumerate();
            let mut found: bool = false;
            while !found {
                let next = iter.next();
                match next {
                    None => break,
                    Some((idx, item)) => {
                        if self.tangle.exists(item) {
                            self.message_q.publish(format!("rtl {}", item.to_string()));
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
            requestSet.pop_front();
            first_unknown = first_unknown-1;
        }
        if self.p_remove_transaction > 0_f32 {
            if rand::random::<f32>() < self.p_remove_transaction {
                requestSet.pop_front();
            }
        }
        response
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(db_path :&str) -> TransactionRequester{
        let test_max_size = 100;
        let p_remove_transaction_test = 0_f32;
        setup_internal(db_path, test_max_size, p_remove_transaction_test)
    }

    fn setup_always_remove(db_path :&str) -> TransactionRequester{
        let test_max_size = 100;
        let p_remove_transaction_test = 1_f32;
        setup_internal(db_path, test_max_size, p_remove_transaction_test)
    }

    fn setup_internal(db_path: &str, test_max_size: usize, p_remove_transaction_test: f32) -> TransactionRequester {
        let tangle = Tangle::safe_new(db_path.to_string(),
                                      true);
        let tangle_read_only = Rc::new(tangle);
        let message_q = MessageQ::new(
            1,
            None,
            5566,
            false);
        let message_q_ref = Rc::new(message_q);
        let transactions_to_request = LinkedHashSet::with_capacity(test_max_size);
        let milestone_transactions_to_request = LinkedHashSet::with_capacity(500);
        let null_hash = TxHash::new("999999999999999999999999999999999999999999999999999999999999999999999999999999999");
        TransactionRequester {
            transactions_to_request,
            milestone_transactions_to_request,
            max_size: test_max_size,
            p_remove_transaction: p_remove_transaction_test,
            tangle: tangle_read_only,
            message_q: message_q_ref.clone(),
            null_hash
        }
    }

    #[test]
    fn size_test() {
        let mut transaction_requester = setup("dbtests/unittest1");
        assert_eq!(transaction_requester.size(),0);
        assert_eq!(transaction_requester.transaction_to_request_is_full(),false);
        assert_eq!(transaction_requester.get_transactions_to_request().len(),0);
    }

    #[test]
    fn request_transaction_test() {
        let mut transaction_requester = setup("dbtests/unittest2");
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        transaction_requester.request_transaction(h1,false);
        assert_eq!(transaction_requester.size(),1);
        assert_eq!(transaction_requester.transaction_to_request(false).unwrap(),h1);
    }

    #[test]
    fn request_milestone_transaction_test() {
        let mut transaction_requester = setup("dbtests/unittest3");
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        transaction_requester.request_transaction(h1,true);
        assert_eq!(transaction_requester.size(),1);
        assert_eq!(transaction_requester.transaction_to_request(false).unwrap(),h1);
        assert_eq!(transaction_requester.size(),1);
    }

    #[test]
    fn request_milestone_transaction2_test() {
        let mut transaction_requester = setup("dbtests/unittest4");
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
        let mut transaction_requester = setup_always_remove("dbtests/unittest5");
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        let h2 = TxHash::new("AAADEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        transaction_requester.request_transaction(h1,false)
            .request_transaction(h2,true);
        assert_eq!(transaction_requester.size(),2);
        assert_eq!(transaction_requester.transaction_to_request(true).unwrap(),h2);
        assert_eq!(transaction_requester.size(),1);
    }

}