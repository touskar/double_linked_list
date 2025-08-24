# DoubleRinkedList Benchmark Results

## System Information
- Platform: macOS Darwin 24.1.0
- Date: 2025-08-24
- Rust: Optimized + debuginfo build

## Executive Summary

DoubleRinkedList demonstrates strong performance characteristics for specific use cases:
- **O(1) push_front operations** - Competitive with LinkedList, vastly superior to Vec at scale
- **Large element handling** - 27x faster than Vec for 512B structs at 10K elements
- **Memory efficiency concerns** - Higher overhead than LinkedList (2-3x slower in many cases)

## Key Performance Metrics

### Push Front Operations (100K elements)
| Structure | Time | Relative Performance |
|-----------|------|---------------------|
| Vec | 289.95ms | Baseline (O(n²) complexity) |
| LinkedList | 1.37ms | 211x faster than Vec |
| DoubleRinkedList | 3.14ms | 92x faster than Vec |

### Middle Insertions (50K elements)
| Structure | Time | Notes |
|-----------|------|-------|
| Vec | 23.48ms | O(n) per operation |
| DoubleRinkedList | 3.86s | 164x slower - index traversal overhead |

### Large Elements (512B, 25K elements)
| Operation | Vec | DoubleRinkedList | Improvement |
|-----------|-----|------------------|-------------|
| Push Front | 2.25s | 3.21ms | **700x faster** |
| Middle Insert | 1.08s | 968ms | 1.1x faster |

## Detailed Results

### Small Scale Operations (100 elements)

#### Push Front
- **Vec**: 588.71ns
- **LinkedList**: 1.16µs (1.97x slower than Vec)
- **DoubleRinkedList**: 3.30µs (5.61x slower than Vec)

*Analysis*: At small scales, Vec benefits from cache locality despite O(n) complexity.

### Scaling Behavior

#### Push Front Scaling
| Size | Vec | LinkedList | DoubleRinkedList | DRL vs Vec |
|------|-----|------------|------------------|------------|
| 1K | 20.42µs | 12.97µs | 36.53µs | 1.79x slower |
| 5K | 471.32µs | 65.11µs | 192.41µs | 2.45x faster |
| 10K | 1.89ms | 135.68µs | 388.87µs | 4.86x faster |
| 100K | 289.95ms | 1.37ms | 3.14ms | **92x faster** |

*Trend*: DoubleRinkedList advantage grows quadratically with size due to Vec's O(n²) total complexity.

#### Pop Front Scaling
| Size | Vec | LinkedList | DoubleRinkedList | DRL vs Vec |
|------|-----|------------|------------------|------------|
| 1K | 20.05µs | 7.78µs | 17.03µs | 1.18x faster |
| 5K | 454.31µs | 38.50µs | 87.57µs | 5.19x faster |
| 10K | 1.70ms | 77.12µs | 181.24µs | 9.38x faster |
| 100K | 271.87ms | 784.62µs | 1.71ms | **159x faster** |

### Middle Operations Performance

#### Middle Insertions (Critical Performance Issue)
| Size | Vec | DoubleRinkedList | Ratio |
|------|-----|------------------|-------|
| 1K | 11.09µs | 324.08µs | 29x slower |
| 5K | 232.29µs | 18.65ms | 80x slower |
| 10K | 913.32µs | 135.39ms | 148x slower |
| 50K | 23.48ms | **3.86s** | **164x slower** |

*Critical Issue*: O(n) index traversal for each operation creates O(n²) total complexity.

### Large Element Benchmarks (512B structs)

#### Push Front with Large Elements
| Size | Vec | DoubleRinkedList | Improvement |
|------|-----|------------------|-------------|
| 1K | 3.51ms | 122.84µs | 28.6x faster |
| 5K | 89.31ms | 646.83µs | 138x faster |
| 10K | 358.08ms | 1.31ms | 273x faster |
| 25K | 2.25s | 3.21ms | **700x faster** |

*Key Insight*: Memory copying dominates Vec performance with large elements.

#### Middle Insert with Large Elements
| Size | Vec | DoubleRinkedList | Performance |
|------|-----|------------------|-------------|
| 1K | 1.59ms | 415.91µs | 3.8x faster |
| 5K | 44.75ms | 23.48ms | 1.9x faster |
| 10K | 179.19ms | 124.09ms | 1.4x faster |
| 25K | 1.08s | 968.41ms | 1.1x faster |

### Memory Pool Performance

#### Allocation/Deallocation Cycles (10 iterations)
| Size | No Pool | With Pool | LinkedList | Pool Benefit |
|------|---------|-----------|------------|--------------|
| 1K | 325.93µs | 354.52µs | 132.80µs | -8.8% (worse) |
| 5K | 1.94ms | 2.02ms | 712.00µs | -4.1% (worse) |
| 10K | 3.80ms | 4.00ms | 1.41ms | -5.3% (worse) |
| 50K | 21.95ms | 22.23ms | 7.53ms | -1.3% (worse) |

*Finding*: Memory pool shows no benefit, possibly due to Rc overhead.

### Cursor Operations

- **DoubleRinkedList cursor insertions**: 2.38µs (20 sequential inserts)
- **Vec equivalent**: 126.24ns (18.9x faster)

*Note*: Cursor maintains position, avoiding repeated traversals.

### Mixed Operations (100 ops)
- **Vec**: 391.39ns
- **LinkedList**: 942.06ns (2.4x slower than Vec)
- **DoubleRinkedList**: 3.08µs (7.9x slower than Vec)

## Performance Recommendations

### Use DoubleRinkedList When:
1. **Frequent push_front/pop_front** with >5K elements
2. **Large elements** (>256B) with front operations
3. **Cursor-based sequential modifications**

### Avoid DoubleRinkedList When:
1. **Random access by index** - O(n) traversal is prohibitive
2. **Small datasets** (<1K elements) - overhead dominates
3. **Simple push_back operations** - Vec is optimal

### Critical Issues to Address:
1. **Middle operations**: O(n²) complexity at scale (3.86s for 50K)
2. **Memory pool**: No performance benefit, adds complexity
3. **Small scale overhead**: 5.6x slower than Vec for 100 elements

## Comparison Matrix

| Operation | Best Choice | Second Choice | Avoid |
|-----------|------------|---------------|-------|
| Push Front (>10K) | LinkedList | DoubleRinkedList | Vec |
| Push Front (<1K) | Vec | LinkedList | DoubleRinkedList |
| Random Index Access | Vec | - | DoubleRinkedList |
| Large Elements Front | DoubleRinkedList | LinkedList | Vec |
| Memory Efficiency | LinkedList | Vec | DoubleRinkedList |
| Sequential Cursor Ops | DoubleRinkedList | - | Vec |

## Conclusion

DoubleRinkedList excels in specific scenarios (large-scale front operations, large elements) but suffers from:
- High overhead for small operations (3-5x vs LinkedList)
- Catastrophic O(n²) performance for indexed operations at scale
- Ineffective memory pool optimization
- Generally 2-3x slower than std::collections::LinkedList

The implementation needs optimization for index-based operations and memory management to be competitive as a general-purpose data structure.