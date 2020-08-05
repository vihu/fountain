use std::rc::Rc;
use std::cell::RefCell;
use crate::types::{DropType, CatchResult};
use crate::block::Block;
use crate::droplet::{Droplet, RxDroplet};
use crate::encoder::get_sample_from_rng_by_seed;

/// Decoder for the Luby transform
pub struct Decoder {
    total_length: usize,
    blocksize: usize,
    unknown_chunks: usize,
    number_of_chunks: usize,
    cnt_received_drops: usize,
    blocks: Vec<Block>,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct Statistics {
    pub cnt_droplets: usize,
    pub cnt_chunks: usize,
    pub overhead: f32,
    pub unknown_chunks: usize,
}

impl Decoder {
    /// Creates a new Decoder for LT codes
    ///
    /// # Example
    ///
    /// ```
    /// extern crate rand;
    /// extern crate fountaincode;
    ///
    /// fn main() {
    ///     use self::fountaincode::encoder::Encoder;
    ///     use self::fountaincode::decoder::Decoder;
    ///     use self::fountaincode::types::*;
    ///     use self::rand::{thread_rng, Rng};
    ///
    ///     let s:String = thread_rng().gen_ascii_chars().take(1_024).collect();
    ///     let buf = s.into_bytes();
    ///     let to_compare = buf.clone();
    ///     let length = buf.len();
    ///
    ///     let mut enc = Encoder::new(buf, 64, EncoderType::Random);
    ///     let mut dec = Decoder::new(length, 64);
    ///
    ///     for drop in enc {
    ///         match dec.catch(drop) {
    ///             CatchResult::Missing(stats) => {
    ///                 println!("Missing blocks {:?}", stats);
    ///             }
    ///             CatchResult::Finished(data, stats) => {
    ///                 for i in 0..length {
    ///                     assert_eq!(to_compare[i], data[i]);
    ///                 }
    ///                 println!("Finished, stas: {:?}", stats);
    ///                 //write data to disk??
    ///                 return
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn new(len: usize, blocksize: usize) -> Decoder {
        let number_of_chunks = ((len as f32) / blocksize as f32).ceil() as usize;
        let data: Vec<u8> = vec![0; number_of_chunks * blocksize];
        let mut edges: Vec<Block> = Vec::with_capacity(number_of_chunks);
        for i in 0..number_of_chunks {
            let blk = Block::new(i, Vec::new(), blocksize * i, false);
            edges.push(blk);
        }

        Decoder {
            total_length: len,
            number_of_chunks: number_of_chunks,
            unknown_chunks: number_of_chunks,
            cnt_received_drops: 0,
            blocks: edges,
            data: data,
            blocksize: blocksize,
        }
    }

    fn process_droplet(&mut self, droplet: RxDroplet) {
        let mut drops: Vec<Box<RxDroplet>> = Vec::new();
        drops.push(Box::new(droplet));
        loop {
            // a loop is used instead of recursion
            match drops.pop() {
                None => return,
                Some(drop) => {
                    let edges = drop.edges_idx.clone();
                    // TODO: Maybe add shortcut for the first wave of
                    // systematic codes, reduce overhead

                    for ed in edges {
                        // the list is edited, hence we copy first
                        let block = self.blocks.get_mut(ed).unwrap();
                        if block.is_known {
                            let mut b_drop = drop.clone();
                            for i in 0..self.blocksize {
                                b_drop.data[i] ^= self.data[block.begin_at + i];
                            }
                            let pos = b_drop.edges_idx.iter().position(|x| x == &ed).unwrap();
                            b_drop.edges_idx.remove(pos);
                        } else {
                            block.edges.push(drop.clone());
                        }
                    }
                    if drop.clone().edges_idx.len() == 1 {
                        let first_idx = drop.edges_idx.clone().get(0).unwrap().clone();

                        let block = self.blocks.get_mut(first_idx).unwrap();

                        if block.is_known == false {
                            {
                                let b_drop = &drop;
                                for i in 0..self.blocksize {
                                    self.data[block.begin_at + i] = b_drop.data[i];
                                }
                            }
                            block.is_known = true;
                            self.unknown_chunks -= 1;

                            while block.edges.len() > 0 {
                                let mut edge = block.edges.pop().unwrap();
                                let m_edge = &mut edge;

                                if m_edge.edges_idx.len() == 1 {
                                    drops.push(edge);
                                } else {
                                    for i in 0..self.blocksize {
                                        m_edge.data[i] ^= self.data[block.begin_at + i]
                                    }

                                    let pos = m_edge.edges_idx
                                        .iter()
                                        .position(|x| x == &block.idx)
                                        .unwrap();
                                    m_edge.edges_idx.remove(pos);
                                    if m_edge.edges_idx.len() == 1 {
                                        drops.push(edge.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Catches a Droplet
    /// When it is possible to reconstruct a set, the bytes are returned
    pub fn catch(&mut self, drop: Droplet) -> CatchResult {
        self.cnt_received_drops += 1;
        let sample: Vec<usize> =
            match drop.droptype {
                DropType::Seeded(seed, degree) => {
                    get_sample_from_rng_by_seed(seed, self.number_of_chunks, degree)
                }
                DropType::Edges(edges) => vec![edges],
            };

        let rxdrop = RxDroplet {
            edges_idx: sample,
            data: drop.data,
        };
        self.process_droplet(rxdrop);
        let stats = Statistics {
            cnt_droplets: self.cnt_received_drops,
            cnt_chunks: self.number_of_chunks,
            overhead: self.cnt_received_drops as f32 * 100.0 / self.number_of_chunks as f32,
            unknown_chunks: self.unknown_chunks,
        };

        if self.unknown_chunks == 0 {
            let mut result = Vec::with_capacity(self.total_length);
            for i in 0..self.total_length {
                // TODO: we should be able to do that without copying
                result.push(self.data[i]);
            }
            CatchResult::Finished(result, stats)
        } else {
            CatchResult::Missing(stats)
        }
    }
}