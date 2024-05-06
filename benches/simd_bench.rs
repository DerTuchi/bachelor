#![allow(warnings)]

use bachelor::static_files::simd_traits::{SimdPrimitiveImpl, TargetExtension};
use criterion::{criterion_group, criterion_main, Criterion};

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem;
use std::ptr;
use std::slice;
use byteorder::{LittleEndian, ReadBytesExt, ByteOrder};

use bachelor::generated::declarations::*;
use bachelor::generated::extensions::simd::intel::avx2::avx2;
use bachelor::generated::extensions::simd::intel::sse::sse;
use bachelor::static_files::*;

fn load_data_into_memory<T>(filename: &str) -> io::Result<(Vec<T>, usize)> {
    let mut file = File::open(filename)?;
    let file_size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?; // Reset to the beginning of the file
    let num_elements = (file_size as usize) / mem::size_of::<T>();
    let mut buffer = Vec::<T>::with_capacity(num_elements);
    unsafe {
        buffer.set_len(num_elements);
        let buffer_ptr = buffer.as_mut_ptr() as *mut u8;
        let buffer_slice = slice::from_raw_parts_mut(buffer_ptr, file_size as usize);
        file.read_exact(buffer_slice)?;
    }

    Ok((buffer, num_elements))
}

// SIMD processing function
unsafe fn process_data_simd(data: &[base], size: usize, threshold: base) {
    let mut result_vec = ls::set1::<true, target>::apply(0);
    let all_ones = ls::set1::<true, target>::apply(1);
    for i in (0..size).step_by(target::vector_element_count()) {
        let offset_ptr = data.as_ptr().add(i);
        let vec_age = ls::loadu::<true, target>::apply(offset_ptr);
        let vec_val = ls::set1::<true, target>::apply((threshold));
        let result_mask = compare::less_than::<true, target>::apply((vec_age, vec_val));
        result_vec = calc::mask_add::<true, target>::apply((result_mask, result_vec, all_ones));
    }
    let mut result_arr = [0;target::vector_element_count()];
    ls::storeu::<true, target>::apply((result_arr.as_mut_ptr(), result_vec));
    let mut sum : u32 = 0;
    for &val in result_arr.iter(){
        sum += val as u32;
    }
    println!("Result: {}", sum);
}

// Benchmark function
fn simd_benchmark(c: &mut Criterion) {
    let (data, size) = load_data_into_memory("/home/dertuchi/data/int.bin").expect("Failed to load data");

    c.bench_function("simd_processing", |b| {
        b.iter(|| unsafe {
            process_data_simd(&data, size, 40)
        })
    });
}

type base = u32;
type target = avx2<base>;
criterion_group!(benches, simd_benchmark);
criterion_main!(benches);
