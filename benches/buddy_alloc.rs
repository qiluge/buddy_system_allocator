#[macro_use]
extern crate criterion;

use std::alloc::Layout;

use criterion::{Criterion, Throughput};

use buddy_system_allocator::Heap;

const HEAP_SIZE: usize = 64 * 1024 * 1024;
const ALLOC_SIZE: usize = 32 * 1024 * 1024;
// const LEAF_SIZE: usize = 16;
const ORDER: usize = 32;

fn bench_alloc(alloc_size: usize) {
    let buf: Vec<u8> = Vec::with_capacity(HEAP_SIZE);
    unsafe {
        let mut allocator = Heap::<ORDER>::new();
        allocator.init(buf.as_ptr() as usize, HEAP_SIZE);
        for _i in 0..(ALLOC_SIZE / alloc_size) {
            allocator.alloc(Layout::from_size_align(alloc_size, 1).unwrap()).unwrap();
        }
    }
}

fn bench_alloc_then_free(alloc_size: usize) {
    let buf: Vec<u8> = Vec::with_capacity(HEAP_SIZE);
    unsafe {
        let mut allocator = Heap::<ORDER>::new();
        allocator.init(buf.as_ptr() as usize, HEAP_SIZE);
        let count = ALLOC_SIZE / alloc_size;
        let mut ptrs = Vec::with_capacity(count);
        for _i in 0..count {
            ptrs.push(allocator.alloc(Layout::from_size_align(alloc_size, 1).unwrap()).unwrap());
        }
        for _i in 0..count {
            allocator.dealloc(ptrs.pop().unwrap(), Layout::from_size_align(alloc_size, 1).unwrap());
        }
    }
}


fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("alloc");
    for &size in &[16, 32, 64, 128] {
        let count = ALLOC_SIZE / size;
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(format!("{} Bytes", size), &size, |b, &size| {
            b.iter(|| bench_alloc(size));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("alloc then free");
    for &size in &[16, 32, 64, 128] {
        let count = ALLOC_SIZE / size;
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(format!("{} Bytes", size), &size, |b, &size| {
            b.iter(|| bench_alloc_then_free(size));
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = bench
);
criterion_main!(benches);
