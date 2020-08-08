use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct RobustSoliton {
    k: usize,
    // constant
    c: f32,
    // another constant
    r: f32,
    // failure probability
    delta: f32,
    // normalization factor
    beta: f32,
    // spike position
    m: usize,
    rng: StdRng,
    curr: u64
}

impl RobustSoliton {
    pub fn new(k: usize, seed: u64, c: f32, delta: f32) -> RobustSoliton {
        let r = compute_r(k, c, delta);
        let m = compute_spike_pos(k, r);
        let beta = compute_normalization_factor(k, m, r, delta);
        RobustSoliton {
            k,
            c,
            r,
            delta,
            beta,
            m,
            curr: 0,
            rng: SeedableRng::seed_from_u64(seed),
        }
    }

    pub fn new_from_spike(k: usize, seed: u64, c: f32, m: usize, delta: f32) -> RobustSoliton {
        let r = k as f32 / m as f32;
        let beta = compute_normalization_factor(k, m, r, delta);
        RobustSoliton {
            k,
            c,
            r,
            delta,
            beta,
            m,
            curr: 0,
            rng: SeedableRng::seed_from_u64(seed),
        }
    }
}

impl Iterator for RobustSoliton {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut sum = 0.0;
        let mut index = 1;
        let u = self.rng.gen::<f32>();

        while sum <= u {
            sum += (ideal_soliton(self.k, index) + unnormalized_robust_soliton(index as usize, self.m, self.r, self.delta)) / self.beta;
            index += 1;
        }
        self.curr = self.curr + 1;
        Some(index - 1)
    }
}

fn compute_r(k: usize, c: f32, delta: f32) -> f32 {
    c * ((k as f32) / delta).ln() * (k as f32).sqrt()
}

fn compute_spike_pos(k: usize, r: f32) -> usize {
    ((k as f32) / r).floor() as usize
}

fn compute_normalization_factor(
    k: usize,
    m: usize,
    r: f32,
    delta: f32) -> f32 {
    let mut sum = 0.0;

    for pos in 1..k {
        sum += ideal_soliton(k, pos) + unnormalized_robust_soliton(pos, m, r, delta)
    }
    sum
}

fn unnormalized_robust_soliton(
    index: usize,
    m: usize,
    r: f32,
    delta: f32) -> f32 {
    if 1 <= index && index <= m - 1 {
        (1 / (index * m)) as f32
    } else if index == m {
        (r / delta).ln() as f32 / m as f32
    } else {
        0.0
    }
}

fn ideal_soliton(k: usize, i: usize) -> f32 {
    if i == 1 {
        1.0 / k as f32
    } else {
        1.0 / (i * (i - 1)) as f32
    }
}
