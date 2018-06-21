pub trait Sponge
{
    fn absorb(&mut self, trites_to_calculate :&mut [i8]);
    fn squeeze(&mut self, out: &mut [i8]);
}