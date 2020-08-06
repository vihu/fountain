use crate::soliton::IdealSoliton;
use crate::{
    droplet::Droplet,
    types::{DropType, EncoderType},
};
use rand::{
    distributions::Uniform,
    rngs::StdRng,
    {Rng, SeedableRng},
};
use std::{cmp, vec::Vec};

/// Encoder for Luby transform codes
#[derive(Clone)]
pub struct Encoder {
    data: Vec<u8>,
    len: usize,
    blocksize: usize,
    rng: StdRng,
    dist: rand::distributions::Uniform<usize>,
    cnt_blocks: usize,
    sol: IdealSoliton,
    pub cnt: usize,
    encodertype: EncoderType,
}

impl Encoder {
    /// Constructs a new encoder for Luby transform codes.
    /// In case you send the packages over UDP, the blocksize
    /// should be the MTU size.
    ///
    /// There are two versions of the 'Encoder', Systematic and Random.
    /// The Systematic encoder first produces a set of the source symbols. After each
    /// symbol is sent once, it switches to Random.
    ///
    /// The Encoder implements the iterator. You can use the iterator
    /// to produce an infinte stream of Droplets
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
    ///     let mut enc = Encoder::new(buf, 64, EncoderType::Random);
    ///
    ///     for i in 1..10 {
    ///         println!("droplet {:?}: {:?}", i, enc.next());
    ///     }
    /// }
    /// ```
    pub fn new(data: Vec<u8>, blocksize: usize, encodertype: EncoderType) -> Encoder {
        let mut rng = StdRng::from_entropy();

        let len = data.len();
        let cnt_blocks = ((len as f32) / blocksize as f32).ceil() as usize;
        let sol = IdealSoliton::new(cnt_blocks, rng.gen::<u64>());
        Encoder {
            data: data,
            len: len,
            blocksize: blocksize,
            rng: rng,
            dist: Uniform::new(0, cnt_blocks),
            cnt_blocks: cnt_blocks,
            sol: sol,
            cnt: 0,
            encodertype: encodertype,
        }
    }
}

pub fn get_sample_from_rng_by_seed(
    seed: u64,
    range: rand::distributions::Uniform<usize>,
    degree: usize,
) -> Vec<usize> {
    let rng: StdRng = SeedableRng::seed_from_u64(seed);
    //sample(&mut rng, 0..n, degree)
    let v: Vec<usize> = rng.sample_iter(range).take(degree).collect();
    return v;
}

impl Iterator for Encoder {
    type Item = Droplet;
    fn next(&mut self) -> Option<Droplet> {
        let drop = match self.encodertype {
            EncoderType::Random => {
                let degree = self.sol.next().unwrap() as usize; //TODO: try! macro
                let seed = self.rng.gen::<u64>();
                let sample = get_sample_from_rng_by_seed(seed, self.dist, degree);
                let mut r: Vec<u8> = vec![0; self.blocksize];

                for k in sample {
                    let begin = k * self.blocksize;
                    let end = cmp::min((k + 1) * self.blocksize, self.len);
                    let mut j = 0;

                    for i in begin..end {
                        r[j] ^= self.data[i];
                        j += 1;
                    }
                }
                Some(Droplet::new(DropType::Seeded(seed, degree), r))
            }
            EncoderType::Systematic => {
                let begin = self.cnt * self.blocksize;
                let end = cmp::min((self.cnt + 1) * self.blocksize, self.len);
                let mut r: Vec<u8> = vec![0; self.blocksize];

                let mut j = 0;
                for i in begin..end {
                    r[j] = self.data[i];
                    j += 1;
                }
                if (self.cnt + 2) > self.cnt_blocks {
                    self.encodertype = EncoderType::Random;
                }
                Some(Droplet::new(DropType::Edges(self.cnt), r))
            }
        };

        self.cnt += 1;
        drop
    }
}
