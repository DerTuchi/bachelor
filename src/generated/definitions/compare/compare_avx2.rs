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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/compare/compare_avx2.rs
 * \date 2024-05-02
 * \brief Compare primitives.
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8 (7302664ad7b976795a660a3a21d6f31554148172)
 *
 */
use std::arch::x86_64::*;
use crate::generated::declarations::compare::*;
use crate::generated::extensions::simd::intel::avx2::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/compare.yaml::599
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_cmpgt_epi8(vec_b, vec_a)
    }
} // end of struct less_than for template specialization of less_than for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/compare.yaml::599
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_cmpgt_epi16(vec_b, vec_a)
    }
} // end of struct less_than for template specialization of less_than for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/compare.yaml::599
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_cmpgt_epi32(vec_b, vec_a)
    }
} // end of struct less_than for template specialization of less_than for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/compare.yaml::599
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_cmpgt_epi64(vec_b, vec_a)
    }
} // end of struct less_than for template specialization of less_than for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/compare.yaml::603
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_cmp_ps(vec_a, vec_b, _CMP_LT_OQ)
    }
} // end of struct less_than for template specialization of less_than for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/compare.yaml::603
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_cmp_pd(vec_a, vec_b, _CMP_LT_OQ)
    }
} // end of struct less_than for template specialization of less_than for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/compare.yaml::607
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_andnot_si256(
_mm256_cmpeq_epi8(vec_a, vec_b),
_mm256_cmpeq_epi8(vec_a, _mm256_min_epu8(vec_a, vec_b)))
    }
} // end of struct less_than for template specialization of less_than for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/compare.yaml::607
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_andnot_si256(
_mm256_cmpeq_epi16(vec_a, vec_b),
_mm256_cmpeq_epi16(vec_a, _mm256_min_epu16(vec_a, vec_b)))
    }
} // end of struct less_than for template specialization of less_than for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for less_than<Idof, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "less_than" (primitive less_than).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/compare.yaml::607
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::MaskType;
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return true;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (vec_a, vec_b) = args;
        _mm256_andnot_si256(
_mm256_cmpeq_epi32(vec_a, vec_b),
_mm256_cmpeq_epi32(vec_a, _mm256_min_epu32(vec_a, vec_b)))
    }
} // end of struct less_than for template specialization of less_than for avx2 using u32.


