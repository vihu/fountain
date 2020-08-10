extern crate fountaincode;
extern crate rand;

use self::fountaincode::decoder::Decoder;
use self::fountaincode::ideal_encoder::IdealEncoder;
use self::fountaincode::types::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn enc_dec_helper(total_len: usize, chunk_len: usize, loss: f32, enc_type: EncoderType) {
    let s: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(total_len)
        .collect();
    let buf = s.into_bytes();
    let len = buf.len();
    let to_compare = buf.clone();

    let enc = IdealEncoder::new(buf, chunk_len, enc_type);
    let mut dec = Decoder::new(len, chunk_len);

    let mut loss_rng = thread_rng();

    for drop in enc {
        if loss_rng.gen::<f32>() > loss {
            match dec.catch(drop) {
                CatchResult::Missing(stats) => {
                    //a systematic encoder and no loss on channel should only need k symbols
                    //assert_eq!(stats.cnt_chunks-stats.unknown_chunks, cnt_drops)
                    println!("Missing blocks {:?}", stats);
                }
                CatchResult::Finished(data, stats) => {
                    println!("Finished, stats: {:?}", stats);
                    assert_eq!(to_compare.len(), data.len());
                    for i in 0..len {
                        assert_eq!(to_compare[i], data[i]);
                    }
                    return;
                }
            }
        }
    }
}

#[test]
fn ideal_test_encode_decode_simple() {
    enc_dec_helper(1_024, 512, 0.0, EncoderType::Random);
}

#[test]
fn ideal_encode_decode_with_uneven_sizes_random() {
    for size in 1000..1100 {
        for chunk in 100..130 {
            enc_dec_helper(size, chunk, 0.0, EncoderType::Random);
        }
    }
}

#[test]
fn ideal_small_test_systematic_encoder() {
    enc_dec_helper(1300, 128, 0.0, EncoderType::Systematic);
}

#[test]
fn ideal_combinations_encode_decode_with_uneven_sizes_systematic() {
    for size in 1000..1100 {
        for chunk in 100..130 {
            enc_dec_helper(size, chunk, 0.0, EncoderType::Systematic);
        }
    }
}

#[test]
fn ideal_small_encode_decode_with_loss_begin_with_systematic() {
    enc_dec_helper(8, 2, 0.1, EncoderType::Systematic);
}

#[test]
fn ideal_combinations_encode_decode_with_loss_begin_with_systematic() {
    for size in 1000..1100 {
        for chunk in 100..130 {
            for loss in vec![0.1, 0.3, 0.5, 0.9] {
                enc_dec_helper(size, chunk, loss, EncoderType::Systematic);
            }
        }
    }
}
