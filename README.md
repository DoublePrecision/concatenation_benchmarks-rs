# Rust String Concatenation Benchmarks

> **Note:** This project was originally forked from [Hendrik Sollich's concatenation_benchmarks-rs](https://github.com/hoodie/concatenation_benchmarks-rs). It has been updated to use [Criterion](https://github.com/bheisler/criterion.rs) for more updated benchmarking for 2025 and extended with additional benchmarks for number-to-string formatting.

## Intro

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here we benchmark several different ways to concatenate the strings `"2014-11-28"`, `"T"` and `"12:00:09Z"` into `"2014-11-28T12:00:09Z"`.

Additionally, We've added benchmarks such as formatting RGB values into a CSS background-color string like `"background-color: rgb(255, 128, 64)"` as well as using `compact_str`.

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
| 2 | `mut_string_with_capacity_push_str` | 4.73 ns | ±0.02 ns | 13.91x slower |
| 3 | `concat_strs_macro` | 4.81 ns | ±0.02 ns | 14.14x slower |
| 4 | `mut_string_with_capacity_push_str_char` | 4.93 ns | ±0.22 ns | 14.49x slower |
| 5 | `string_concat_macro` | 4.97 ns | ±0.37 ns | 14.62x slower |
| 6 | `concat_string_macro` | 5.02 ns | ±0.02 ns | 14.74x slower |
| 7 | `concat_in_place_macro` | 9.49 ns | ±0.03 ns | 27.89x slower |
| 8 | `mut_string_with_too_much_capacity_push_str` | 9.95 ns | ±0.14 ns | 29.25x slower |
| 9 | `array_join` | 19.38 ns | ±0.12 ns | 56.98x slower |
| 10 | `array_join_long` | 19.70 ns | ±0.09 ns | 57.91x slower |
| 11 | `array_concat` | 20.02 ns | ±0.43 ns | 58.86x slower |
| 12 | `to_string_plus_op` | 20.28 ns | ±0.13 ns | 59.61x slower |
| 13 | `to_owned_plus_op` | 20.38 ns | ±0.14 ns | 59.91x slower |
| 14 | `string_from_plus_op` | 21.15 ns | ±0.14 ns | 62.16x slower |
| 15 | `mut_string_push_str` | 23.65 ns | ±0.17 ns | 69.52x slower |
| 16 | `string_from_all` | 24.09 ns | ±0.18 ns | 70.81x slower |
| 17 | `collect_from_array_to_string` | 27.34 ns | ±0.29 ns | 80.37x slower |
| 18 | `mut_string_with_too_little_capacity_push_str` | 27.94 ns | ±0.28 ns | 82.13x slower |
| 19 | `collect_from_vec_to_string` | 29.06 ns | ±0.15 ns | 85.44x slower |
| 20 | `joinery` | 35.57 ns | ±0.12 ns | 104.56x slower |
| 21 | `mut_string_push_string` | 45.54 ns | ±0.63 ns | 133.86x slower |
| 22 | `format_macro_implicit_args` | 46.56 ns | ±0.66 ns | 136.87x slower |
| 23 | `format_macro` | 46.93 ns | ±0.20 ns | 137.96x slower |

## RGB Formatting (with number conversion)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `rgb_itoa_with_capacity` | 5.28 ns | ±0.05 ns | 1.00x (baseline) |
| 2 | `rgb_itoa_reuse_buffer` | 6.51 ns | ±0.52 ns | 1.23x slower |
| 3 | `rgb_precomputed_concat_string` | 16.25 ns | ±0.28 ns | 3.08x slower |
| 4 | `rgb_string_concat_macro` | 38.12 ns | ±0.42 ns | 7.22x slower |
| 5 | `rgb_mut_string_with_capacity_push_str` | 38.32 ns | ±0.74 ns | 7.26x slower |
| 6 | `rgb_concat_strs_macro` | 39.19 ns | ±1.05 ns | 7.42x slower |
| 7 | `rgb_concat_string_macro` | 39.87 ns | ±0.41 ns | 7.55x slower |
| 8 | `rgb_concat_in_place_macro` | 40.97 ns | ±0.13 ns | 7.76x slower |
| 9 | `rgb_string_from_plus_op` | 48.78 ns | ±0.90 ns | 9.24x slower |
| 10 | `rgb_write_macro` | 52.67 ns | ±2.50 ns | 9.98x slower |
| 11 | `rgb_mut_string_push_str` | 52.69 ns | ±0.39 ns | 9.98x slower |
| 12 | `rgb_precomputed_strings` | 56.27 ns | ±4.05 ns | 10.66x slower |
| 13 | `rgb_array_join` | 62.82 ns | ±0.27 ns | 11.90x slower |
| 14 | `rgb_format_macro_implicit_args` | 64.13 ns | ±0.56 ns | 12.15x slower |
| 15 | `rgb_format_macro` | 64.18 ns | ±0.60 ns | 12.16x slower |

## CompactString Concatenation (DateTime)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `collect_from_vec_to_compact_string` | 13.24 ns | ±0.70 ns | 1.00x (baseline) |
| 2 | `collect_from_array_to_compact_string` | 13.37 ns | ±0.72 ns | 1.01x slower |
| 3 | `compact_string_from_plus_op` | 22.94 ns | ±0.81 ns | 1.73x slower |
| 4 | `compact_string_new_push_str` | 26.69 ns | ±0.38 ns | 2.02x slower |
| 5 | `compact_string_with_capacity_push_str` | 27.30 ns | ±0.23 ns | 2.06x slower |
| 6 | `compact_string_with_capacity_push_str_char` | 27.81 ns | ±0.25 ns | 2.10x slower |
| 7 | `compact_string_to_compact_string_plus_op` | 43.41 ns | ±0.63 ns | 3.28x slower |

## CompactString RGB Formatting (with number conversion)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `rgb_compact_string_itoa` | 50.19 ns | ±0.41 ns | 1.00x (baseline) |
| 2 | `rgb_compact_string_with_capacity_push_str` | 81.77 ns | ±0.83 ns | 1.63x slower |
| 3 | `rgb_compact_string_write_macro` | 85.15 ns | ±1.81 ns | 1.70x slower |
| 4 | `rgb_compact_string_push_str` | 87.16 ns | ±0.52 ns | 1.74x slower |

## Summary

- **Fastest String concat:** `from_bytes` (0.34 ns)
- **Fastest RGB format:** `rgb_itoa_with_capacity` (5.28 ns)
- **Fastest CompactString concat:** `collect_from_vec_to_compact_string` (13.24 ns)
- **Fastest CompactString RGB format:** `rgb_compact_string_itoa` (50.19 ns)

## Macbook Air M1 (base model)

## Hardware

- CPU: Apple Silicon M1
- RAM: 8GB
- Kernel: 25.0.0

## Compiler

```
rustc 1.94.0-nightly (1aa9bab4e 2025-12-05)
binary: rustc
commit-hash: 1aa9bab4ecbce4859eaad53000f78158ebe2be2c
commit-date: 2025-12-05
host: aarch64-apple-darwin
release: 1.94.0-nightly
LLVM version: 21.1.5
```

## String Concatenation (DateTime)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `from_bytes` | 0.32 ns | ±0.00 ns | 1.00x (baseline) |
| 2 | `mut_string_with_capacity_push_str` | 5.75 ns | ±0.03 ns | 17.95x slower |
| 3 | `mut_string_with_capacity_push_str_char` | 5.75 ns | ±0.04 ns | 17.96x slower |
| 4 | `string_concat_macro` | 5.75 ns | ±0.04 ns | 17.96x slower |
| 5 | `concat_strs_macro` | 5.76 ns | ±0.03 ns | 17.98x slower |
| 6 | `concat_string_macro` | 5.77 ns | ±0.04 ns | 18.00x slower |
| 7 | `concat_in_place_macro` | 9.37 ns | ±0.05 ns | 29.24x slower |
| 8 | `mut_string_with_too_much_capacity_push_str` | 10.45 ns | ±0.05 ns | 32.61x slower |
| 9 | `array_join` | 15.32 ns | ±1.60 ns | 47.81x slower |
| 10 | `to_string_plus_op` | 17.24 ns | ±0.08 ns | 53.81x slower |
| 11 | `to_owned_plus_op` | 17.26 ns | ±0.09 ns | 53.88x slower |
| 12 | `string_from_plus_op` | 17.30 ns | ±0.08 ns | 54.00x slower |
| 13 | `array_concat` | 19.07 ns | ±0.37 ns | 59.52x slower |
| 14 | `array_join_long` | 19.20 ns | ±0.29 ns | 59.92x slower |
| 15 | `string_from_all` | 23.98 ns | ±0.16 ns | 74.83x slower |
| 16 | `mut_string_push_str` | 24.78 ns | ±0.48 ns | 77.34x slower |
| 17 | `collect_from_array_to_string` | 26.09 ns | ±0.09 ns | 81.43x slower |
| 18 | `mut_string_with_too_little_capacity_push_str` | 27.64 ns | ±0.18 ns | 86.26x slower |
| 19 | `collect_from_vec_to_string` | 28.45 ns | ±0.16 ns | 88.79x slower |
| 20 | `joinery` | 32.86 ns | ±0.15 ns | 102.55x slower |
| 21 | `format_macro` | 39.83 ns | ±0.25 ns | 124.31x slower |
| 22 | `format_macro_implicit_args` | 39.83 ns | ±0.29 ns | 124.31x slower |
| 23 | `mut_string_push_string` | 42.76 ns | ±0.24 ns | 133.47x slower |

## RGB Formatting (with number conversion)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `rgb_itoa_reuse_buffer` | 6.43 ns | ±0.03 ns | 1.00x (baseline) |
| 2 | `rgb_itoa_with_capacity` | 6.44 ns | ±0.03 ns | 1.00x slower |
| 3 | `rgb_precomputed_concat_string` | 13.13 ns | ±0.06 ns | 2.04x slower |
| 4 | `rgb_mut_string_with_capacity_push_str` | 35.36 ns | ±0.14 ns | 5.50x slower |
| 5 | `rgb_string_concat_macro` | 36.79 ns | ±0.13 ns | 5.72x slower |
| 6 | `rgb_concat_strs_macro` | 36.86 ns | ±0.13 ns | 5.73x slower |
| 7 | `rgb_concat_string_macro` | 36.91 ns | ±0.22 ns | 5.74x slower |
| 8 | `rgb_concat_in_place_macro` | 36.92 ns | ±0.23 ns | 5.74x slower |
| 9 | `rgb_precomputed_strings` | 45.07 ns | ±0.18 ns | 7.01x slower |
| 10 | `rgb_write_macro` | 45.65 ns | ±0.20 ns | 7.10x slower |
| 11 | `rgb_string_from_plus_op` | 47.24 ns | ±0.24 ns | 7.35x slower |
| 12 | `rgb_mut_string_push_str` | 50.47 ns | ±0.22 ns | 7.85x slower |
| 13 | `rgb_format_macro` | 52.53 ns | ±0.22 ns | 8.17x slower |
| 14 | `rgb_format_macro_implicit_args` | 52.59 ns | ±0.35 ns | 8.18x slower |
| 15 | `rgb_array_join` | 59.96 ns | ±0.32 ns | 9.32x slower |

## CompactString Concatenation (DateTime)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `collect_from_array_to_compact_string` | 10.51 ns | ±0.03 ns | 1.00x (baseline) |
| 2 | `collect_from_vec_to_compact_string` | 10.51 ns | ±0.03 ns | 1.00x slower |
| 3 | `compact_string_from_plus_op` | 15.52 ns | ±0.10 ns | 1.48x slower |
| 4 | `compact_string_with_capacity_push_str` | 16.25 ns | ±0.06 ns | 1.55x slower |
| 5 | `compact_string_new_push_str` | 16.25 ns | ±0.07 ns | 1.55x slower |
| 6 | `compact_string_with_capacity_push_str_char` | 17.20 ns | ±0.07 ns | 1.64x slower |
| 7 | `compact_string_to_compact_string_plus_op` | 26.77 ns | ±1.71 ns | 2.55x slower |

## CompactString RGB Formatting (with number conversion)

| Rank | Benchmark | Mean | Std Dev | vs Fastest |
|------|-----------|------|---------|------------|
| 1 | `rgb_compact_string_itoa` | 49.14 ns | ±0.16 ns | 1.00x (baseline) |
| 2 | `rgb_compact_string_with_capacity_push_str` | 72.55 ns | ±0.22 ns | 1.48x slower |
| 3 | `rgb_compact_string_write_macro` | 77.12 ns | ±0.21 ns | 1.57x slower |
| 4 | `rgb_compact_string_push_str` | 87.14 ns | ±0.32 ns | 1.77x slower |

## Summary

- **Fastest String concat:** `from_bytes` (0.32 ns)
- **Fastest RGB format:** `rgb_itoa_reuse_buffer` (6.43 ns)
- **Fastest CompactString concat:** `collect_from_array_to_compact_string` (10.51 ns)
- **Fastest CompactString RGB format:** `rgb_compact_string_itoa` (49.14 ns)
