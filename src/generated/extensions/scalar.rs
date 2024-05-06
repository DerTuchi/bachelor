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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/extensions/scalar.rs
 * \date 2024-05-05
 * \brief Definition of the SIMD TargetExtension sse.
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8-1-ga6bbe75 (a6bbe756f616e7ae096743b00267b33d7e112930)
 *
 */
use std::marker::PhantomData;
use crate::static_files::TSLArithmetic;
use crate::static_files::simd_traits::*;


// Trait to select scalar register_type
pub trait SelectRegisterscalar {
    type Output;
}
impl<T: TSLArithmetic> SelectRegisterscalar for T{
    type Output = T;
}

// Trait to select Imask Type
pub trait SelectImaskscalar{
    type Output;
}
impl<T: TSLArithmetic> SelectImaskscalar for T{
    type Output = u8; // maybe use bool instead?
}

pub struct scalar<T: TSLArithmetic>(pub PhantomData<T>);
impl<T> scalar<T> 
where
    T : TSLArithmetic + SelectRegisterscalar + SelectImaskscalar + SelectOffsetBaseType,
        <T as SelectOffsetBaseType>::Output: SelectRegisterscalar
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

impl<T> TargetExtension for scalar<T>
where
    T : TSLArithmetic + SelectRegisterscalar + SelectImaskscalar + SelectOffsetBaseType,
        <T as SelectOffsetBaseType>::Output: SelectRegisterscalar
{
    const DEFAULT_SIZE_IN_BITS : usize = Self::DEFAULT_SIZE_IN_BITS;
    type BaseType = T;
    type RegisterType = <T as SelectRegisterscalar>::Output;
    type MaskType = bool;
    type ImaskType = <T as SelectImaskscalar>::Output;
    type OffsetBaseType = <T as SelectOffsetBaseType>::Output;
    type OffsetBaseRegisterType = <Self::OffsetBaseType as SelectRegisterscalar>::Output;
}


