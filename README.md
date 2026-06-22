# Subordinates – Two Algorithms in Rust

## Overview

Solves the Subordinates problem using two different approaches:
1. **Recursive DFS** – uses the system call stack
2. **Iterative DFS** – uses an explicit heap‑based stack

The goal is to count the number of subordinates (direct and indirect) for each employee in a company hierarchy.

---

## Algorithms

### 1. Recursive DFS (`subordinates_dfs`)

Traverses the tree from the root (employee 1). For each node, it recursively computes the size of its subtree. The number of subordinates is the subtree size minus 1.

**Time:** O(n) – visits each node once  
**Space:** O(n) – recursion stack 

### 2. Iterative DFS (`subordinates_iterative`)

Uses an explicit stack to perform a post‑order traversal. This avoids recursion overhead and is safe for very deep trees.

**Time:** O(n) – visits each node once  
**Space:** O(n) – stack and order list

---

## Benchmark

The benchmark runs both algorithms on **all CSES test cases**. It reads each `.in` file from `tests/inout/`, measures execution time for both algorithms, and prints a comparison table.

```bash
cargo run --release --bin bench
```

**Results(N=200000):**

| n | Greedy (ms) | Pattern (ms) | Diff (ms) |
|---|---|---|---|
| 200000 | 1.70 | 4.30 | -2.60 |

The recursive version is faster for large `n`, but it risks stack overflow on deep trees. The iterative version is safer and still fast.


---

## Why Recursive is Faster (But Risky)

### 1. Function Call Overhead

The iterative version uses a `Vec` as an explicit stack and a separate `order` list. This involves heap allocations and extra bookkeeping. The recursive version uses the system call stack, which is extremely fast because it's just moving a pointer.

### 2. Cache Behaviour

The recursive call stack is contiguous in memory, which is cache‑friendly. The iterative version's `Vec` and `order` list are also contiguous, but the extra operations (pushing/popping, writing to the order list) add overhead.

### 3. Memory Impact & Stack Safety

- **Recursive**: Uses the system stack, which is fast and has O(n) space, but it is limited. For a chain of 200,000 nodes, it can overflow.
- **Iterative**: Uses heap‑allocated `Vecs` for the stack and the `order` list. This avoids recursion limits, but the heap allocation and deallocation add overhead.

**Conclusion:** The recursive version is faster, but the iterative version is safer for deep trees. The iterative version is used to guarantee no stack overflow, while the recursive version is kept for benchmarking and comparison.

---

## How to Run

### Run the main program
```bash
echo "7" | cargo run
```

### Run the benchmark
```bash
cargo run --release --bin bench
```

### Run tests
```bash
cargo test
```

```