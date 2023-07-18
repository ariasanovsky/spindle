use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rayon::prelude::IntoParallelRefMutIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[spindle::basic_range]
fn identity(x: i32) -> i32 {
    x
}

fn rayon_vec(x: i32) -> Vec<i32> {
    (0..x).into_par_iter().collect()
}

fn bench_square_over_two(c: &mut Criterion) {
    let mut group = c.benchmark_group("filling_boxed_array");
    for n in [10_000i32, 100_000, 1_000_000, 10_000_000, 100_000_000] {
        group.bench_with_input(
            BenchmarkId::new("spindle identity", n),
            &n,
            |b, &n| {
                b.iter(||black_box(
                    unsafe { n.identity() }.unwrap()
                ))
            }
        );

        group.bench_with_input(
            BenchmarkId::new("rayon identity", n),
            &n,
            |b, &n| {
                b.iter(||black_box(rayon_vec(n)))
            }
        );
    }
}

criterion_group!(benches, bench_square_over_two);
criterion_main!(benches);
