use std::arch::x86_64::*;
use crate::constants::simd::TSLArithmetic;
use crate::constants::simd::simd_type_trait_bounds::TargetExtensionTypes;
use crate::constants::utils::helper_traits::TransformType;

// Trait to select SSE register_type
pub trait SelectRegisterSSE {
    type Output;
}
macro_rules! impl_register_SSE {
    ($($val:ty => $reg:ty),*) => {
        $(impl SelectRegisterSSE for $val {
            type Output = $reg;
        })*
    };
}
impl_register_SSE!{
    i8 => __m128i, i16 => __m128i, i32 => __m128i, i64 => __m128i, isize => __m128i,
    u8 => __m128i, u16 => __m128i, u32 => __m128i, u64 => __m128i, usize => __m128i,
    f32 => __m128, f64 => __m128d
}

// Trait to select Imask Type
pub trait SelectImaskSSE{
    type Output;
}

macro_rules! impl_imask_SSE {
    ($($val1:ty),*; $($val2:ty),*) => {
        $(
            impl SelectImaskSSE for $val1 {
                type Output = u16;
            }
        )*
        $(
            impl SelectImaskSSE for $val2{
                type Output = u8;
            }
        )*
    };
}

impl_imask_SSE!{
    u8, i8; 
    u16, i16, u32, i32, u64, i64, f32, f64
}


// Implementation of Transformtype Trait
impl <OtherType> TransformType<OtherType, 128> for Sse
where
OtherType: TSLArithmetic + SelectRegisterSSE + SelectImaskSSE
{
    type Output = <Self as TargetExtensionTypes<OtherType, 128>>::RegisterType;
}

pub struct Sse;
impl<T: TSLArithmetic + SelectRegisterSSE + SelectImaskSSE> TargetExtensionTypes<T, 128> for Sse { 
    type RegisterType = <T as SelectRegisterSSE>::Output;   // needs to be like that because no attributes are available in Rust
    type MaskType = Self::RegisterType;
    type ImaskType = <T as SelectImaskSSE>::Output; // needs to be like that because no attributes are available in Rust
}