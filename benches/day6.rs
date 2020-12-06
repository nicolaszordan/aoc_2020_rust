use aoc_2020::day6::{solve_part1, solve_part1_iter_over_alpha, solve_part2, solve_part2_iter_over_alpha};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::Read;

fn bench_day6_solve_part1(c: &mut Criterion) {
    let mut file = File::open("input/2020/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day6 part1", |b| b.iter(|| solve_part1(&input)));
}

fn bench_day6_solve_part1_iter_over_alpha(c: &mut Criterion) {
    let mut file = File::open("input/2020/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day6 part1 iter over alpha", |b| {
        b.iter(|| solve_part1_iter_over_alpha(&input))
    });
}

fn bench_day6_solve_part2(c: &mut Criterion) {
    let mut file = File::open("input/2020/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day6 part2", |b| b.iter(|| solve_part2(&input)));
}

fn bench_day6_solve_part2_iter_over_alpha(c: &mut Criterion) {
    let mut file = File::open("input/2020/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    c.bench_function("day6 part2 iter over alpha", |b| {
        b.iter(|| solve_part2_iter_over_alpha(&input))
    });
}

criterion_group!(
    day6_benches,
    bench_day6_solve_part1,
    bench_day6_solve_part1_iter_over_alpha,
    bench_day6_solve_part2,
    bench_day6_solve_part2_iter_over_alpha,
);
criterion_main!(day6_benches);
