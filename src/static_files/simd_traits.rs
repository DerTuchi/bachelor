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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/static_files/simd_traits.rs
 * \date 2024-05-05
 * \brief Traits for TargetExtension and Primitive Impl handling.
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8-1-ga6bbe75 (a6bbe756f616e7ae096743b00267b33d7e112930)
 *
 */
use super::TSLArithmetic;


pub trait SimdPrimitiveImpl{
  type Args;
  type ReturnType;
  type BaseType;
  type TargetExtension;
  type AdditionalParam;
  type RegisterType;
  type ImaskType;
  type MaskType;
  type OffsetBaseType;
  type OffsetBaseRegisterType;
  const is_native: bool;
  const check: ();

  fn parameters_queryable() -> bool;
  fn has_return_value() -> bool;
  fn native_supported() -> bool {
    Self::is_native
  }
  unsafe fn apply(args: Self::Args) -> Self::ReturnType;
}

pub trait TargetExtension{
  const DEFAULT_SIZE_IN_BITS : usize;
  type BaseType: TSLArithmetic;
  type RegisterType;
  type MaskType;
  type ImaskType;
  type OffsetBaseType: TSLArithmetic;
  type OffsetBaseRegisterType;
}

// Trait to select OffsetBaseType
pub trait SelectOffsetBaseType {
    type Output : TSLArithmetic;
}
macro_rules! impl_signed_to_unsigned {
    ($($signed:ty => $unsigned:ty),*; $($selfmap:ty),*) => {
        // Map signed types to their unsigned equivalents
        $(impl SelectOffsetBaseType for $signed where $signed : TSLArithmetic {
            type Output = $unsigned;
        })*

        // Map unsigned types to themselves
        $(impl SelectOffsetBaseType for $selfmap where $selfmap : TSLArithmetic{
            type Output = $selfmap;
        })*
    };
}
impl_signed_to_unsigned!{
    i8 => u8,
    i16 => u16,
    i32 => u32,
    i64 => u64,
    i128 => u128,
    isize => usize,
    f32 => u32,
    f64 => u64;
    u8, u16, u32, u64, u128, usize
}


