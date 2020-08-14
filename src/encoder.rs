use crate::{
    droplet::Droplet,
    soliton::Soliton,
    types::{DropType, EncoderType},
};
use rand::{
    distributions::Uniform,
    rngs::StdRng,
    {Rng, SeedableRng},
};
use std::cmp;

/// Encoder for Luby transform codes
#[derive(Clone)]
pub struct Encoder {
    data: Vec<u8>,
    len: usize,
    blocksize: usize,
    rng: StdRng,
    dist: rand::distributions::Uniform<usize>,
    cnt_blocks: usize,
    sol: Soliton,
    pub cnt: usize,
    encodertype: EncoderType,
}

/// Encoder for Luby transform codes.
///
/// In case you send the packages over UDP, the blocksize
/// should be the MTU size.
///
/// There are two versions of the 'Encoder', Systematic and Random.
/// The Systematic encoder first produces a set of the source symbols. After each
/// symbol is sent once, it switches to Random.
///
/// # Examples
///
/// ```
/// extern crate rand;
/// extern crate fountaincode;
///
/// fn main() {
///     use fountaincode::encoder::Encoder;
///     use fountaincode::types::EncoderType;
///     use self::rand::{thread_rng, Rng};
///     use rand::distributions::Alphanumeric;
///
///     let s: String = thread_rng().sample_iter(Alphanumeric).take(1024).collect();
///     let buf = s.into_bytes();
///
///     let mut enc = Encoder::robust(buf, 64, EncoderType::Random, 0.2, None, 0.05);
///
///     for i in 1..10 {
///         println!("droplet {:?}: {:?}", i, enc.next());
///     }
/// }
/// ```
impl Encoder {
    pub fn robust(
        data: Vec<u8>,
        blocksize: usize,
        encodertype: EncoderType,
        c: f32,
        spike: Option<usize>,
        delta: f32,
    ) -> Self {
        let mut rng = StdRng::from_entropy();
        let len = data.len();
        let cnt_blocks = (len + blocksize - 1) / blocksize;
        let sol = Soliton::robust(cnt_blocks, rng.gen::<u64>(), c, spike, delta);
        Encoder {
            data,
            len,
            blocksize,
            rng,
            dist: Uniform::new(0, cnt_blocks),
            cnt_blocks,
            sol,
            cnt: 0,
            encodertype,
        }
    }

    pub fn ideal(data: Vec<u8>, blocksize: usize, encodertype: EncoderType) -> Self {
        let mut rng = StdRng::from_entropy();
        let len = data.len();
        let cnt_blocks = (len + blocksize - 1) / blocksize;
        let sol = Soliton::ideal(cnt_blocks, rng.gen::<u64>());
        Self {
            data,
            len,
            blocksize,
            rng,
            dist: Uniform::new(0, cnt_blocks),
            cnt_blocks,
            sol,
            cnt: 0,
            encodertype,
        }
    }

    pub fn drop(&mut self) -> Droplet {
        let mut r = vec![0; self.blocksize];

        let drop = match self.encodertype {
            EncoderType::Random => {
                let degree = self.sol.gen();
                let seed = self.rng.gen::<u64>();
                let sample = get_sample_from_rng_by_seed(seed, self.dist, degree);

                for k in sample {
                    let begin = k * self.blocksize;
                    let end = cmp::min((k + 1) * self.blocksize, self.len);

                    for (src_dat, drop_dat) in self.data[begin..end].iter().zip(r.iter_mut()) {
                        *drop_dat ^= src_dat;
                    }
                }
                Droplet::new(DropType::Seeded(seed, degree), r)
            }
            EncoderType::Systematic => {
                let begin = (self.cnt % self.cnt_blocks) * self.blocksize;
                let end = cmp::min(
                    ((self.cnt % self.cnt_blocks) + 1) * self.blocksize,
                    self.len,
                );

                for (src_dat, drop_dat) in self.data[begin..end].iter().zip(r.iter_mut()) {
                    *drop_dat = *src_dat;
                }
                if (self.cnt + 2) > self.cnt_blocks * 2 {
                    self.encodertype = EncoderType::Random;
                }
                Droplet::new(DropType::Edges(self.cnt % self.cnt_blocks), r)
            }
        };

        self.cnt += 1;
        drop
    }
}

pub fn get_sample_from_rng_by_seed(
    seed: u64,
    range: rand::distributions::Uniform<usize>,
    degree: usize,
) -> impl Iterator<Item = usize> {
    let rng: StdRng = SeedableRng::seed_from_u64(seed);
    rng.sample_iter(range).take(degree)
}

impl Iterator for Encoder {
    type Item = Droplet;
    fn next(&mut self) -> Option<Droplet> {
        Some(self.drop())
    }
}
