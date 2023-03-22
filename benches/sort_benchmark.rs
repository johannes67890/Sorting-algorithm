use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sorting_algorithm::bubblesort;
use sorting_algorithm::mergesort;
use sorting_algorithm::quicksort;
use sorting_algorithm::insertionsort;
use sorting_algorithm::generate_random_array;

fn sort_arrays_benchmark(c: &mut Criterion) {
    let mut arr = black_box([-2,-7 ,2 ,10 ,20 ,9]);

    c.bench_function("Bubble Sort", |b| b.iter(|| bubblesort(&mut arr)));
    c.bench_function("Merge Sort", |b| b.iter(|| mergesort(&mut arr)));
    c.bench_function("Insertion Sort", |b| b.iter(|| insertionsort(&mut arr)));
    c.bench_function("Quick Sort", |b| b.iter(|| quicksort(&mut arr)));
}

fn sort_random_arrays_benchmark(c: &mut Criterion){
    let mut random_arr = black_box(generate_random_array(0..100, 10000));
    c.bench_function("Bubble Sort - Random array", |b| b.iter(|| bubblesort(&mut random_arr)));
    c.bench_function("Merge Sort - Random array", |b| b.iter(|| mergesort(&mut random_arr)));
    c.bench_function("Insertion Sort - Random array", |b| b.iter(|| insertionsort(&mut random_arr)));
    c.bench_function("Quick Sort - Random array", |b| b.iter(|| quicksort(&mut random_arr)));
}

criterion_group!(benches, sort_arrays_benchmark, sort_random_arrays_benchmark);
criterion_main!(benches);
