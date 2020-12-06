use aoc_2020::day5::{solve_part2, solve_part2_vector, solve_part2_with_min_max};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::Read;

fn bench_day5_solve_part2(c: &mut Criterion) {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day5 part2", |b| b.iter(|| solve_part2(&input)));
}

fn bench_day5_solve_part2_min_max(c: &mut Criterion) {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day5 part2 mix max", |b| {
        b.iter(|| solve_part2_with_min_max(&input))
    });
}

fn bench_day5_solve_part2_vector(c: &mut Criterion) {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day5 part2 vector", |b| {
        b.iter(|| solve_part2_vector(&input))
    });
}

criterion_group!(
    day5_benches,
    bench_day5_solve_part2,
    bench_day5_solve_part2_min_max,
    bench_day5_solve_part2_vector
);
criterion_main!(day5_benches);
