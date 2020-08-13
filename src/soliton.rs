use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone)]
pub enum Soliton {
    Ideal {
        rng: StdRng,
        limit: f32,
    },
    Robust {
        rng: StdRng,
        k: usize,
        // constant
        c: f32,
        // another constant
        r: f32,
        // failure probability
        delta: f32,
        // normalization factor
        beta: f32,
        // spike position, usually: k/R unless configured manually as a tuning parameter
        m: usize,
        curr: u64,
    },
}

impl Soliton {
    pub fn ideal(k: usize, seed: u64) -> Self {
        let rng = SeedableRng::seed_from_u64(seed);
        Self::Ideal {
            limit: 1.0 / (k as f32),
            rng,
        }
    }

    pub fn robust(k: usize, seed: u64, c: f32, spike: Option<usize>, delta: f32) -> Self {
        let r = compute_r(k, c, delta);
        if let Some(m) = spike {
            // Spike position was given, use that instead of calculating
            let r = k as f32 / m as f32;
            let beta = compute_beta(k, m, r, delta);
            Self::Robust {
                rng: SeedableRng::seed_from_u64(seed),
                k,
                c,
                r,
                delta,
                beta,
                m,
                curr: 0,
            }
        } else {
            let m = compute_m(k, r);
            let beta = compute_beta(k, m, r, delta);
            Self::Robust {
                rng: SeedableRng::seed_from_u64(seed),
                k,
                c,
                r,
                delta,
                beta,
                m,
                curr: 0,
            }
        }
    }

    pub fn gen(&mut self) -> usize {
        match self {
            Self::Ideal { limit, rng } => {
                let y = rng.gen::<f32>();
                if y >= *limit {
                    (1.0 / y).ceil() as usize
                } else {
                    1
                }
            }
            Self::Robust {
                k,
                c: _c,
                r,
                delta,
                beta,
                m,
                rng,
                curr,
            } => {
                let mut sum = 0.0;
                let mut index = 1;
                let u = rng.gen::<f32>();

                while sum <= u {
                    sum += (rho(*k, index) + tau(index as usize, *m, *r, *delta)) / *beta;
                    index += 1;
                }
                *curr += 1;
                index - 1
            }
        }
    }
}

fn compute_r(k: usize, c: f32, delta: f32) -> f32 {
    c * ((k as f32) / delta).ln() * (k as f32).sqrt()
}

fn compute_m(k: usize, r: f32) -> usize {
    ((k as f32) / r).floor() as usize
}

fn compute_beta(k: usize, m: usize, r: f32, delta: f32) -> f32 {
    let mut sum = 0.0;

    for pos in 1..k {
        sum += rho(k, pos) + tau(pos, m, r, delta)
    }
    sum
}

fn tau(index: usize, m: usize, r: f32, delta: f32) -> f32 {
    if index >= 1 && index < m {
        (1 / (index * m)) as f32
    } else if index == m {
        (r / delta).ln() as f32 / m as f32
    } else {
        0.0
    }
}

fn rho(k: usize, i: usize) -> f32 {
    if i == 1 {
        1.0 / k as f32
    } else {
        1.0 / (i * (i - 1)) as f32
    }
}

impl Iterator for Soliton {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        Some(self.gen())
    }
}
