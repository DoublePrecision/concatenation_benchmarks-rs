# Rust String Concatenation Benchmarks

> **Note:** This project was originally forked from [Hendrik Sollich's concatenation_benchmarks-rs](https://github.com/hoodie/concatenation_benchmarks-rs). It has been updated to use [Criterion](https://github.com/bheisler/criterion.rs) for more updated benchmarking for 2025 and extended with additional benchmarks for number-to-string formatting.

## Intro

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here we benchmark several different ways to concatenate the strings `"2014-11-28"`, `"T"` and `"12:00:09Z"` into `"2014-11-28T12:00:09Z"`.

Additionally, We've added benchmarks for a more realistic use case: formatting RGB values into a CSS background-color string like `"background-color: rgb(255, 128, 64)"`. This tests how different approaches handle number-to-string conversion.

## How to run?

* benchmarks: `cargo bench`
* generate results table: `bun run scripts/results-to-markdown.ts`

## Criterion

Benchmarks were run using [Criterion](https://crates.io/crates/criterion) with [mimalloc](https://crates.io/crates/mimalloc) as the global allocator.

# Results

## Linux

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

## String Concatenation (DateTime)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `from_bytes` | 0.34 ns | ±0.11 ns | 1.00x (baseline) |
| 2 | `concat_string_macro` | 4.71 ns | ±0.01 ns | 14.03x slower |
| 3 | `mut_string_with_capacity_push_str` | 4.77 ns | ±0.04 ns | 14.21x slower |
| 4 | `string_concat_macro` | 4.79 ns | ±0.02 ns | 14.28x slower |
| 5 | `concat_strs_macro` | 4.94 ns | ±0.01 ns | 14.72x slower |
| 6 | `mut_string_with_capacity_push_str_char` | 5.74 ns | ±0.34 ns | 17.09x slower |
| 7 | `concat_in_place_macro` | 9.33 ns | ±0.02 ns | 27.80x slower |
| 8 | `mut_string_with_too_much_capacity_push_str` | 10.69 ns | ±0.46 ns | 31.86x slower |
| 9 | `array_join` | 19.49 ns | ±0.26 ns | 58.06x slower |
| 10 | `array_concat` | 19.78 ns | ±0.47 ns | 58.94x slower |
| 11 | `array_join_long` | 19.94 ns | ±0.44 ns | 59.41x slower |
| 12 | `to_string_plus_op` | 23.58 ns | ±0.16 ns | 70.24x slower |
| 13 | `to_owned_plus_op` | 23.68 ns | ±0.35 ns | 70.56x slower |
| 14 | `string_from_plus_op` | 24.43 ns | ±0.16 ns | 72.80x slower |
| 15 | `mut_string_push_str` | 25.77 ns | ±0.24 ns | 76.77x slower |
| 16 | `string_from_all` | 27.08 ns | ±0.21 ns | 80.69x slower |
| 17 | `collect_from_array_to_string` | 27.27 ns | ±0.10 ns | 81.24x slower |
| 18 | `collect_from_vec_to_string` | 28.40 ns | ±0.44 ns | 84.61x slower |
| 19 | `mut_string_with_too_little_capacity_push_str` | 31.58 ns | ±0.28 ns | 94.10x slower |
| 20 | `joinery` | 37.10 ns | ±1.00 ns | 110.53x slower |
| 21 | `format_macro` | 45.05 ns | ±0.95 ns | 134.22x slower |
| 22 | `format_macro_implicit_args` | 45.69 ns | ±1.57 ns | 136.12x slower |
| 23 | `mut_string_push_string` | 48.03 ns | ±0.25 ns | 143.10x slower |

## RGB Formatting (with number conversion)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `rgb_itoa_reuse_buffer` | 5.34 ns | ±0.09 ns | 1.00x (baseline) |
| 2 | `rgb_itoa_with_capacity` | 5.35 ns | ±0.09 ns | 1.00x slower |
| 3 | `rgb_precomputed_concat_string` | 15.67 ns | ±0.24 ns | 2.94x slower |
| 4 | `rgb_string_concat_macro` | 38.29 ns | ±0.21 ns | 7.17x slower |
| 5 | `rgb_mut_string_with_capacity_push_str` | 38.69 ns | ±0.56 ns | 7.25x slower |
| 6 | `rgb_concat_strs_macro` | 38.89 ns | ±0.48 ns | 7.29x slower |
| 7 | `rgb_concat_in_place_macro` | 40.67 ns | ±0.40 ns | 7.62x slower |
| 8 | `rgb_concat_string_macro` | 40.85 ns | ±0.34 ns | 7.66x slower |
| 9 | `rgb_string_from_plus_op` | 48.18 ns | ±1.23 ns | 9.03x slower |
| 10 | `rgb_mut_string_push_str` | 51.09 ns | ±1.07 ns | 9.57x slower |
| 11 | `rgb_write_macro` | 54.84 ns | ±0.37 ns | 10.28x slower |
| 12 | `rgb_precomputed_strings` | 56.19 ns | ±2.98 ns | 10.53x slower |
| 13 | `rgb_array_join` | 62.45 ns | ±0.59 ns | 11.70x slower |
| 14 | `rgb_format_macro_implicit_args` | 63.64 ns | ±0.30 ns | 11.92x slower |
| 15 | `rgb_format_macro` | 64.99 ns | ±0.69 ns | 12.18x slower |

## Summary

**Fastest string concat:** `from_bytes` (0.34 ns)

**Fastest RGB format:** `rgb_itoa_reuse_buffer` (5.34 ns)
