use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day5::{solve_part2, solve_part2_with_min_max, solve_part2_vector};
use std::io::Read;
use std::fs::File;

fn bench_solve_part2(c: &mut Criterion) {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day5 part2", |b| b.iter(|| solve_part2(&input)));
}

fn bench_solve_part2_min_max(c: &mut Criterion) {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day5 part2 mix max", |b| b.iter(|| solve_part2_with_min_max(&input)));
}

fn bench_solve_part2_vector(c: &mut Criterion) {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day5 part2 vector", |b| b.iter(|| solve_part2_vector(&input)));
}

criterion_group!(benches, bench_solve_part2, bench_solve_part2_min_max, bench_solve_part2_vector);
criterion_main!(benches);
