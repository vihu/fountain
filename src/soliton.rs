use rand::{distributions::Distribution, Rng};

#[derive(Debug, Clone)]
pub enum Soliton {
    Ideal {
        limit: f32,
    },
    Robust {
        k: usize,
        // another constant
        r: f32,
        // failure probability
        delta: f32,
        // normalization factor
        beta: f32,
        // spike position, usually: k/R unless configured manually as a tuning parameter
        m: usize,
    },
}

impl Soliton {
    pub fn ideal(k: usize) -> Self {
        Self::Ideal {
            limit: 1.0 / (k as f32),
        }
    }

    pub fn robust(k: usize, c: f32, spike: Option<usize>, delta: f32) -> Self {
        if let Some(m) = spike {
            // Spike position was given, use that instead of calculating
            let r = k as f32 / m as f32;
            let beta = compute_beta(k, m, r, delta);
            Self::Robust {
                k,
                r,
                delta,
                beta,
                m,
            }
        } else {
            let r = compute_r(k, c, delta);
            let m = compute_m(k, r);
            let beta = compute_beta(k, m, r, delta);
            Self::Robust {
                k,
                r,
                delta,
                beta,
                m,
            }
        }
    }
}

impl Distribution<usize> for Soliton {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        match self {
            Self::Ideal { limit } => {
                let y = rng.gen::<f32>();
                if y >= *limit {
                    (1.0 / y).ceil() as usize
                } else {
                    1
                }
            }
            Self::Robust {
                k,
                r,
                delta,
                beta,
                m,
            } => {
                let mut sum = 0.0;
                let mut index = 1;
                let u = rng.gen::<f32>();

                while sum <= u {
                    sum += (rho(*k, index) + tau(index as usize, *m, *r, *delta)) / *beta;
                    index += 1;
                }
                index - 1
            }
        }
    }
}

fn compute_r(k: usize, c: f32, delta: f32) -> f32 {
    c * ((k as f32) / delta).ln() * (k as f32).sqrt()
}

fn compute_m(k: usize, r: f32) -> usize {
    ((k as f32) / r).ceil() as usize
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
