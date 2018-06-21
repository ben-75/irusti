pub trait Sponge
{
    fn absorb(&mut self, trites_to_calculate :Vec<i8>, mut offset: usize,mut length :usize);
    fn squeeze(&mut self, trits: Vec<i8>,mut offset: usize,mut length: usize) -> Vec<i8>;
}