#![allow(warnings)]

use std::arch::x86_64::*;

mod static_files;
mod generated;

use generated::extensions::simd::intel::sse::sse;
use generated::extensions::simd::intel::avx2::avx2;
use generated::declarations::convert::*;
use generated::declarations::ls::*;
use generated::declarations::compare::*;
use crate::{generated::extensions::scalar::scalar, static_files::simd_traits::{SimdPrimitiveImpl, TargetExtension}};

fn main() {
    let element_count = sse::<u8>::vector_element_count();

    let vec_a = unsafe { _mm256_set1_epi8(0) };
    let vec_b = unsafe { _mm256_set1_epi8(2) };
    let result = unsafe{generated::declarations::compare::less_than::<true, avx2<i8>>::apply((vec_a, vec_b))};

    let temp : <sse<f64> as TargetExtension>::RegisterType = unsafe{ _mm_set1_pd(2.3)};

    let temp2: <sse<i64> as TargetExtension>::RegisterType = unsafe {_mm_set1_epi64x(500)};

    let temp3  = unsafe{_mm_set1_epi32(20)};
    let base_cast = unsafe{cast::<true, sse<i32>, sse<f32>>::apply((temp3))};
    let target_cast = unsafe{cast::<true, sse<i32>, avx2<i32>>::apply((temp3))};

    let memory: [i64; 4] = [1, 2, 3, 4];
    let mem_as_vec = unsafe{loadu::<true, avx2<i64>>::apply((memory.as_ptr()))};
    let mut mut_mem: [i64; 4] = [0; 4];
    unsafe{storeu::<true, avx2<i64>>::apply((mut_mem.as_mut_ptr(), mem_as_vec))};

    let test = unsafe{extract_value::<true, 2, avx2<i64>>::apply((mem_as_vec))};

    println!("Result of addition: {:?}", result);
    println!("Result of temp: {:?}", temp);
    println!("Result of temp2: {:?}", temp2);
    println!("Result of element_count: {:?}", element_count);
    println!("------------------------------------------------");
    println!("Before Cast: {:?}", temp3);
    println!("Base Cast: {:?}", base_cast);
    println!("Target Cast: {:?}", target_cast);
    println!("------------------------------------------------");
    println!("As Array: {:?}", memory);
    println!("As SIMD: {:?}", mem_as_vec);
    println!("Stored Data: {:?}", mut_mem);
    println!("Stored Data: {:?}", test);

}
