use tangle::Tangle;
use tips_view_model::TipsViewModel;

pub struct TransactionRequester{
    tips_view_model :TipsViewModel,
}

impl TransactionRequester {

    pub fn new(tips_view_model :TipsViewModel) -> TransactionRequester {
        TransactionRequester{tips_view_model}
    }

}
