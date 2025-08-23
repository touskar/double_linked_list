use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use double_linked_list::DoubleRinkedList;
use std::collections::LinkedList;

#[derive(Clone, Debug)]
struct LargeData {
    data: [u64; 32],
}

impl LargeData {
    fn new(val: u64) -> Self {
        LargeData { data: [val; 32] }
    }
}

#[derive(Clone, Debug)]
struct HugeData {
    data: [u64; 64],
}

impl HugeData {
    fn new(val: u64) -> Self {
        HugeData { data: [val; 64] }
    }
}

fn bench_push_front_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_front_100");
    
    group.bench_function("Vec", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..100 {
                vec.insert(0, i);
            }
            black_box(vec);
        });
    });
    
    group.bench_function("LinkedList", |b| {
        b.iter(|| {
            let mut list = LinkedList::new();
            for i in 0..100 {
                list.push_front(i);
            }
            black_box(list);
        });
    });
    
    group.bench_function("DoubleRinkedList", |b| {
        b.iter(|| {
            let mut list = DoubleRinkedList::new();
            for i in 0..100 {
                let _ = list.push_front(i);
            }
            black_box(list);
        });
    });
    
    group.finish();
}

fn bench_push_front_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_front_scaling");
    
    for size in [1000, 5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    vec.insert(0, i);
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("LinkedList", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_front(i);
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let _ = list.push_front(i);
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_pop_front_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_front_scaling");
    
    for size in [1000, 5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut vec = Vec::new();
                    for i in 0..size {
                        vec.push(i);
                    }
                    vec
                },
                |mut vec| {
                    while !vec.is_empty() {
                        vec.remove(0);
                    }
                    black_box(vec);
                }
            );
        });
        
        group.bench_with_input(BenchmarkId::new("LinkedList", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut list = LinkedList::new();
                    for i in 0..size {
                        list.push_back(i);
                    }
                    list
                },
                |mut list| {
                    while !list.is_empty() {
                        list.pop_front();
                    }
                    black_box(list);
                }
            );
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut list = DoubleRinkedList::new();
                    for i in 0..size {
                        let _ = list.push(i);
                    }
                    list
                },
                |mut list| {
                    while !list.is_empty() {
                        list.pop_front();
                    }
                    black_box(list);
                }
            );
        });
    }
    group.finish();
}

fn bench_middle_insertions(c: &mut Criterion) {
    let mut group = c.benchmark_group("middle_insertions");
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    let pos = if vec.is_empty() { 0 } else { vec.len() / 2 };
                    vec.insert(pos, i);
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let pos = list.len() / 2;
                    let _ = list.insert_at_index(pos, i);
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_middle_removals(c: &mut Criterion) {
    let mut group = c.benchmark_group("middle_removals");
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut vec = Vec::new();
                    for i in 0..size {
                        vec.push(i);
                    }
                    vec
                },
                |mut vec| {
                    while vec.len() > size / 2 {
                        let pos = vec.len() / 2;
                        vec.remove(pos);
                    }
                    black_box(vec);
                }
            );
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut list = DoubleRinkedList::new();
                    for i in 0..size {
                        let _ = list.push(i);
                    }
                    list
                },
                |mut list| {
                    while list.len() > size / 2 {
                        let pos = list.len() / 2;
                        let _ = list.remove_at_index(pos);
                    }
                    black_box(list);
                }
            );
        });
    }
    group.finish();
}

fn bench_large_elements_256b(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_elements_256B");
    
    for size in [50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::new("Vec_push_front", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    vec.insert(0, LargeData::new(i as u64));
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList_push_front", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let _ = list.push_front(LargeData::new(i as u64));
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_large_elements_512b_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_elements_512B_scaling");
    
    for size in [500, 1000, 2000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec_push_front", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    vec.insert(0, HugeData::new(i as u64));
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList_push_front", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let _ = list.push_front(HugeData::new(i as u64));
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("Vec_middle_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    let pos = if vec.is_empty() { 0 } else { vec.len() / 2 };
                    vec.insert(pos, HugeData::new(i as u64));
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList_middle_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let pos = list.len() / 2;
                    let _ = list.insert_at_index(pos, HugeData::new(i as u64));
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_mixed_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_operations");
    
    group.bench_function("Vec", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..100 {
                match i % 4 {
                    0 => vec.insert(0, i),
                    1 => { if !vec.is_empty() { vec.remove(0); } },
                    2 => vec.push(i),
                    _ => { if !vec.is_empty() { let pos = vec.len() / 2; vec.insert(pos, i); } }
                }
            }
            black_box(vec);
        });
    });
    
    group.bench_function("LinkedList", |b| {
        b.iter(|| {
            let mut list = LinkedList::new();
            for i in 0..100 {
                match i % 4 {
                    0 => list.push_front(i),
                    1 => { list.pop_front(); },
                    2 => list.push_back(i),
                    _ => list.push_back(i),
                }
            }
            black_box(list);
        });
    });
    
    group.bench_function("DoubleRinkedList", |b| {
        b.iter(|| {
            let mut list = DoubleRinkedList::new();
            for i in 0..100 {
                match i % 4 {
                    0 => { let _ = list.push_front(i); },
                    1 => { list.pop_front(); },
                    2 => { let _ = list.push(i); },
                    _ => { if !list.is_empty() { let pos = list.len() / 2; let _ = list.insert_at_index(pos, i); } }
                }
            }
            black_box(list);
        });
    });
    
    group.finish();
}

fn bench_cursor_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cursor_operations");
    
    group.bench_function("DoubleRinkedList_cursor_insertions", |b| {
        b.iter_with_setup(
            || {
                let mut list = DoubleRinkedList::new();
                for i in 0..100 {
                    let _ = list.push(i);
                }
                list
            },
            |mut list| {
                let _ = list.move_cursor_at_index(50);
                for i in 0..20 {
                    let _ = list.insert_after_cursor(1000 + i);
                    let _ = list.move_cursor_to_next();
                }
                black_box(list);
            }
        );
    });
    
    group.bench_function("Vec_equivalent", |b| {
        b.iter_with_setup(
            || {
                let mut vec = Vec::new();
                for i in 0..100 {
                    vec.push(i);
                }
                vec
            },
            |mut vec| {
                let mut pos = 50;
                for i in 0..20 {
                    vec.insert(pos, 1000 + i);
                    pos += 1;
                }
                black_box(vec);
            }
        );
    });
    
    group.finish();
}

fn bench_memory_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool");
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList_no_pool", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for _cycle in 0..10 {
                    for i in 0..size {
                        let _ = list.push(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList_with_pool", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::with_capacity(size);
                for _cycle in 0..10 {
                    for i in 0..size {
                        let _ = list.push(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("LinkedList_baseline", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for _cycle in 0..10 {
                    for i in 0..size {
                        list.push_back(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_memory_pool_large_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool_large_scale");
    
    for size in [5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList_no_pool", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for _cycle in 0..10 {
                    for i in 0..(size / 10) {
                        let _ = list.push(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleRinkedList_with_pool", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::with_capacity(size / 10);
                for _cycle in 0..10 {
                    for i in 0..(size / 10) {
                        let _ = list.push(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_push_front_large_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_front_large_scaling");
    
    for size in [1000, 5000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    vec.insert(0, i);
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("LinkedList", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_front(i);
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleLinkedList", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let _ = list.push_front(i);
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_pop_front_large_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_front_large_scaling");
    
    for size in [1000, 5000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut vec = Vec::new();
                    for i in 0..size {
                        vec.push(i);
                    }
                    vec
                },
                |mut vec| {
                    while !vec.is_empty() {
                        vec.remove(0);
                    }
                    black_box(vec);
                }
            );
        });
        
        group.bench_with_input(BenchmarkId::new("LinkedList", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut list = LinkedList::new();
                    for i in 0..size {
                        list.push_back(i);
                    }
                    list
                },
                |mut list| {
                    while !list.is_empty() {
                        list.pop_front();
                    }
                    black_box(list);
                }
            );
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleLinkedList", size), size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut list = DoubleRinkedList::new();
                    for i in 0..size {
                        let _ = list.push(i);
                    }
                    list
                },
                |mut list| {
                    while !list.is_empty() {
                        list.pop_front();
                    }
                    black_box(list);
                }
            );
        });
    }
    group.finish();
}

fn bench_middle_operations_large_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("middle_insertions_large_scaling");
    
    for size in [1000, 5000, 10000, 50000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    let pos = if vec.is_empty() { 0 } else { vec.len() / 2 };
                    vec.insert(pos, i);
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleLinkedList", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let pos = list.len() / 2;
                    let _ = list.insert_at_index(pos, i);
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_large_elements_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_elements_512B_large_scaling");
    
    for size in [1000, 5000, 10000, 25000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec_push_front", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    vec.insert(0, HugeData::new(i as u64));
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleLinkedList_push_front", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let _ = list.push_front(HugeData::new(i as u64));
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("Vec_middle_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    let pos = if vec.is_empty() { 0 } else { vec.len() / 2 };
                    vec.insert(pos, HugeData::new(i as u64));
                }
                black_box(vec);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleLinkedList_middle_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for i in 0..size {
                    let pos = list.len() / 2;
                    let _ = list.insert_at_index(pos, HugeData::new(i as u64));
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

fn bench_memory_pool_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool_scaling");
    
    for size in [1000, 5000, 10000, 50000].iter() {
        group.bench_with_input(BenchmarkId::new("DoubleLinkedList_no_pool", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::new();
                for _cycle in 0..10 {
                    for i in 0..size {
                        let _ = list.push(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("DoubleLinkedList_with_pool", size), size, |b, &size| {
            b.iter(|| {
                let mut list = DoubleRinkedList::with_capacity(size);
                for _cycle in 0..10 {
                    for i in 0..size {
                        let _ = list.push(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("LinkedList_baseline", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for _cycle in 0..10 {
                    for i in 0..size {
                        list.push_back(i);
                    }
                    list.clear();
                }
                black_box(list);
            });
        });
    }
    group.finish();
}

criterion_group!(
    basic_benches,
    bench_push_front_small,
    bench_middle_insertions,
    bench_large_elements_256b,
    bench_mixed_operations,
    bench_cursor_operations,
    bench_memory_pool
);

criterion_group!(
    scaling_benches,
    bench_push_front_scaling,
    bench_pop_front_scaling,
    bench_middle_removals,
    bench_large_elements_512b_scaling,
    bench_memory_pool_large_scale,
    bench_push_front_large_scaling,
    bench_pop_front_large_scaling,
    bench_middle_operations_large_scaling,
    bench_large_elements_scaling,
    bench_memory_pool_scaling
);

criterion_main!(basic_benches, scaling_benches);