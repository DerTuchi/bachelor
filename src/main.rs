#![allow(warnings)]

use std::arch::x86_64::*;

mod static_files;
mod generated;


use generated::extensions::simd::intel::sse::sse;
use generated::extensions::simd::intel::avx2::avx2;
use generated::declarations::convert::*;


use crate::{generated::extensions::scalar::scalar, static_files::simd_traits::{SimdPrimitiveImpl, TargetExtension}};

fn main() {
    let element_count = sse::<u8>::vector_element_count();

    let vec_a = unsafe { _mm256_set1_epi8(0) };
    let vec_b = unsafe { _mm256_set1_epi8(2) };
    let result = generated::declarations::compare::less_than::<true, avx2<i8>>::apply((vec_a, vec_b));

    let temp : <sse<f64> as TargetExtension>::RegisterType = unsafe{ _mm_set1_pd(2.3)};

    let temp2: <sse<i64> as TargetExtension>::RegisterType = unsafe {_mm_set1_epi64x(500)};

    let temp3  = unsafe{_mm_set1_epi32(20)};
    let base_cast = cast::<true, sse<f32>, sse<i32>>::apply((temp3));
    let target_cast = cast::<true, avx2<i32>, sse<i32>>::apply((temp3));

    println!("Result of addition: {:?}", result);
    println!("Result of temp: {:?}", temp);
    println!("Result of temp2: {:?}", temp2);
    println!("Result of element_count: {:?}", element_count);
    println!("------------------------------------------------");
    println!("Before Cast: {:?}", temp3);
    println!("Base Cast: {:?}", base_cast);
    println!("Target Cast: {:?}", target_cast);
}
