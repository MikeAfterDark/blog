use criterion::{criterion_group, criterion_main, Criterion};
use rand::distr::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::rng;
use rand_pcg::Pcg64Mcg;
use rand::SeedableRng;
use rand::Rng;

const SAMPLE_COUNT: u32 = 1_000_000;
const MIN: u128 = 0;
const MAX: u128 = u128::MAX;

fn gen_random_u128s(
    count: u32,
    distribution: &Uniform<u128>,
    rng: &mut (impl Rng + ?Sized),
) -> Vec<u128> {
    let mut results = Vec::with_capacity(count as usize);
    let iter = distribution.sample_iter(rng);
    for (_, sample) in (0..count).zip(iter) {
        results.push(sample);
    }
    results
}

fn bench_rng(c: &mut Criterion) {
    let dist = Uniform::<u128>::new_inclusive(MIN, MAX).unwrap();
    c.bench_function("Rng u128", |b| {
        b.iter(|| {
            let mut rng = rng();
            let _ = gen_random_u128s(SAMPLE_COUNT, &dist, &mut rng);
        });
    });
}

fn bench_small_rng(c: &mut Criterion) {
    let dist = Uniform::<u128>::new_inclusive(MIN, MAX).unwrap();
    c.bench_function("SmallRng u128", |b| {
        b.iter(|| {
            let mut rng = SmallRng::from_rng(&mut rand::rng()); 
            let _ = gen_random_u128s(SAMPLE_COUNT, &dist, &mut rng);
        });
    });
}

fn bench_pcg64(c: &mut Criterion) {
    let dist = Uniform::<u128>::new_inclusive(MIN, MAX).unwrap();
    c.bench_function("Pcg64Mcg u128", |b| {
        b.iter(|| {
            let mut rng = Pcg64Mcg::from_rng(&mut rand::rng());
            let _ = gen_random_u128s(SAMPLE_COUNT, &dist, &mut rng);
        });
    });
}

criterion_group!(
    benches,
    bench_rng,
    bench_small_rng,
    bench_pcg64
);
criterion_main!(benches);
