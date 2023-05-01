use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};
use rand::{distributions::Uniform, Rng};
use sorting_algorithm::{bubblesort, insertionsort, mergesort, quicksort, shellsort};

fn sorting_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");

    for i in [1,5,10, 25,50,75, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000] { // size of array  
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 100); // range of random numbers

        // Bubble sort benchmark (Time complexity: O(n^2))
        group.bench_function(BenchmarkId::new("Bubble Sort", i), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..i).map(|_| rng.sample(&range)).collect() }, // creates a vector of random numbers
                |v| bubblesort( v), // v is the vector/array to be sorted
                BatchSize::SmallInput, // BatchSize::SmallInput is the number of times the function is run
            )
        });
        // Merge sort benchmark (Time complexity: O(nlogn))
        group.bench_function(BenchmarkId::new("Merge Sort", i), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..i).map(|_| rng.sample(&range)).collect() },
                |v| mergesort (v),
                BatchSize::SmallInput,
            )
        });
        // Insertion sort benchmark (Time complexity: O(n^2))
        group.bench_function(BenchmarkId::new("Insertion Sort", i), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..i).map(|_| rng.sample(&range)).collect() },
                |v| insertionsort (v),
                BatchSize::SmallInput,
            )
        });
        // Quick sort benchmark (Time complexity: O(nlogn))
        group.bench_function(BenchmarkId::new("Quick Sort", i), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..i).map(|_| rng.sample(&range)).collect() },
                |v| quicksort (v),
                BatchSize::SmallInput,
            )
        });
        // Shell sort benchmark (Time complexity: O(nlogn))
        group.bench_function(BenchmarkId::new("Shell Sort", i), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..i).map(|_| rng.sample(&range)).collect() },
                |v| shellsort (v),
                BatchSize::SmallInput,
            )
        });

    }
    group.finish();
}

criterion_group!(benches, sorting_benchmarks);
criterion_main!(benches);
