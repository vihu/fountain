use criterion::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use itertools::izip;
use fountaincode::decoder::Decoder;
use fountaincode::ideal_encoder::IdealEncoder;
use fountaincode::robust_encoder::RobustEncoder;
use fountaincode::types::*;

fn encode_decode_systematic_with_loss(total_len: usize, chunk_len: usize, loss: f32) {
    let s: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(total_len)
        .collect();
    let buf = s.into_bytes();
    let len = buf.len();
    let to_compare = buf.clone();

    let enc = IdealEncoder::new(buf, chunk_len, EncoderType::Systematic);
    let mut dec = Decoder::new(len, chunk_len);

    let mut loss_rng = thread_rng();

    for drop in enc {
        if loss_rng.gen::<f32>() > loss {
            match dec.catch(drop) {
                CatchResult::Missing(_) => {
                    //a systematic encoder and no loss on channel should only need k symbols
                    //assert_eq!(stats.cnt_chunks-stats.unknown_chunks, cnt_drops)
                }
                CatchResult::Finished(data, _) => {
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

fn encode_decode_systematic_with_loss_robust_spike(
    total_len: usize,
    chunk_len: usize,
    spike: usize,
    loss: f32) {
    let s: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(total_len)
        .collect();
    let buf = s.into_bytes();
    let len = buf.len();
    let to_compare = buf.clone();

    let enc = RobustEncoder::new(buf, chunk_len, EncoderType::Systematic, SolitonType::RobustSpike, 0.2, spike, loss);
    let mut dec = Decoder::new(len, chunk_len);

    let mut loss_rng = thread_rng();

    for drop in enc {
        if loss_rng.gen::<f32>() > loss {
            match dec.catch(drop) {
                CatchResult::Missing(_) => {
                    //a systematic encoder and no loss on channel should only need k symbols
                    //assert_eq!(stats.cnt_chunks-stats.unknown_chunks, cnt_drops)
                }
                CatchResult::Finished(data, _) => {
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

fn encode_decode_systematic_with_loss_robust_const(
    total_len: usize,
    chunk_len: usize,
    constant: f32,
    loss: f32) {
    let s: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(total_len)
        .collect();
    let buf = s.into_bytes();
    let len = buf.len();
    let to_compare = buf.clone();

    let enc = RobustEncoder::new(buf, chunk_len, EncoderType::Systematic, SolitonType::RobustConst, constant, 40, loss);
    let mut dec = Decoder::new(len, chunk_len);

    let mut loss_rng = thread_rng();

    for drop in enc {
        if loss_rng.gen::<f32>() > loss {
            match dec.catch(drop) {
                CatchResult::Missing(_) => {
                    //a systematic encoder and no loss on channel should only need k symbols
                    //assert_eq!(stats.cnt_chunks-stats.unknown_chunks, cnt_drops)
                }
                CatchResult::Finished(data, _) => {
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

fn bench_ideal_vs_robust(c: &mut Criterion) {
    let mut group = c.benchmark_group("IdealVsRobust");

    let sizes: Vec<usize> = (1000..1002).collect();
    let chunks: Vec<usize> = (100..105).collect();
    let losses: Vec<f32> = vec![0.1, 0.3, 0.5, 0.9];

    for (size, chunk, loss) in izip!(&sizes, &chunks, &losses) {
        group.bench_with_input(BenchmarkId::new("Ideal", size), size,
            |b, size| b.iter(|| encode_decode_systematic_with_loss(*size, *chunk, *loss)));
        group.bench_with_input(BenchmarkId::new("RobustSpike", size), size,
            |b, size| b.iter(|| encode_decode_systematic_with_loss_robust_spike(*size, *chunk, 40, *loss)));
        group.bench_with_input(BenchmarkId::new("RobustConst", size), size,
            |b, size| b.iter(|| encode_decode_systematic_with_loss_robust_const(*size, *chunk, 0.2, *loss)));
    }
    group.finish();
}

criterion_group!(benches, bench_ideal_vs_robust);
criterion_main!(benches);
