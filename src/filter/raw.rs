use super::Filter;

#[allow(dead_code)]
#[derive(Default)]
pub struct RawFilter;

impl Filter for RawFilter {
    fn apply(&mut self, raw: &[f32], out: &mut [u32]) {
        for (i, e) in raw.iter().enumerate() {
            out[i] = *e as u32;
        }
    }
}
