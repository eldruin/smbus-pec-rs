use criterion::{
    black_box, criterion_group, criterion_main, Bencher, Criterion, ParameterizedBenchmark,
    Throughput,
};
use rand::{rngs::SmallRng, FromEntropy, Rng};
use smbus_pec::pec;

fn do_pec_bench(b: &mut Bencher, size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(*size);
    fill_random(&mut v);
    b.iter(|| {
        let pec = pec(&v);
        black_box(&pec);
    });
}

fn fill_random(v: &mut Vec<u8>) {
    let mut rng = SmallRng::from_entropy();
    for _ in 0..v.capacity() {
        v.push(rng.gen::<u8>());
    }
}

const MESSAGE_SIZES: [usize; 7] = [1, 3, 5, 20, 50, 100, 500];

fn benchmarks(byte_sizes: &[usize]) -> ParameterizedBenchmark<usize> {
    ParameterizedBenchmark::new("pec", do_pec_bench, byte_sizes.iter().cloned())
        .throughput(|s| Throughput::Bytes(*s as u64))
}

fn bench(c: &mut Criterion) {
    c.bench("bench", benchmarks(&MESSAGE_SIZES[..]));
}

criterion_group!(benches, bench);
criterion_main!(benches);
