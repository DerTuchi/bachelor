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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/mask_ls/mask_ls_avx2.rs
 * \date 2024-05-05
 * \brief Load/Store primitives on masked registers
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8-1-ga6bbe75 (a6bbe756f616e7ae096743b00267b33d7e112930)
 *
 */
use std::arch::x86_64::*;
use crate::generated::declarations::mask_ls::*;
use crate::generated::extensions::simd::intel::avx2::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool , const N : i32> SimdPrimitiveImpl for mask_gather<Idof, N, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "mask_gather" (primitive mask_gather).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/mask_ls.yaml::513
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, *const Self::BaseType, Self::OffsetBaseRegisterType);
    type ReturnType = Self::RegisterType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    
    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (mask, source, memory, index) = args;
        _mm256_mask_i64gather_epi64(source, memory, index, mask, N)
    }
} // end of struct mask_gather for template specialization of mask_gather for avx2 using i64.


impl< const Idof: bool , const N : i32> SimdPrimitiveImpl for mask_gather<Idof, N, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "mask_gather" (primitive mask_gather).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/mask_ls.yaml::513
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, *const Self::BaseType, Self::OffsetBaseRegisterType);
    type ReturnType = Self::RegisterType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    
    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (mask, source, memory, index) = args;
        _mm256_mask_i32gather_epi32(source, memory, index, mask, N)
    }
} // end of struct mask_gather for template specialization of mask_gather for avx2 using i32.


impl< const Idof: bool , const N : i32> SimdPrimitiveImpl for mask_gather<Idof, N, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "mask_gather" (primitive mask_gather).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/mask_ls.yaml::517
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, *const Self::BaseType, Self::OffsetBaseRegisterType);
    type ReturnType = Self::RegisterType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    
    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (mask, source, memory, index) = args;
        _mm256_mask_i64gather_epi64(source, memory as *const i64, index, mask, N)
    }
} // end of struct mask_gather for template specialization of mask_gather for avx2 using u64.


impl< const Idof: bool , const N : i32> SimdPrimitiveImpl for mask_gather<Idof, N, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "mask_gather" (primitive mask_gather).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/mask_ls.yaml::517
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, *const Self::BaseType, Self::OffsetBaseRegisterType);
    type ReturnType = Self::RegisterType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    
    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (mask, source, memory, index) = args;
        _mm256_mask_i32gather_epi32(source, memory as *const i32, index, mask, N)
    }
} // end of struct mask_gather for template specialization of mask_gather for avx2 using u32.


impl< const Idof: bool , const N : i32> SimdPrimitiveImpl for mask_gather<Idof, N, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "mask_gather" (primitive mask_gather).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/mask_ls.yaml::521
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, *const Self::BaseType, Self::OffsetBaseRegisterType);
    type ReturnType = Self::RegisterType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    
    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (mask, source, memory, index) = args;
        _mm256_mask_i32gather_ps(source, memory, index, mask, N)
    }
} // end of struct mask_gather for template specialization of mask_gather for avx2 using f32.


impl< const Idof: bool , const N : i32> SimdPrimitiveImpl for mask_gather<Idof, N, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "mask_gather" (primitive mask_gather).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/mask_ls.yaml::525
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, *const Self::BaseType, Self::OffsetBaseRegisterType);
    type ReturnType = Self::RegisterType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    
    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (mask, source, memory, index) = args;
        _mm256_mask_i64gather_pd(source, memory, index, mask, N)
    }
} // end of struct mask_gather for template specialization of mask_gather for avx2 using f64.


