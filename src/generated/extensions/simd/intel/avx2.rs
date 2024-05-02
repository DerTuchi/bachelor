/*==========================================================================*
 * This file is part of the TSL - a template SIMD library.                  *
 *                                                                          *
 * Copyright 2024 TSL-Team, Database Research Group TU Dresden              *
 *                                                                          *
 * Licensed under the Apache License, Version 2.0 (the "License");          *
 * you may not use this file except in compliance with the License.         *
 * You may obtain a copy of the License at                                  *
 *                                                                          *
 *     http://www.apache.org/licenses/LICENSE-2.0                           *
 *                                                                          *
 * Unless required by applicable law or agreed to in writing, software      *
 * distributed under the License is distributed on an "AS IS" BASIS,        *
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. *
 * See the License for the specific language governing permissions and      *
 * limitations under the License.                                           *
 *==========================================================================*/
/*
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/extensions/simd/intel/avx2.rs
 * \date 2024-05-02
 * \brief Definition of the SIMD TargetExtension avx2.
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8 (7302664ad7b976795a660a3a21d6f31554148172)
 *
 */
use std::marker::PhantomData;
use crate::static_files::TSLArithmetic;
use std::arch::x86_64::*;
use crate::static_files::simd_traits::*;


// Trait to select avx2 register_type
pub trait SelectRegisteravx2 {
    type Output;
}
macro_rules! impl_register_avx2 {
    ($($val:ty => $reg:ty),*) => {
        $(impl SelectRegisteravx2 for $val {
            type Output = $reg;
        })*
    };
}
impl_register_avx2!{
    u8 => __m256i, 
    u16 => __m256i, 
    u32 => __m256i, 
    u64 => __m256i, 
    i8 => __m256i, 
    i16 => __m256i, 
    i32 => __m256i, 
    i64 => __m256i, 
    f32 => __m256, 
    f64 => __m256d
}

// Trait to select Imask Type
pub trait SelectImaskavx2{
    type Output;
}
macro_rules! impl_imask_avx2 {
    ($($val:ty => $reg:ty),*) => {
        $(impl SelectImaskavx2 for $val {
            type Output = $reg;
        })*
    };
}

impl_imask_avx2!{
     u8 => u32, 
     u16 => u16, 
     u32 => u8, 
     u64 => u8, 
     i8 => u32, 
     i16 => u16, 
     i32 => u8, 
     i64 => u8, 
     f32 => u8, 
     f64 => u8
}

pub struct avx2<T: TSLArithmetic>(pub PhantomData<T>);
impl<T> avx2<T> 
where
    T : TSLArithmetic + SelectRegisteravx2 + SelectImaskavx2 + SelectOffsetBaseType,
        <T as SelectOffsetBaseType>::Output: SelectRegisteravx2
{
    const DEFAULT_SIZE_IN_BITS : usize = 128;

    pub const fn vector_size_b() -> usize{
        Self::DEFAULT_SIZE_IN_BITS
    }
    pub const fn vector_size_bytes() -> usize{
        Self::vector_element_count() * std::mem::size_of::<T>()
    }
    pub const fn vector_element_count() -> usize{
        Self::vector_size_b() / (std::mem::size_of::<T>() * 8)
    }
    pub const fn vector_alignment() -> usize {
        if Self::vector_size_bytes() > 32 {
            64
        } else {
            Self::vector_size_bytes()
        }
    }
    pub const fn vector_mask_ratio() -> usize {
        (std::mem::size_of::<<Self as TargetExtension>::MaskType>() * 8) / Self::vector_element_count()
    }
    pub const fn mask_shift() -> usize {
        Self::vector_element_count()
    }
}

impl<T> TargetExtension for avx2<T>
where
    T : TSLArithmetic + SelectRegisteravx2 + SelectImaskavx2 + SelectOffsetBaseType,
        <T as SelectOffsetBaseType>::Output: SelectRegisteravx2
{
    const DEFAULT_SIZE_IN_BITS : usize = Self::DEFAULT_SIZE_IN_BITS;
    type BaseType = T;
    type RegisterType = <T as SelectRegisteravx2>::Output;
    type MaskType = Self::RegisterType;
    type ImaskType = <T as SelectImaskavx2>::Output;
    type OffsetBaseType = <T as SelectOffsetBaseType>::Output;
    type OffsetBaseRegisterType = <Self::OffsetBaseType as SelectRegisteravx2>::Output;
}


