# Concatenation Benchmark Results

Generated: 2025-12-06T19:17:54.552Z

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

**Fastest String concat:** `from_bytes` (0.32 ns)

**Fastest RGB format:** `rgb_itoa_reuse_buffer` (6.43 ns)

**Fastest CompactString concat:** `collect_from_array_to_compact_string` (10.51 ns)

**Fastest CompactString RGB format:** `rgb_compact_string_itoa` (49.14 ns)

