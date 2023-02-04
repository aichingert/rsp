// Random - generating random numbers
// aichingert

const A: i32 = 234;
const M: i32 = 34975321;

const Q: i32 = M / A;
const R: i32 = M % A;

#[derive(Debug)]
pub struct Rand {
    seed: i32
}

impl Rand {
    pub fn new(seed: i32) -> Self {
        Self { seed }
    }
}

impl IntoIterator for Rand {
    type Item = i32;
    type IntoIter = RandIterator;

    fn into_iter(self) -> Self::IntoIter {
        RandIterator { seed: self.seed }
    }
}

pub struct RandIterator {
    seed: i32
}

impl Iterator for RandIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = A * (self.seed % Q) - R * (self.seed / Q);

        if self.seed < 0 {
            Some(self.seed + M)
        } else {
            Some(self.seed)
        }
    }
}
