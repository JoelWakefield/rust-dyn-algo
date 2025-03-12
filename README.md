# Rust Dyn Algo

This is the library for the `rust_dyn_algo` (AKA: dynamic algorithms). It is a simple library, mostly for practice with rust and algorithms.

For a more extensive library, please see refer to [The Algorithms](https://github.com/TheAlgorithms/Rust) or [Tianyi Shi](https://github.com/TianyiShi2001/Algorithms).

## Algorithms

The algorithms are listed by type.

### Fibonacci

Taking the "bottom-up" approach, this utilises a for loop to iterate and a hashmap for storing precalculated values. It only calculates results up to the 186th order, values beyond this exceed [`u128::MAX`](https://doc.rust-lang.org/std/u128/constant.MAX.html).
