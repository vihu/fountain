use criterion::*;
use std::time::Duration;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use fountaincode::decoder::Decoder;
use fountaincode::encoder::Encoder;
use fountaincode::types::*;

fn encode_decode_random(total_len: usize, chunk_len: usize) {
    let s: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(total_len)
        .collect();
    let buf = s.into_bytes();
    let len = buf.len();
    let to_compare = buf.clone();

    let enc = Encoder::new(buf, chunk_len, EncoderType::Random);
    let mut dec = Decoder::new(len, chunk_len);

    for drop in enc {
        match dec.catch(drop) {
            CatchResult::Missing(_stats) => {
                // println!("Missing blocks {:?}", stats);
            }
            CatchResult::Finished(data, _stats) => {
                assert_eq!(to_compare.len(), data.len());
                for i in 0..len {
                    // println!("stats: {:?}", stats);
                    assert_eq!(to_compare[i], data[i]);
                }
                // println!("Finished, stats: {:?}", stats);
                return;
            }
        }
    }
}

fn bench_enc_dec_uneven_sizes_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("enc_dec");
    group.measurement_time(Duration::from_secs(1));

    for size in 1000..1003 {
        for chunk in 100..103 {
            let data = (size, chunk);
            let parameter_string = format!(" size: {}, chunk: {}", size, chunk);
            group.bench_with_input(BenchmarkId::new("random_uneven_sizes", parameter_string), &data,
                |b, (data_size, data_chunk) | b.iter(|| encode_decode_random(*data_size, *data_chunk)));
        }
    }
    group.finish();
}

criterion_group!(benches, bench_enc_dec_uneven_sizes_random);
criterion_main!(benches);
