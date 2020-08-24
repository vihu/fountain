extern crate fountaincode;
extern crate rand;

use self::fountaincode::soliton::Soliton;
use rand::{
    rngs::StdRng,
    distributions::Distribution,
    SeedableRng,
};

pub fn ideal_dist(n: usize) -> Vec<f64> {
    let mut probs = vec![0.0, 1.0 / n as f64];

    for k in 2..n + 1 {
        probs.push(1.0 / (k * (k - 1)) as f64)
    }

    probs
}

pub fn robust_dist(n: usize, c: f64, delta: f64) -> Vec<f64> {
    // let m = ((n as f64 / 2.0) + 1.0).ceil() as usize;
    // let r = n as f64 / m as f64;

    // R = c.ln(k/delta).sqrt(k)
    let r = c * (n as f64 / delta).ln() * (n as f64).sqrt();

    // m = k / R
    let m = (n as f64 / r).ceil() as usize;

    println!("spike: {:?}", m);

    let mut extra_prob: Vec<f64> = vec![0.0];
    for i in 1..m {
        extra_prob.push(1.0 / (i as f64 * m as f64))
    }

    extra_prob.push(((r / delta).ln()) / m as f64);

    for _ in m + 1..n + 1 {
        extra_prob.push(0.0)
    }

    let ideal_probs = ideal_dist(n);

    let mut probs = Vec::with_capacity(n + 1);

    for (a, b) in extra_prob.iter().zip(ideal_probs.iter()) {
        probs.push(a + b)
    }

    let prob_sum: f64 = probs.iter().sum();

    probs = probs.iter().map(|x| x / prob_sum).collect::<Vec<f64>>();

    probs
}

#[test]
fn ideal_dist_test() {
    let mut histogram = vec![0.0; 11];
    let mut rng = StdRng::from_entropy();
    let sol = Soliton::ideal(histogram.len() - 1);
    let iterations = 100000 * histogram.len();
    for _ in 0..iterations {
        let deg = sol.sample(&mut rng);
        histogram[deg] += 1.0 / iterations as f64;
    }

    let truth = ideal_dist(10);

    println!("Truth (ideal):     \t{:.6?}", truth);
    println!("Generated (ideal): \t{:.6?}", histogram);

    assert_eq!(histogram[0], 0.0);
}

#[test]
fn robust_dist_test() {
    let mut histogram = vec![0.0; 10];
    let mut rng = StdRng::from_entropy();
    let sol = Soliton::robust(histogram.len(), 0.1, None, 0.01);
    let iterations = 100000 * histogram.len();
    for _ in 0..iterations {
        let deg = sol.sample(&mut rng);
        histogram[deg] += 1.0 / iterations as f64;
    }

    let truth = robust_dist(9, 0.1, 0.01);

    println!("Truth (robust):     \t{:.6?}", truth);
    println!("Generated (robust): \t{:.6?}", histogram);

    assert_eq!(histogram[0], 0.0);
}
