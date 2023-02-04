// Random - pseudo random number generator using a seed
// aichingert

const A: f32 = 32132.232;
const M: f32 = 23832.;
const B: f32 = 12482.23;

#[derive(Debug)]
pub struct Rand {
    seed: f32 
}

impl Rand {
    pub fn new(seed: f32) -> Self {
        Self { seed: seed % M }
    }
}

impl IntoIterator for Rand {
    type Item = f32;
    type IntoIter = RandIterator;

    fn into_iter(self) -> Self::IntoIter {
        RandIterator { seed: self.seed }
    }
}

pub struct RandIterator {
    seed: f32
}

impl Iterator for RandIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = (A * self.seed + B) % (M+1.);
        Some(self.seed / M)
    }
}
