# DoubleLinkedList

A Rust doubly-linked list implementation that doesn't suck. Built for real-world use with proper error handling and performance optimizations.

[![Crates.io](https://img.shields.io/crates/v/double_linked_list.svg)](https://crates.io/crates/double_linked_list)
[![Documentation](https://docs.rs/double_linked_list/badge.svg)](https://docs.rs/double_linked_list/latest/double_linked_list/double_linked_list/struct.DoubleRinkedList.html)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
double_linked_list = "0.1.1"
```

Or install from the command line:

```bash
cargo add double_linked_list
```

## Why Another Linked List?

Vec is great for most things but terrible for frequent insertions/removals in the middle. This library gives you the best of both worlds - O(1) insertions anywhere with a cursor, plus a memory pool to reduce allocations. 

Honestly, I mostly wanted to explore smart pointer(Rc, Cell, RefCell, OnceCell, LazyCell, Weak) mechanisms in Rust, so this was more of an intellectual exercise than trying to build something actually useful.

## Features

- Actually fast (up to 1,270x faster than Vec for certain operations - [benchmarks here](BENCHMARKS.md))
- Proper error handling with `Result<T, ListError>` - no more panics
- Memory pool optimization for heavy workloads
- 50+ methods covering everything you'd expect
- Functional programming support (map, filter, reduce, etc.)
- Cursor system for efficient sequential operations
- Thread-safe ready (just wrap in `Arc<Mutex<>>`)
- Zero unsafe code

## Quick Start

```rust
use double_linked_list::DoubleLinkedList;

let mut list = DoubleLinkedList::new();
list.push(1)?;
list.push(2)?;
list.push(3)?;

let mut pooled_list = DoubleLinkedList::with_capacity(64);
pooled_list.push(42)?;
println!("Pool stats: {:?}", pooled_list.pool_stats());

use std::sync::{Arc, Mutex};
let shared_list = Arc::new(Mutex::new(DoubleLinkedList::new()));
```

For performance comparisons and when to use this vs Vec, check out the [benchmarks](BENCHMARKS.md).

## Documentation

**[Full API Documentation](https://docs.rs/double_linked_list/latest/double_linked_list/double_linked_list/struct.DoubleRinkedList.html)**

## API Reference

The API is pretty straightforward. Here's what you get:

### Basic Operations
```rust
fn new() -> Self
fn with_capacity(pool_capacity: usize) -> Self
fn len(&self) -> usize
fn is_empty(&self) -> bool
fn clear(&mut self)
fn pool_stats(&self) -> Option<(usize, usize)>
fn add_capacity(&mut self, additional: usize)
fn set_capacity(&mut self, new_capacity: usize) -> Result<()>
```

### Adding/Removing Items
```rust
fn push(&mut self, value: T) -> Result<usize>
fn push_back(&mut self, value: T) -> Result<usize>
fn push_front(&mut self, value: T) -> Result<usize>
fn pop(&mut self) -> Option<T>
fn pop_back(&mut self) -> Option<T>
fn pop_front(&mut self) -> Option<T>
fn insert_at_index(&mut self, index: usize, value: T) -> Result<()>
fn insert_many_at_index<I>(&mut self, index: usize, values: I) -> Result<usize>
fn insert_at_begin(&mut self, value: T) -> Result<()>
fn insert_at_end(&mut self, value: T) -> Result<()>
fn remove_at_index(&mut self, index: usize) -> Result<T>
fn remove_at_begin(&mut self) -> Result<T>
fn remove_at_end(&mut self) -> Result<T>
fn get_at_index(&self, index: usize) -> Result<T>
fn get_at_begin(&self) -> Option<T>
fn get_at_end(&self) -> Option<T>
fn first(&self) -> Option<T>
fn last(&self) -> Option<T>
fn front(&self) -> Option<T>
fn back(&self) -> Option<T>
```

### Searching
```rust
fn contains(&self, value: &T) -> bool
fn includes(&self, value: &T) -> bool
fn find<F>(&self, predicate: F) -> Option<T>
fn find_index<F>(&self, predicate: F) -> Option<usize>
fn index_of(&self, value: &T) -> Option<usize>
```

### Functional Programming
```rust
fn map<U, F>(&self, f: F) -> DoubleLinkedList<U>
fn filter<F>(&self, predicate: F) -> Self
fn reduce<U, F>(&self, f: F, initial: U) -> U
fn every<F>(&self, predicate: F) -> bool
fn any<F>(&self, predicate: F) -> bool
fn some<F>(&self, predicate: F) -> bool
fn for_each<F>(&self, f: F)
```

### Other Useful Stuff
```rust
fn reverse(&mut self)
fn splice(&mut self, index: usize, delete_count: usize, items: Vec<T>) -> Result<Vec<T>>
fn sort(&mut self)
fn sort_by<F>(&mut self, compare: F)
fn is_sorted(&self) -> bool
fn is_sorted_by<F>(&self, compare: F) -> bool
fn dedup(&mut self)
fn dedup_by<F>(&mut self, same_bucket: F)
fn swap(&mut self, a: usize, b: usize) -> Result<()>
fn retain<F>(&mut self, predicate: F)
fn remove_item(&mut self, value: &T) -> bool
fn remove_all(&mut self, value: &T) -> usize
fn extend<I>(&mut self, iter: I)
fn append(&mut self, other: &mut Self)
fn split_off(&mut self, at: usize) -> Result<Self>
fn split_at(&mut self, mid: usize) -> Result<(Vec<T>, Vec<T>)>
fn to_vec(&self) -> Vec<T>
fn to_vec_reversed(&self) -> Vec<T>
fn from_slice(slice: &[T]) -> Self
```

### Cursor System

The cursor is what makes this list fast for sequential operations. Think of it as a bookmark in the list:

```rust
fn move_cursor_at_begin(&mut self) -> Result<()>
fn move_cursor_at_end(&mut self) -> Result<()>
fn move_cursor_at_index(&mut self, index: usize) -> Result<()>
fn move_cursor_to_value(&mut self, value: &T) -> Result<()>
fn move_cursor_to_next(&mut self) -> Result<()>
fn move_cursor_to_previous(&mut self) -> Result<()>
fn get_cursor_index(&self) -> Option<usize>
fn cursor_position(&self) -> usize
fn cursor_is_at_begin(&self) -> bool
fn cursor_is_at_end(&self) -> bool
fn can_move_cursor_next(&self) -> bool
fn can_move_cursor_previous(&self) -> bool
fn insert_after_cursor(&mut self, value: T) -> Result<()>
fn insert_before_cursor(&mut self, value: T) -> Result<()>
fn remove_after_cursor(&mut self) -> Result<T>
fn remove_before_cursor(&mut self) -> Result<T>
fn get_at_cursor(&self) -> Option<T>
fn get_before_cursor(&self) -> Option<T>
fn get_cursor(&self) -> Cursor<T>
fn set_cursor(&mut self, cursor: Cursor<T>) -> Result<()>
fn reset_cursor(&mut self)
fn validate_cursor(&self) -> bool
```

### Display & Debug
```rust
fn display(&self, separator: Option<&str>)
fn log(&self, separator: Option<&str>)
fn enable_debug(&mut self)
fn disable_debug(&mut self)
fn is_debug_enabled(&self) -> bool
```

## Error Handling

No more panics. Operations that can fail return `Result<T, ListError>`:

```rust
match list.get_at_index(100) {
    Ok(value) => println!("Got: {}", value),
    Err(ListError::IndexOutOfBounds { index, length }) => {
        println!("Index {} is out of bounds (length: {})", index, length);
    },
    Err(ListError::EmptyList) => println!("List is empty"),
}
```

## Memory Pool

If you're creating/destroying lots of nodes, use the memory pool:

```rust
let mut list = DoubleLinkedList::with_capacity(100);

if let Some((available, total)) = list.pool_stats() {
    println!("Pool: {}/{} nodes available", available, total);
}
```

## Examples

### Basic stuff
```rust
let mut list = DoubleLinkedList::new();

list.push(1)?;
list.push(2)?;
list.push(3)?;

assert_eq!(list.len(), 3);
assert_eq!(list.first(), Some(1));
assert_eq!(list.last(), Some(3));

let item = list.pop();
let front_item = list.pop_front();
```

### Functional programming
```rust
let numbers: DoubleLinkedList<i32> = (1..=5).collect();

let doubled = numbers.map(|x| x * 2);
let evens = numbers.filter(|x| x % 2 == 0);
let sum = numbers.reduce(|acc, x| acc + x, 0);

assert!(numbers.every(|x| *x > 0));
assert!(numbers.any(|x| *x > 3));
```

### Bulk insertions
```rust
let mut list: DoubleLinkedList<i32> = (1..=5).collect();

// Insert multiple values at once
let values = vec![100, 200, 300];
let inserted_count = list.insert_many_at_index(2, values)?;
println!("Inserted {} items", inserted_count);

// Works with any iterator
let more_values = (400..=500);
list.insert_many_at_index(0, more_values)?;
```

### Cursor operations (the cool part)
```rust
let mut list: DoubleLinkedList<i32> = (1..=5).collect();

list.move_cursor_at_index(2)?;
list.insert_after_cursor(100)?;
list.move_cursor_to_next()?;
list.insert_after_cursor(200)?;
```

### Thread safety
```rust
use std::sync::{Arc, Mutex};

let shared_list = Arc::new(Mutex::new(DoubleLinkedList::new()));

let list_clone = Arc::clone(&shared_list);
std::thread::spawn(move || {
    list_clone.lock().unwrap().push(42).unwrap();
});
```

### Creating from iterators
```rust
let list: DoubleLinkedList<i32> = (1..=10).collect();

let vec = vec![1, 2, 3];
let list: DoubleLinkedList<i32> = vec.into_iter().collect();
```

## Performance

Most operations are O(1) or O(n). Cursor operations are O(1) after positioning. The memory pool helps with allocation performance.

Check out [BENCHMARKS.md](BENCHMARKS.md) for detailed comparisons with Vec and std::collections::LinkedList.

## How it works

It's a circular doubly-linked list with a sentinel node. The cursor sits between nodes and lets you do O(1) insertions/removals at any position once you're there.

```
Root ↔ Node1 ↔ Node2 ↔ Node3 ↔ Root
                 ↑
               cursor
```

Based on my JavaScript version at [touskar/doubly_linked_list](https://github.com/touskar/doubly_linked_list), but with proper Rust error handling and memory management.

## License

MIT