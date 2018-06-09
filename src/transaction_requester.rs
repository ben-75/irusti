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
    null_hash :TxHash,
}

impl TransactionRequester {

    pub fn new(max_size :usize, p_remove_transaction :f32, tangle :Rc<Tangle>, message_q :Rc<MessageQ>) -> TransactionRequester {
        let transactions_to_request = LinkedHashSet::with_capacity(max_size);
        let milestone_transactions_to_request = LinkedHashSet::with_capacity(500);
        let null_hash = TxHash::new("999999999999999999999999999999999999999999999999999999999999999999999999999999999");
        TransactionRequester{transactions_to_request,milestone_transactions_to_request,max_size,
            p_remove_transaction, tangle, null_hash}
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

    pub fn request_transaction(&mut self, hash :TxHash, milestone :bool) {
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
    }

    pub fn transaction_to_request(mut self, milestone :bool, messageQ :MessageQ) -> Option<TxHash> {
        let mut response :Option<TxHash> = None;
        if milestone && self.milestone_transactions_to_request.len()>0 {
            let mut first_unknown = 0;
            {
                let mut iter = self.milestone_transactions_to_request.iter().enumerate();
                let mut found: bool = false;
                while !found {
                    let next = iter.next();
                    match next {
                        None => break,
                        Some((idx, item)) => {
                            if !self.tangle.exists(item) {
                                messageQ.publish(format!("rtl {}", item.to_string()));
                            } else {
                                first_unknown = idx;
                                response = Some(*item);
                            }
                        }
                    }
                }
            }
            while first_unknown > 0 {
                self.milestone_transactions_to_request.pop_front();
                first_unknown = first_unknown-1;
            }
            if self.p_remove_transaction > 0_f32 {
                if rand::random::<f32>() < self.p_remove_transaction {
                    self.milestone_transactions_to_request.pop_front();
                }
            }
        }

        if (!milestone || response.is_none()) && self.transactions_to_request.len() > 0 {
            let mut first_unknown = 0;
            {
               let mut iter = self.transactions_to_request.iter().enumerate();
               let mut found: bool = false;
                while !found {
                    let next = iter.next();
                    match next {
                        None => break,
                        Some((idx, item)) => {
                            if !self.tangle.exists(item) {
                                messageQ.publish(format!("rtl {}", item.to_string()));
                            } else {
                                first_unknown = idx;
                                response = Some(*item);
                            }
                        }
                    }
                }
            }
            while first_unknown > 0 {
                self.transactions_to_request.pop_front();
                first_unknown = first_unknown-1;
            }
            if self.p_remove_transaction>0_f32 {
                if rand::random::<f32>() < self.p_remove_transaction {
                    self.transactions_to_request.pop_front();
                }
            }
        }
        response
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TransactionRequester{
        let tangle = Tangle::safe_new(configuration.get_param(DefaultConfSettings::DbPath).unwrap(),
                                      configuration.get_flag(DefaultConfSettings::TESTNET));
        let tangle_read_only = Rc::new(tangle);

        let message_q = MessageQ::new(
            Configuration::integer_param(&configuration, DefaultConfSettings::ZmqThreads),
            Configuration::stringify_param(&configuration, DefaultConfSettings::ZmqIpc),
            Configuration::integer_param(&configuration, DefaultConfSettings::ZmqPort),
            Configuration::booling_param(&configuration, DefaultConfSettings::ZmqEnabled)
        );
        let message_q_ref = Rc::new(message_q);

    }
    #[test]
    fn size_test() {
        let tangle = Tangle::new("unittest".to_string());
        let mut transaction_requester = TransactionRequester::new(100,0_f32,Rc::new(tangle));
        assert_eq!(transaction_requester.size(),0);
        assert_eq!(transaction_requester.transaction_to_request_is_full(),false);
        assert_eq!(transaction_requester.get_transactions_to_request().len(),0);
    }

    #[test]
    fn size_test() {
        let tangle = Tangle::new("unittest".to_string());
        let mut transaction_requester = TransactionRequester::new(100,0_f32,Rc::new(tangle));
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        transaction_requester.request_transaction(h1,false);
        assert_eq!(transaction_requester.size(),1);
        assert_eq!(transaction_requester.transaction_to_request(false).unwrap(),h1);
    }


}