use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fountaincode::{
    decoder::{CatchResult, Decoder},
    encoder::{Encoder, EncoderType},
};
use itertools::iproduct;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

fn robust_enc_dec_helper(
    chunk_len: usize,
    loss: f32,
    c: f32,
    spike: Option<usize>,
    delta: f32,
    enc_type: EncoderType,
    msg: Vec<u8>,
) {
    let to_compare = msg.clone();
    let mut enc = Encoder::robust(msg, chunk_len, enc_type, c, spike, delta);
    let mut dec = Decoder::new(to_compare.len(), chunk_len);
    let mut loss_rng = thread_rng();

    loop {
        let drop = enc.drop();
        if loss_rng.gen::<f32>() > loss {
            match dec.catch(drop) {
                CatchResult::Missing(_stats) => {}
                CatchResult::Finished(data, _stats) => {
                    assert_eq!(data, to_compare);
                    return;
                }
            }
        }
    }
}

fn ideal_enc_dec_helper(chunk_len: usize, loss: f32, enc_type: EncoderType, msg: Vec<u8>) {
    let to_compare = msg.clone();
    let mut enc = Encoder::ideal(msg, chunk_len, enc_type);
    let mut dec = Decoder::new(to_compare.len(), chunk_len);
    let mut loss_rng = thread_rng();

    loop {
        let drop = enc.drop();
        if loss_rng.gen::<f32>() > loss {
            match dec.catch(drop) {
                CatchResult::Missing(_stats) => {}
                CatchResult::Finished(data, _stats) => {
                    assert_eq!(data, to_compare);
                    return;
                }
            }
        }
    }
}

fn bench_ideal_vs_robust(c: &mut Criterion) {
    let mut group = c.benchmark_group("IdealVsRobust");

    let sizes = 1000_usize..1002;
    let chunks = 100_usize..105;
    let losses = [0.1, 0.3, 0.5, 0.9];

    // We should parameterize these as well
    let robust_c = 0.2;
    let robust_delta = 0.05;

    for (size, chunk, loss) in iproduct!(sizes, chunks, losses.iter()) {
        let msg: Vec<u8> = thread_rng()
            .sample_iter(Alphanumeric)
            .take(size)
            .collect::<String>()
            .into_bytes();

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("Ideal_{}_{}_{}", size, chunk, loss)),
            &(chunk, loss, msg.clone()),
            |b, (chunk, &loss, msg)| {
                b.iter(|| ideal_enc_dec_helper(*chunk, loss, EncoderType::Systematic, msg.to_vec()))
            },
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("Robust_{}_{}_{}", size, chunk, loss)),
            &(chunk, loss, msg),
            |b, (chunk, &loss, msg)| {
                b.iter(|| {
                    robust_enc_dec_helper(
                        *chunk,
                        loss,
                        robust_c,
                        None,
                        robust_delta,
                        EncoderType::Systematic,
                        msg.to_vec(),
                    )
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_ideal_vs_robust);
criterion_main!(benches);
