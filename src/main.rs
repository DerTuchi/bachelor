use std::marker::PhantomData;
use std::arch::x86_64::*;

mod constants;
mod generated;

use generated::declarations::calc::add;
use generated::extensions::sse::Sse;

use crate::constants::simd::simd_primitive_trait_bounds::SimdPrimitiveImpl;
use crate::constants::simd::simd_type_trait_bounds::{TargetExtensionTypes, VectorProcessingStyle};
use crate::constants::utils::helper_traits::TransformType;
use crate::constants::simd::simd_type::Simd;

fn main() {
    type SimdType = Simd<u64, Sse, 128>;
    let simd_instance = SimdType{_phantom_base: PhantomData, _phantom_ext: PhantomData,};
    let element_count = simd_instance.vector_element_count();


    let vec_a = unsafe { _mm_set1_epi8(0) };
    let vec_b = unsafe { _mm_set1_epi8(2) };
    
    let result = add::<u8, Sse, 128>(PhantomData, PhantomData).apply((vec_a, vec_b));

    let temp : <Sse as TransformType<f64, 128>>::Output = unsafe{ _mm_set1_pd(2.3)};

    let temp2: <Sse as TargetExtensionTypes<i64, 128>>::RegisterType = unsafe {_mm_set1_epi64x(64)};

    println!("Result of addition: {:?}", result);
    println!("Result of temp: {:?}", temp);
    println!("Result of temp2: {:?}", temp2);
    println!("Result of element_count: {:?}", element_count);
}
