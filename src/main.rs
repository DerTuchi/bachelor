use std::marker::PhantomData;
use std::arch::x86_64::*;

mod constants;
mod generated;

use generated::declarations::calc::Add;
use generated::extensions::sse::Sse;

use crate::constants::simd_traits::{SimdPrimitiveImpl, TargetExtension};

fn main() {
    let element_count = Sse::<u8>::new().vector_element_count();

    let vec_a = unsafe { _mm_set1_epi8(0) };
    let vec_b = unsafe { _mm_set1_epi8(2) };

    let result = Add::<Sse<u8>>::new().apply((vec_a, vec_b));

    let temp : <Sse<f64> as TargetExtension>::RegisterType = unsafe{ _mm_set1_pd(2.3)};

    let temp2: <Sse<i64> as TargetExtension>::RegisterType = unsafe {_mm_set1_epi64x(500)};

    println!("Result of addition: {:?}", result);
    println!("Result of temp: {:?}", temp);
    println!("Result of temp2: {:?}", temp2);
    println!("Result of element_count: {:?}", element_count);
}
