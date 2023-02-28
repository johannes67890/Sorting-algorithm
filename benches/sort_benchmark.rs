use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sorting_algorithm::bubblesort;

fn sort_arrays_benchmark(c: &mut Criterion) {
    let mut arr = black_box([-2,-7,2,10,20,9]);

    c.bench_function("Bubblesort", |b| b.iter(|| bubblesort(&mut arr)));

}

criterion_group!(benches, sort_arrays_benchmark);
criterion_main!(benches);
