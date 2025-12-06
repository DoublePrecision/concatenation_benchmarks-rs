use criterion::{Criterion, black_box, criterion_group, criterion_main};

use concat_string::concat_string;
use concat_strs::concat_strs;
use joinery::prelude::*;
use mimalloc::MiMalloc;

#[macro_use]
extern crate string_concat;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static DATE: &str = "2014-11-28";
static T: &str = "T";
static TIME: &str = "12:00:09Z";

// RGB values for background-color benchmarks (u8 bytes, max 3 digits each)
static RED: u8 = 255;
static GREEN: u8 = 128;
static BLUE: u8 = 64;
// "background-color: rgb(255, 128, 64)" = 36 chars

fn array_concat(c: &mut Criterion) {
    c.bench_function("array_concat", |b| {
        b.iter(|| {
            let datetime: &str = &[DATE, T, TIME].concat();
            black_box(datetime);
        });
    });
}

fn array_join(c: &mut Criterion) {
    c.bench_function("array_join", |b| {
        b.iter(|| {
            let datetime: &str = &[DATE, TIME].join(T);
            black_box(datetime);
        });
    });
}

fn array_join_long(c: &mut Criterion) {
    c.bench_function("array_join_long", |b| {
        b.iter(|| {
            let datetime: &str = &[DATE, T, TIME].join("");
            black_box(datetime);
        });
    });
}

fn collect_from_array_to_string(c: &mut Criterion) {
    let list = [DATE, T, TIME];
    c.bench_function("collect_from_array_to_string", |b| {
        b.iter(|| {
            let datetime: String = list.iter().map(|x| *x).collect();
            black_box(datetime);
        });
    });
}

fn collect_from_vec_to_string(c: &mut Criterion) {
    let list = vec![DATE, T, TIME];
    c.bench_function("collect_from_vec_to_string", |b| {
        b.iter(|| {
            let datetime: String = list.iter().map(|x| *x).collect();
            black_box(datetime);
        });
    });
}

fn format_macro(c: &mut Criterion) {
    c.bench_function("format_macro", |b| {
        b.iter(|| {
            let datetime: &str = &format!("{}T{}", DATE, TIME);
            black_box(datetime);
        });
    });
}

fn format_macro_implicit_args(c: &mut Criterion) {
    c.bench_function("format_macro_implicit_args", |b| {
        b.iter(|| {
            let datetime: &str = &format!("{DATE}T{TIME}");
            black_box(datetime);
        });
    });
}

#[cfg(unix)]
fn from_bytes(c: &mut Criterion) {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::slice;

    c.bench_function("from_bytes", |b| {
        b.iter(|| {
            let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };
            let datetime = OsStr::from_bytes(bytes);
            black_box(datetime);
        });
    });
}

fn mut_string_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::new();
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        });
    });
}

fn mut_string_push_string(c: &mut Criterion) {
    c.bench_function("mut_string_push_string", |b| {
        b.iter(|| {
            let mut datetime = Vec::<String>::new();
            datetime.push(String::from(DATE));
            datetime.push(String::from(T));
            datetime.push(String::from(TIME));
            let datetime = datetime.join("");
            black_box(datetime);
        });
    });
}

fn mut_string_with_capacity_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_with_capacity_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::with_capacity(20);
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        });
    });
}

fn mut_string_with_capacity_push_str_char(c: &mut Criterion) {
    c.bench_function("mut_string_with_capacity_push_str_char", |b| {
        b.iter(|| {
            let mut datetime = String::with_capacity(20);
            datetime.push_str(DATE);
            datetime.push('T');
            datetime.push_str(TIME);
            black_box(datetime);
        });
    });
}

fn mut_string_with_too_little_capacity_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_with_too_little_capacity_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::with_capacity(2);
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        });
    });
}

fn mut_string_with_too_much_capacity_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_with_too_much_capacity_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::with_capacity(200);
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        });
    });
}

fn string_from_all(c: &mut Criterion) {
    c.bench_function("string_from_all", |b| {
        b.iter(|| {
            let datetime: &str = &(String::from(DATE) + &String::from(T) + &String::from(TIME));
            black_box(datetime);
        });
    });
}

fn string_from_plus_op(c: &mut Criterion) {
    c.bench_function("string_from_plus_op", |b| {
        b.iter(|| {
            let datetime: &str = &(String::from(DATE) + T + TIME);
            black_box(datetime);
        });
    });
}

fn to_owned_plus_op(c: &mut Criterion) {
    c.bench_function("to_owned_plus_op", |b| {
        b.iter(|| {
            let datetime: &str = &(DATE.to_owned() + T + TIME);
            black_box(datetime);
        });
    });
}

fn to_string_plus_op(c: &mut Criterion) {
    c.bench_function("to_string_plus_op", |b| {
        b.iter(|| {
            let datetime: &str = &(DATE.to_string() + T + TIME);
            black_box(datetime);
        });
    });
}

fn concat_in_place_macro(c: &mut Criterion) {
    c.bench_function("concat_in_place_macro", |b| {
        b.iter(|| {
            let mut url = String::new();
            let datetime = concat_in_place::strcat!(&mut url, DATE T TIME);
            black_box(datetime);
        });
    });
}

fn string_concat_macro(c: &mut Criterion) {
    c.bench_function("string_concat_macro", |b| {
        b.iter(|| {
            let datetime = &string_concat!(DATE, T, TIME);
            black_box(datetime);
        });
    });
}

fn concat_strs_macro(c: &mut Criterion) {
    c.bench_function("concat_strs_macro", |b| {
        b.iter(|| {
            let datetime = &concat_strs!(DATE, T, TIME);
            black_box(datetime);
        });
    });
}

fn concat_string_macro(c: &mut Criterion) {
    c.bench_function("concat_string_macro", |b| {
        b.iter(|| {
            let datetime = concat_string!(DATE, T, TIME);
            black_box(datetime);
        });
    });
}

fn joinery_bench(c: &mut Criterion) {
    let vec = vec![DATE, T, TIME];
    c.bench_function("joinery", |b| {
        b.iter(|| {
            let datetime = &vec.iter().join_concat().to_string();
            black_box(datetime);
        });
    });
}

// ===== RGB FORMATTING BENCHMARKS =====

fn rgb_format_macro(c: &mut Criterion) {
    c.bench_function("rgb_format_macro", |b| {
        b.iter(|| {
            let style = format!("background-color: rgb({}, {}, {})", RED, GREEN, BLUE);
            black_box(style);
        });
    });
}

fn rgb_format_macro_implicit_args(c: &mut Criterion) {
    c.bench_function("rgb_format_macro_implicit_args", |b| {
        b.iter(|| {
            let style = format!("background-color: rgb({RED}, {GREEN}, {BLUE})");
            black_box(style);
        });
    });
}

fn rgb_mut_string_push_str(c: &mut Criterion) {
    c.bench_function("rgb_mut_string_push_str", |b| {
        b.iter(|| {
            let mut style = String::new();
            style.push_str("background-color: rgb(");
            style.push_str(&RED.to_string());
            style.push_str(", ");
            style.push_str(&GREEN.to_string());
            style.push_str(", ");
            style.push_str(&BLUE.to_string());
            style.push(')');
            black_box(style);
        });
    });
}

fn rgb_mut_string_with_capacity_push_str(c: &mut Criterion) {
    // "background-color: rgb(255, 128, 64)" = 36 chars max
    c.bench_function("rgb_mut_string_with_capacity_push_str", |b| {
        b.iter(|| {
            let mut style = String::with_capacity(36);
            style.push_str("background-color: rgb(");
            style.push_str(&RED.to_string());
            style.push_str(", ");
            style.push_str(&GREEN.to_string());
            style.push_str(", ");
            style.push_str(&BLUE.to_string());
            style.push(')');
            black_box(style);
        });
    });
}

fn rgb_string_from_plus_op(c: &mut Criterion) {
    c.bench_function("rgb_string_from_plus_op", |b| {
        b.iter(|| {
            let style = String::from("background-color: rgb(")
                + &RED.to_string()
                + ", "
                + &GREEN.to_string()
                + ", "
                + &BLUE.to_string()
                + ")";
            black_box(style);
        });
    });
}

fn rgb_array_join(c: &mut Criterion) {
    c.bench_function("rgb_array_join", |b| {
        b.iter(|| {
            let style = String::from("background-color: rgb(")
                + &[RED.to_string(), GREEN.to_string(), BLUE.to_string()].join(", ")
                + ")";
            black_box(style);
        });
    });
}

fn rgb_concat_string_macro(c: &mut Criterion) {
    c.bench_function("rgb_concat_string_macro", |b| {
        b.iter(|| {
            let r = RED.to_string();
            let g = GREEN.to_string();
            let bl = BLUE.to_string();
            let style = concat_string!("background-color: rgb(", r, ", ", g, ", ", bl, ")");
            black_box(style);
        });
    });
}

fn rgb_concat_strs_macro(c: &mut Criterion) {
    c.bench_function("rgb_concat_strs_macro", |b| {
        b.iter(|| {
            let r = RED.to_string();
            let g = GREEN.to_string();
            let bl = BLUE.to_string();
            let style = concat_strs!(
                "background-color: rgb(",
                r.as_str(),
                ", ",
                g.as_str(),
                ", ",
                bl.as_str(),
                ")"
            );
            black_box(style);
        });
    });
}

fn rgb_concat_in_place_macro(c: &mut Criterion) {
    c.bench_function("rgb_concat_in_place_macro", |b| {
        b.iter(|| {
            let r = RED.to_string();
            let g = GREEN.to_string();
            let bl = BLUE.to_string();
            let style = concat_in_place::strcat!("background-color: rgb(" r.as_str() ", " g.as_str() ", " bl.as_str() ")");
            black_box(style);
        });
    });
}

fn rgb_string_concat_macro(c: &mut Criterion) {
    c.bench_function("rgb_string_concat_macro", |b| {
        b.iter(|| {
            let r = RED.to_string();
            let g = GREEN.to_string();
            let bl = BLUE.to_string();
            let style = string_concat!("background-color: rgb(", r, ", ", g, ", ", bl, ")");
            black_box(style);
        });
    });
}

// Using itoa crate for faster integer-to-string conversion
fn rgb_itoa_with_capacity(c: &mut Criterion) {
    c.bench_function("rgb_itoa_with_capacity", |b| {
        b.iter(|| {
            let mut style = String::with_capacity(36);
            style.push_str("background-color: rgb(");
            style.push_str(itoa::Buffer::new().format(RED));
            style.push_str(", ");
            style.push_str(itoa::Buffer::new().format(GREEN));
            style.push_str(", ");
            style.push_str(itoa::Buffer::new().format(BLUE));
            style.push(')');
            black_box(style);
        });
    });
}

fn rgb_itoa_reuse_buffer(c: &mut Criterion) {
    c.bench_function("rgb_itoa_reuse_buffer", |b| {
        b.iter(|| {
            let mut buf = itoa::Buffer::new();
            let mut style = String::with_capacity(36);
            style.push_str("background-color: rgb(");
            style.push_str(buf.format(RED));
            style.push_str(", ");
            style.push_str(buf.format(GREEN));
            style.push_str(", ");
            style.push_str(buf.format(BLUE));
            style.push(')');
            black_box(style);
        });
    });
}

// Pre-compute string conversions outside the hot loop
fn rgb_precomputed_strings(c: &mut Criterion) {
    let r = RED.to_string();
    let g = GREEN.to_string();
    let bl = BLUE.to_string();
    c.bench_function("rgb_precomputed_strings", |b| {
        b.iter(|| {
            let style = format!("background-color: rgb({}, {}, {})", r, g, bl);
            black_box(style);
        });
    });
}

fn rgb_precomputed_concat_string(c: &mut Criterion) {
    let r = RED.to_string();
    let g = GREEN.to_string();
    let bl = BLUE.to_string();
    c.bench_function("rgb_precomputed_concat_string", |b| {
        b.iter(|| {
            let style = concat_string!("background-color: rgb(", &r, ", ", &g, ", ", &bl, ")");
            black_box(style);
        });
    });
}

fn rgb_write_macro(c: &mut Criterion) {
    use std::fmt::Write;
    c.bench_function("rgb_write_macro", |b| {
        b.iter(|| {
            let mut style = String::with_capacity(36);
            write!(style, "background-color: rgb({}, {}, {})", RED, GREEN, BLUE).unwrap();
            black_box(style);
        });
    });
}

#[cfg(unix)]
criterion_group!(
    benches,
    array_concat,
    array_join,
    array_join_long,
    collect_from_array_to_string,
    collect_from_vec_to_string,
    format_macro,
    format_macro_implicit_args,
    from_bytes,
    mut_string_push_str,
    mut_string_push_string,
    mut_string_with_capacity_push_str,
    mut_string_with_capacity_push_str_char,
    mut_string_with_too_little_capacity_push_str,
    mut_string_with_too_much_capacity_push_str,
    string_from_all,
    string_from_plus_op,
    to_owned_plus_op,
    to_string_plus_op,
    concat_in_place_macro,
    string_concat_macro,
    concat_strs_macro,
    concat_string_macro,
    joinery_bench,
    // RGB benchmarks
    rgb_format_macro,
    rgb_format_macro_implicit_args,
    rgb_mut_string_push_str,
    rgb_mut_string_with_capacity_push_str,
    rgb_string_from_plus_op,
    rgb_array_join,
    rgb_concat_string_macro,
    rgb_concat_strs_macro,
    rgb_concat_in_place_macro,
    rgb_string_concat_macro,
    rgb_itoa_with_capacity,
    rgb_itoa_reuse_buffer,
    rgb_precomputed_strings,
    rgb_precomputed_concat_string,
    rgb_write_macro,
);

#[cfg(not(unix))]
criterion_group!(
    benches,
    array_concat,
    array_join,
    array_join_long,
    collect_from_array_to_string,
    collect_from_vec_to_string,
    format_macro,
    format_macro_implicit_args,
    mut_string_push_str,
    mut_string_push_string,
    mut_string_with_capacity_push_str,
    mut_string_with_capacity_push_str_char,
    mut_string_with_too_little_capacity_push_str,
    mut_string_with_too_much_capacity_push_str,
    string_from_all,
    string_from_plus_op,
    to_owned_plus_op,
    to_string_plus_op,
    concat_in_place_macro,
    string_concat_macro,
    concat_strs_macro,
    concat_string_macro,
    joinery_bench,
    // RGB benchmarks
    rgb_format_macro,
    rgb_format_macro_implicit_args,
    rgb_mut_string_push_str,
    rgb_mut_string_with_capacity_push_str,
    rgb_string_from_plus_op,
    rgb_array_join,
    rgb_concat_string_macro,
    rgb_concat_strs_macro,
    rgb_concat_in_place_macro,
    rgb_string_concat_macro,
    rgb_itoa_with_capacity,
    rgb_itoa_reuse_buffer,
    rgb_precomputed_strings,
    rgb_precomputed_concat_string,
    rgb_write_macro,
);

criterion_main!(benches);
