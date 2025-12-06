# Rust String Concatenation Benchmarks

> **Note:** This project was originally forked from [Hendrik Sollich's concatenation_benchmarks-rs](https://github.com/hoodie/concatenation_benchmarks-rs). It has been updated to use [Criterion](https://github.com/bheisler/criterion.rs) for more updated benchmarking for 2025 and extended with additional benchmarks for number-to-string formatting.

## Intro

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here we benchmark several different ways to concatenate the strings `"2014-11-28"`, `"T"` and `"12:00:09Z"` into `"2014-11-28T12:00:09Z"`.

Additionally, We've added benchmarks for a more realistic use case: formatting RGB values into a CSS background-color string like `"background-color: rgb(255, 128, 64)"`. This tests how different approaches handle number-to-string conversion.

## How to run?

* benchmarks: `cargo bench`
* generate results table: `bun run scripts/results-to-markdown.ts`

## Hardware

- CPU: AMD Ryzen 7 5800X3D @ 4.5GHz
- RAM: 32GB DDR4 3200MHz
- Kernel: 6.17.9-arch1-1

## Compiler

```
rustc 1.94.0-nightly (1aa9bab4e 2025-12-05)
binary: rustc
commit-hash: 1aa9bab4ecbce4859eaad53000f78158ebe2be2c
commit-date: 2025-12-05
host: x86_64-unknown-linux-gnu
release: 1.94.0-nightly
LLVM version: 21.1.5
```


## Results

Benchmarks were run using [Criterion](https://crates.io/crates/criterion) with [mimalloc](https://crates.io/crates/mimalloc) as the global allocator.

## String Concatenation (DateTime)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `from_bytes` (UNSAFE) | 0.44 ns | ±0.00 ns | 1.00x (baseline) |
| 2 | `string_concat_macro` | 4.77 ns | ±0.01 ns | 10.78x slower |
| 3 | `concat_strs_macro` | 5.00 ns | ±0.04 ns | 11.30x slower |
| 4 | `mut_string_with_capacity_push_str_char` | 5.17 ns | ±0.03 ns | 11.70x slower |
| 5 | `mut_string_with_capacity_push_str` | 5.19 ns | ±0.04 ns | 11.72x slower |
| 6 | `concat_string_macro` | 5.39 ns | ±0.04 ns | 12.19x slower |
| 7 | `mut_string_with_too_much_capacity_push_str` | 9.85 ns | ±0.19 ns | 22.27x slower |
| 8 | `concat_in_place_macro` | 10.16 ns | ±0.03 ns | 22.97x slower |
| 9 | `to_string_plus_op` | 22.01 ns | ±0.17 ns | 49.75x slower |
| 10 | `to_owned_plus_op` | 22.12 ns | ±0.09 ns | 50.00x slower |
| 11 | `string_from_plus_op` | 22.18 ns | ±0.11 ns | 50.14x slower |
| 12 | `mut_string_push_str` | 24.32 ns | ±0.06 ns | 54.97x slower |
| 13 | `array_join` | 26.16 ns | ±0.12 ns | 59.13x slower |
| 14 | `collect_from_array_to_string` | 26.22 ns | ±0.12 ns | 59.27x slower |
| 15 | `string_from_all` | 26.77 ns | ±0.34 ns | 60.52x slower |
| 16 | `array_join_long` | 26.82 ns | ±0.15 ns | 60.62x slower |
| 17 | `array_concat` | 26.82 ns | ±0.20 ns | 60.63x slower |
| 18 | `mut_string_with_too_little_capacity_push_str` | 28.63 ns | ±0.34 ns | 64.73x slower |
| 19 | `collect_from_vec_to_string` | 29.41 ns | ±1.37 ns | 66.49x slower |
| 20 | `joinery` | 35.91 ns | ±1.18 ns | 81.17x slower |
| 21 | `format_macro` | 45.50 ns | ±0.06 ns | 102.85x slower |
| 22 | `format_macro_implicit_args` | 45.66 ns | ±0.16 ns | 103.22x slower |
| 23 | `mut_string_push_string` | 57.24 ns | ±0.29 ns | 129.40x slower |

## RGB Formatting (with number conversion)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `rgb_itoa_reuse_buffer` | 4.86 ns | ±0.02 ns | 1.00x (baseline) |
| 2 | `rgb_itoa_with_capacity` | 4.87 ns | ±0.05 ns | 1.00x slower |
| 3 | `rgb_precomputed_concat_string` | 15.54 ns | ±0.15 ns | 3.19x slower |
| 4 | `rgb_string_concat_macro` | 37.68 ns | ±0.34 ns | 7.75x slower |
| 5 | `rgb_concat_in_place_macro` | 37.79 ns | ±0.20 ns | 7.77x slower |
| 6 | `rgb_mut_string_with_capacity_push_str` | 38.35 ns | ±0.23 ns | 7.88x slower |
| 7 | `rgb_concat_strs_macro` | 40.33 ns | ±0.40 ns | 8.29x slower |
| 8 | `rgb_concat_string_macro` | 41.49 ns | ±0.19 ns | 8.53x slower |
| 9 | `rgb_string_from_plus_op` | 47.75 ns | ±0.52 ns | 9.82x slower |
| 10 | `rgb_mut_string_push_str` | 52.40 ns | ±0.27 ns | 10.77x slower |
| 11 | `rgb_write_macro` | 53.06 ns | ±0.20 ns | 10.91x slower |
| 12 | `rgb_precomputed_strings` | 54.92 ns | ±1.32 ns | 11.29x slower |
| 13 | `rgb_array_join` | 60.65 ns | ±2.01 ns | 12.47x slower |
| 14 | `rgb_format_macro_implicit_args` | 61.66 ns | ±0.26 ns | 12.68x slower |
| 15 | `rgb_format_macro` | 62.08 ns | ±0.71 ns | 12.76x slower |

## Summary

**Fastest string concat:** `from_bytes` (UNSAFE) (0.44 ns)

**Fastest RGB format:** `rgb_itoa_reuse_buffer` (4.86 ns)
