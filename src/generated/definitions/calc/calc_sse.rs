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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/calc/calc_sse.rs
 * \date 2024-05-05
 * \brief This file contains arithmetic primitives.
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8-1-ga6bbe75 (a6bbe756f616e7ae096743b00267b33d7e112930)
 *
 */
use std::arch::x86_64::*;
use crate::generated::declarations::calc::*;
use crate::generated::extensions::simd::intel::sse::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<u8>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: u8
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = u8;
    type TargetExtension = sse<u8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi8(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using u8.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<u16>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: u16
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = u16;
    type TargetExtension = sse<u16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi16(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using u16.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<u32>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: u32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = u32;
    type TargetExtension = sse<u32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi32(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using u32.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<u64>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: u64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = u64;
    type TargetExtension = sse<u64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi64(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using u64.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<i8>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: i8
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = i8;
    type TargetExtension = sse<i8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi8(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using i8.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<i16>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: i16
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = i16;
    type TargetExtension = sse<i16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi16(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using i16.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<i32>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: i32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = i32;
    type TargetExtension = sse<i32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi32(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using i32.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<i64>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: i64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::309
     */

    type BaseType = i64;
    type TargetExtension = sse<i64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_epi64(vec_a, _mm_and_si128(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using i64.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/calc.yaml::313
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_ps(vec_a, _mm_and_ps(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for mask_add<Idof, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "mask_add" (primitive mask_add).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/calc.yaml::317
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::MaskType, Self::RegisterType, Self::RegisterType);
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
        let (mask, vec_a, vec_b) = args;
        _mm_add_pd(vec_a, _mm_and_pd(vec_b, mask))
    }
} // end of struct mask_add for template specialization of mask_add for sse using f64.


