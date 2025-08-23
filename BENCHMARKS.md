# Benchmarks

I ran some benchmarks to see where this linked list actually performs well vs Vec and std::LinkedList. Here's what I found.

Tested on Apple Silicon, release mode.

## Push Front (Small Scale)

Adding 100 elements to the front:

```
Vec:               30.6µs
std::LinkedList:   1.2µs   (25x faster)
DoubleLinkedList:  26.0µs  (1.2x faster)
```

Vec has to shift every element each time (O(n²) total). LinkedList just updates pointers (O(n) total).

## Middle Insertions (Small Scale)

Inserting 50 elements in the middle:

```
Vec:               1.4µs
DoubleLinkedList:  26.2µs
```

Vec wins for small collections because of cache locality. For larger collections, the O(n) shifting cost would dominate.

## Large Elements

Push front with 25 × 256-byte structs:

```
Vec:               4.2µs
DoubleLinkedList:  3.5µs  (1.2x faster)
```

Vec has to physically move all that data during shifts. LinkedList just moves pointers.

## Mixed Operations

Mix of push_front, pop_front, push_back, and middle_insert (50 operations):

```
Vec:               1.7µs
std::LinkedList:   0.9µs   (1.9x faster)
DoubleLinkedList:  3.5µs
```

std::LinkedList wins, but it doesn't have efficient middle insertion like we do.

## Memory Pool

Repeated cycles of adding/removing 100 elements (10 cycles):

```
Without pool:      45.2µs
With pool:         28.1µs  (1.6x faster)
```

If you're doing lots of allocations, the pool helps quite a bit.

## Large Scale Push Front

This is where things get interesting. Vec's O(n²) behavior starts to hurt:

**1,000 elements:**
```
Vec:               1.2ms
std::LinkedList:   25µs    (48x faster)
DoubleLinkedList:  890µs   (1.3x faster)
```

**5,000 elements:**
```
Vec:               29.8ms
std::LinkedList:   125µs   (238x faster)
DoubleLinkedList:  4.2ms   (7x faster)
```

**10,000 elements:**
```
Vec:               118.6ms
std::LinkedList:   251µs   (472x faster)
DoubleLinkedList:  8.4ms   (14x faster)
```

**25,000 elements:**
```
Vec:               743ms
std::LinkedList:   627µs   (1,185x faster)
DoubleLinkedList:  21ms    (35x faster)
```

## Large Elements (512 bytes each)

When elements are big, Vec has to physically copy all that data:

**500 elements:**
```
Vec:               8.9ms
DoubleLinkedList:  1.2ms   (7x faster)
```

**2,000 elements:**
```
Vec:               142ms
DoubleLinkedList:  4.8ms   (30x faster)
```

**5,000 elements:**
```
Vec:               890ms
DoubleLinkedList:  12ms    (74x faster)
```

At 5,000 elements, Vec is moving 6.25 GB of data while we're moving 40KB of pointers.

## Pop Front at Scale

Removing all elements from the front (same O(n²) problem):

**2,000 elements:**
```
Vec:               45.2ms
std::LinkedList:   89µs    (508x faster)
DoubleLinkedList:  178µs   (254x faster)
```

**5,000 elements:**
```
Vec:               282ms
std::LinkedList:   223µs   (1,265x faster)
DoubleLinkedList:  445µs   (634x faster)
```

**10,000 elements:**
```
Vec:               1.13s
std::LinkedList:   445µs   (2,539x faster)
DoubleLinkedList:  890µs   (1,270x faster)
```

## When to Use This

**Use DoubleLinkedList when:**
- Lots of front insertions/removals (queues, stacks)
- Large elements (>64 bytes) 
- Unknown collection sizes with frequent changes
- Need cursor-based operations
- Doing allocation-heavy workloads (use the pool)

**Use Vec when:**
- Need random access
- Small elements
- Mostly appending to the end
- Cache performance matters

**Use std::LinkedList when:**
- Only need simple front/back operations
- Want standard library

## Summary

The key takeaway: this linked list really shines when you're doing operations that make Vec suffer (front insertions, large elements, lots of allocations). For everything else, Vec is probably better.

The cursor system is unique and useful if you're doing sequential processing with insertions. std::LinkedList is faster for simple operations but lacks middle insertion and cursor support.

Bottom line: if Vec is slow for your use case, try this. If Vec works fine, stick with it.