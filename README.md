# Data Race in Rust
This repository demonstrates a data race in Rust using mutable references.  The `bug.rs` file contains the buggy code, while `bugSolution.rs` shows how to fix it.

Data races occur when multiple threads access the same memory location concurrently, at least one of which is writing.  In Rust, the borrow checker prevents many data races at compile time, but sophisticated patterns or unsafe code can still lead to them.

## How to Reproduce
1. Clone this repository.
2. Navigate to the directory.
3. Run `rustc bug.rs && ./bug`
4. Observe that the output may not be deterministic and depends on the compiler and its optimization settings.

## Solution
The `bugSolution.rs` file shows a correct approach, using techniques such as using `RefCell` or `Mutex`  to enforce mutual exclusion, allowing only one thread access to a shared mutable resource at a time. This ensures that data access is consistent and predictable.