extern crate linked_hash_set;

use std::collections::HashSet;
use txhash::TxHash;
use std::iter::FromIterator;
use self::linked_hash_set::LinkedHashSet;

pub struct TipsViewModel {
    tips :LinkedHashSet<TxHash>,
    solid_tips :LinkedHashSet<TxHash>,
}

impl TipsViewModel {

    pub fn new() -> TipsViewModel {
        let mut tips = LinkedHashSet::with_capacity(5000);
        let mut solid_tips = LinkedHashSet::with_capacity(5000);
        TipsViewModel{tips,solid_tips}
    }

    pub fn size(&self) -> usize {
        self.solid_tips.len() + self.tips.len()
    }

    pub fn solid_size(&self) -> usize {
        self.solid_tips.len()
    }

    pub fn add_tip_hash(&mut self, tx_hash :TxHash) {
        self.tips.insert(tx_hash);
    }

    pub fn remove_tip_hash(&mut self, tx_hash :TxHash) {
        if ! self.tips.remove(&tx_hash){
            self.solid_tips.remove(&tx_hash);
        }
    }

    pub fn set_solid(&mut self, tx_hash :TxHash) {
        if self.tips.remove(&tx_hash){
            self.solid_tips.insert(tx_hash);
        }
    }

    pub fn get_tips(&self) -> HashSet<&TxHash> {
        HashSet::from_iter(self.tips.iter().chain(self.solid_tips.iter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime};

    #[test]
    fn size_test() {
        let mut tvm = TipsViewModel::new();
        assert_eq!(tvm.size(),0);
        assert_eq!(tvm.solid_size(),0);
        let h1 = TxHash::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9");
        tvm.add_tip_hash(h1);
        assert_eq!(tvm.size(),1);
        assert_eq!(tvm.solid_size(),0);
        tvm.set_solid(h1);
        assert_eq!(tvm.size(),1);
        assert_eq!(tvm.solid_size(),1);
    }
}