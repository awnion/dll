use std::collections::LinkedList;
use std::hint::black_box;

use criterion::BatchSize;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;

fn bench_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_only");
    for &n in &[1usize, 100, 200, 500, 1_000] {
        group.throughput(Throughput::Elements(n as u64));

        group.bench_with_input(BenchmarkId::new("std_linked_list", n), &n, |b, &n| {
            b.iter_batched(
                || n,
                |n| {
                    let mut list: LinkedList<usize> = LinkedList::new();
                    for i in 0..n {
                        list.push_back(black_box(i));
                    }
                    black_box(list.len());
                },
                BatchSize::SmallInput,
            )
        });

        // dll_raw: cannot call len(), ensure the list is used so pushes aren't elided
        group.bench_with_input(BenchmarkId::new("raw_pointer_linked_list", n), &n, |b, &n| {
            b.iter_batched(
                || n,
                |n| {
                    let mut list = dll::dll_raw::DLL::new();
                    for i in 0..n {
                        list.push(black_box(i));
                    }
                    black_box(&list);
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn bench_push_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_then_pop_all");
    for &n in &[1usize, 100, 200, 500, 1_000] {
        group.throughput(Throughput::Elements(n as u64));

        group.bench_with_input(BenchmarkId::new("std_linked_list", n), &n, |b, &n| {
            b.iter_batched(
                || n,
                |n| {
                    let mut list: LinkedList<usize> = LinkedList::new();
                    for i in 0..n {
                        list.push_back(black_box(i));
                    }
                    let mut cnt = 0usize;
                    while let Some(v) = list.pop_back() {
                        cnt = cnt.wrapping_add(black_box(v));
                    }
                    black_box(cnt);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("raw_pointer_linked_list", n), &n, |b, &n| {
            b.iter_batched(
                || n,
                |n| {
                    let mut list = dll::dll_raw::DLL::new();
                    for i in 0..n {
                        list.push(black_box(i));
                    }
                    let mut cnt = 0usize;
                    while let Some(v) = list.pop() {
                        cnt = cnt.wrapping_add(black_box(v));
                    }
                    black_box(cnt);
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, bench_push, bench_push_pop);
criterion_main!(benches);
