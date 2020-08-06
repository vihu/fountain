use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone)]
pub struct IdealSoliton {
    limit: f32,
    rng: StdRng,
}

impl IdealSoliton {
    pub fn new(k: usize, seed: u64) -> IdealSoliton {
        let rng = SeedableRng::seed_from_u64(seed);
        IdealSoliton {
            limit: 1.0 / (k as f32),
            rng,
        }
    }
}

impl Iterator for IdealSoliton {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let y = self.rng.gen::<f32>();
        if y >= self.limit {
            let res = (1.0 / y).ceil() as usize;
            Some(res)
        } else {
            Some(1)
        }
    }
}
