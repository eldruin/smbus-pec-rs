use core::hash::Hasher;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::SmallRng, FromEntropy, Rng};
use smbus_pec::{pec, Pec};

fn fill_random(v: &mut Vec<u8>) {
    let mut rng = SmallRng::from_entropy();
    for _ in 0..v.capacity() {
        v.push(rng.gen::<u8>());
    }
}

const MESSAGE_SIZES: [usize; 7] = [1, 3, 5, 20, 50, 100, 500];
fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("pec");
    for size in &MESSAGE_SIZES {
        group.throughput(Throughput::Bytes(*size as u64));
        let mut data: Vec<u8> = Vec::with_capacity(*size);
        fill_random(&mut data);
        group.bench_with_input(BenchmarkId::new("fn", *size), &data, |b, data| {
            b.iter(|| {
                let pec = pec(data);
                black_box(&pec);
            });
        });
        group.bench_with_input(BenchmarkId::new("hasher", *size), &data, |b, data| {
            b.iter(|| {
                let mut hasher = Pec::new();
                hasher.write(data);
                let pec = hasher.finish();
                black_box(&pec);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
