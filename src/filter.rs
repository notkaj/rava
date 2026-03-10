pub mod cava;
pub mod experimental;
pub mod normal;
pub mod raw;
pub mod smooth;

pub trait Filter {
    fn apply(&mut self, input: &[f32], out: &mut [u32]);
}
