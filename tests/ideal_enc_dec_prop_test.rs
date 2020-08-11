extern crate fountaincode;
extern crate rand;
extern crate stopwatch;

use self::fountaincode::decoder::Decoder;
use self::fountaincode::ideal_encoder::IdealEncoder;
use self::fountaincode::types::*;
use proptest::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use stopwatch::Stopwatch;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50000))]
    #[test]
    fn ideal_prop_test(total_len in 1024u64..8192, chunk_len in 8u32..512) {
        let s: String = thread_rng()
            .sample_iter(Alphanumeric)
            .take(total_len as usize)
            .collect();
        let buf = s.into_bytes();
        let len = buf.len();
        let to_compare = buf.clone();

        let enc = IdealEncoder::new(buf, chunk_len as usize, EncoderType::Systematic);
        let mut dec = Decoder::new(len, chunk_len as usize);

        let sw = Stopwatch::start_new();
        let res = run(enc, &mut dec);
        println!("time: {:#?}", sw.elapsed());

        prop_assert_eq!(to_compare, res);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50000))]
    #[test]
    fn ideal_prop_lossy_test(total_len in 1024u64..8192, chunk_len in 8u32..512) {
        let s: String = thread_rng()
            .sample_iter(Alphanumeric)
            .take(total_len as usize)
            .collect();
        let buf = s.into_bytes();
        let len = buf.len();
        let to_compare = buf.clone();

        let mut dec = Decoder::new(len, chunk_len as usize);
        let mut res: Vec<u8> = vec![];
        let losses = vec![0.1, 0.3, 0.5, 0.9];

        for loss in losses {
            let enc = IdealEncoder::new(buf.clone(), chunk_len as usize, EncoderType::Systematic);
            let sw = Stopwatch::start_new();
            res = run_lossy(enc, &mut dec, loss);
            println!("total_len: {:?}, chunk_len: {:?}, loss: {:?}, time: {:#?}", total_len, chunk_len, loss, sw.elapsed());
        }

        prop_assert_eq!(to_compare, res);
    }
}

fn run(enc: IdealEncoder, dec: &mut Decoder) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    for drop in enc {
        match dec.catch(drop) {
            CatchResult::Missing(_stats) => {
                // println!("Missing blocks {:?}", stats);
            }
            CatchResult::Finished(data, _stats) => {
                // println!("Finished, stats: {:?}", stats);
                out = data;
                break;
            }
        }
    }
    out
}

fn run_lossy(enc: IdealEncoder, dec: &mut Decoder, loss: f32) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    let mut loss_rng = thread_rng();
    for drop in enc {
        if loss_rng.gen::<f32>() > loss {
            match dec.catch(drop) {
                CatchResult::Missing(_stats) => {
                    // println!("Missing blocks {:?}", stats);
                }
                CatchResult::Finished(data, stats) => {
                    println!("overhead: {:?}", stats.overhead);
                    out = data;
                    break;
                }
            }
        }
    }
    out
}
