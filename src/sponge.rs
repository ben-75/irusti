pub trait Sponge
{
    fn absorb(&mut self, trites_to_calculate :Vec<i8>);
    fn squeeze(&mut self, trits: Vec<i8>,mut length: usize) -> Vec<i8>;
}