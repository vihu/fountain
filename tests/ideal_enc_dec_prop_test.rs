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
