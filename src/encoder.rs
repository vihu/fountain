use std::vec::Vec;
use std::cmp;
use rand::{Rng, sample, StdRng, SeedableRng};
use crate::types::{EncoderType, DropType};
use crate::droplet::Droplet;

use soliton::IdealSoliton;

/// Encoder for Luby transform codes
#[derive(Clone)]
pub struct Encoder {
    data: Vec<u8>,
    len: usize,
    blocksize: usize,
    rng: StdRng,
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
    ///     use fountaincode::ltcode::{Encoder, EncoderType};
    ///     use self::rand::{thread_rng, Rng};
    ///
    ///     let s:String = thread_rng().gen_ascii_chars().take(1_024).collect();
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
        let mut rng = StdRng::new().unwrap();

        let len = data.len();
        let cnt_blocks = ((len as f32) / blocksize as f32).ceil() as usize;
        let sol = IdealSoliton::new(cnt_blocks, rng.gen::<usize>());
        Encoder {
            data: data,
            len: len,
            blocksize: blocksize,
            rng: rng,
            cnt_blocks: cnt_blocks,
            sol: sol,
            cnt: 0,
            encodertype: encodertype,
        }
    }
}

pub fn get_sample_from_rng_by_seed(seed: usize, n: usize, degree: usize) -> Vec<usize> {
    let seedarr: &[_] = &[seed];
    let mut rng: StdRng = SeedableRng::from_seed(seedarr);
    sample(&mut rng, 0..n, degree)
}

impl Iterator for Encoder {
    type Item = Droplet;
    fn next(&mut self) -> Option<Droplet> {
        let drop = match self.encodertype {
            EncoderType::Random => {
                let degree = self.sol.next().unwrap() as usize; //TODO: try! macro
                let seed = self.rng.gen::<u32>() as usize;
                let sample = get_sample_from_rng_by_seed(seed, self.cnt_blocks, degree);
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
