#![allow(warnings)]

use bachelor::static_files::simd_traits::SimdPrimitiveImpl;
use criterion::{criterion_group, criterion_main, Criterion};
use std::io::{self, Read};
use std::fs::File;
use std::arch::x86_64::*;
use std::result;

use bachelor::generated::declarations::*;
use bachelor::generated::extensions::simd::intel::avx2::avx2;
use bachelor::generated::extensions::simd::intel::sse::sse;
use bachelor::static_files::*;

type base = u8;
type target = avx2<base>;

// Function to load data from file into memory
fn load_data_into_memory(filename: &str) -> io::Result<Vec<base>> {
    let mut file = File::open(filename)?;
    let mut data: Vec<base> = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

// SIMD processing function
unsafe fn process_data_simd(data: &[base]) {
    let mut result_vec = ls::set1::<true, target>::apply(0);
    let all_ones = ls::set1::<true, target>::apply(1);
    for chunk in data.chunks_exact(8) {
        let vec_age = ls::loadu::<true, target>::apply(chunk.as_ptr());
        let vec_val = ls::set1::<true, target>::apply((40));
        let result_mask = compare::less_than::<true, target>::apply((vec_age, vec_val));
        result_vec = calc::mask_add::<true, target>::apply((result_mask, result_vec, all_ones));
    }
    let mut result_arr = [0;target::vector_element_count()];
    ls::storeu::<true, target>::apply((result_arr.as_mut_ptr(), result_vec));
    let sum: u32 = result_arr.iter().map(|&x| x as u32).sum();
    // Result is 2518
}

// Benchmark function
fn simd_benchmark(c: &mut Criterion) {
    let data = load_data_into_memory("/home/dertuchi/i32_data.bin").expect("Failed to load data");

    c.bench_function("simd_processing", |b| {
        b.iter(|| unsafe {
            process_data_simd(&data)
        })
    });
}

criterion_group!(benches, simd_benchmark);
criterion_main!(benches);
