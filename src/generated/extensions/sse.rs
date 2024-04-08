use std::arch::x86_64::*;
use crate::constants::TSLArithmetic;
use crate::constants::simd_traits::TargetExtension;
use crate::constants::utils::helper_traits::SelectOffsetBaseType;
use std::marker::PhantomData;

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

pub struct Sse<T: TSLArithmetic>(pub PhantomData<T>);
impl<T> Sse<T> 
where
    T : TSLArithmetic + SelectRegisterSSE + SelectImaskSSE + SelectOffsetBaseType,
        <T as SelectOffsetBaseType>::Output: SelectRegisterSSE
{
    const DEFAULT_SIZE_IN_BITS : usize = 128;

    pub fn new() -> Self {
        Sse(PhantomData)
    }
    
    pub const fn vector_size_b(&self) -> usize{
        Self::DEFAULT_SIZE_IN_BITS
    }
    pub const fn vector_size_B(&self) -> usize{
        self.vector_element_count() * std::mem::size_of::<T>()
    }
    pub const fn vector_element_count(&self) -> usize{
        self.vector_size_b() / (std::mem::size_of::<T>() * 8)
    }
    pub const fn vector_alignment(&self) -> usize {
        if self.vector_size_B() > 32 {
            64
        } else {
            self.vector_size_B()
        }
    }
    pub const fn vector_mask_ratio(&self) -> usize {
        (std::mem::size_of::<<Self as TargetExtension>::MaskType>() * 8) / self.vector_element_count()
    }
    pub const fn mask_shift(&self) -> usize {
        self.vector_element_count()
    }
}

impl<T> TargetExtension for Sse<T>
where
    T : TSLArithmetic + SelectRegisterSSE + SelectImaskSSE + SelectOffsetBaseType,
        <T as SelectOffsetBaseType>::Output: SelectRegisterSSE
{
    const DEFAULT_SIZE_IN_BITS : usize = Self::DEFAULT_SIZE_IN_BITS;
    type BaseType = T;
    type RegisterType = <T as SelectRegisterSSE>::Output;   // needs to be like that because no attributes are available in Rust
    type MaskType = Self::RegisterType;
    type ImaskType = <T as SelectImaskSSE>::Output; // needs to be like that because no attributes are available in Rust
    type OffsetBaseType = <T as SelectOffsetBaseType>::Output;
    type OffsetBaseRegisterType = <Self::OffsetBaseType as SelectRegisterSSE>::Output;
}