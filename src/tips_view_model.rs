use std::collections::VecDeque;
use txhash::TxHash;

pub struct TipsViewModel {
    tips :VecDeque<TxHash>,
    solid_tips :VecDeque<TxHash>,
}

impl TipsViewModel {

    pub fn new() -> TipsViewModel {
        let mut tips = VecDeque::with_capacity(5000);
        let mut solid_tips = VecDeque::with_capacity(5000);
        TipsViewModel{tips,solid_tips}
    }

    pub fn size(&self) -> usize {
        self.solid_tips.len() + self.tips.len()
    }


    pub fn solid_size(&self) -> usize {
        self.solid_tips.len()
    }
}