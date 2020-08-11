extern crate fountaincode;
extern crate rand;
extern crate stopwatch;

use self::fountaincode::decoder::Decoder;
use self::fountaincode::robust_encoder::RobustEncoder;
use self::fountaincode::encoder::Encoder;
use self::fountaincode::types::*;
use proptest::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use stopwatch::Stopwatch;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50000))]
    #[test]
    fn robust_prop_test(total_len in 1024u64..8192, chunk_len in 8u32..512) {
        let s: String = thread_rng()
            .sample_iter(Alphanumeric)
            .take(total_len as usize)
            .collect();
        let buf = s.into_bytes();
        let len = buf.len();
        let to_compare = buf.clone();

        let mut enc = RobustEncoder::new(buf, chunk_len as usize, EncoderType::Systematic, 0.2, None, 0.05);
        let mut dec = Decoder::new(len, chunk_len as usize);

        let sw = Stopwatch::start_new();
        let res = run(&mut enc, &mut dec);
        println!("time: {:#?}", sw.elapsed());

        prop_assert_eq!(to_compare, res);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50000))]
    #[test]
    fn robust_prop_lossy_test(total_len in 1024u64..8192, chunk_len in 8u32..512) {
        let s: String = thread_rng()
            .sample_iter(Alphanumeric)
            .take(total_len as usize)
            .collect();
        let buf = s.into_bytes();
        let len = buf.len();
        let to_compare = buf.clone();

        let losses = vec![0.1, 0.3, 0.5, 0.9];

        let mut dec = Decoder::new(len, chunk_len as usize);
        let mut res: Vec<u8> = vec![];

        for loss in losses {
            let mut enc = RobustEncoder::new(buf.clone(), chunk_len as usize, EncoderType::Systematic, 0.2, None, 0.05);
            let sw = Stopwatch::start_new();
            res = run_lossy(&mut enc, &mut dec, loss);
            println!("total_len: {:?}, chunk_len: {:?}, loss: {:?}, time: {:#?}", total_len, chunk_len, loss, sw.elapsed());
        }

        prop_assert_eq!(to_compare, res);
    }
}

fn run(enc: &mut RobustEncoder, dec: &mut Decoder) -> Vec<u8> {
    let out = loop {
        let drop = enc.next();
        match dec.catch(drop) {
            CatchResult::Missing(_stats) => {
                // println!("Missing blocks {:?}", stats);
            }
            CatchResult::Finished(data, _stats) => {
                // println!("Finished, stats: {:?}", stats);
                break data
            }
        }
    };
    out
}

fn run_lossy(enc: &mut RobustEncoder, dec: &mut Decoder, loss: f32) -> Vec<u8> {
    let mut loss_rng = thread_rng();
    let out = loop {
        if loss_rng.gen::<f32>() > loss {
            let drop = enc.next();
            match dec.catch(drop) {
                CatchResult::Missing(_stats) => {
                    // println!("Missing blocks {:?}", stats);
                }
                CatchResult::Finished(data, stats) => {
                    println!("overhead: {:?}", stats.overhead);
                    break data
                }
            }
        }
    };
    out
}
