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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/convert/convert_sse.rs
 * \date 2024-05-06
 * \brief Conversion primitives.
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8-1-ga6bbe75 (a6bbe756f616e7ae096743b00267b33d7e112930)
 *
 */
use std::arch::x86_64::*;
use crate::generated::extensions::simd::intel::avx2::avx2;
use std::mem;
use crate::generated::declarations::convert::*;
use crate::generated::extensions::simd::intel::sse::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i8>, sse<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i8
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = i8;
    type TargetExtension = sse<i8>;
    type AdditionalParam = sse<i8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u8>, sse<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u8
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = u8;
    type TargetExtension = sse<u8>;
    type AdditionalParam = sse<u8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u16>, sse<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u16
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = u16;
    type TargetExtension = sse<u16>;
    type AdditionalParam = sse<u16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i16>, sse<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i16
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = i16;
    type TargetExtension = sse<i16>;
    type AdditionalParam = sse<i16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i32>, sse<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i32
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = i32;
    type TargetExtension = sse<i32>;
    type AdditionalParam = sse<i32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u32>, sse<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u32
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = u32;
    type TargetExtension = sse<u32>;
    type AdditionalParam = sse<u32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i64>, sse<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i64
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = i64;
    type TargetExtension = sse<i64>;
    type AdditionalParam = sse<i64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u64>, sse<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u64
     *  Extension Flags: ['sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::84
     */

    type BaseType = u64;
    type TargetExtension = sse<u64>;
    type AdditionalParam = sse<u64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i8>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i8
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = i8;
    type TargetExtension = sse<i8>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u8>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u8
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = u8;
    type TargetExtension = sse<u8>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u16>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u16
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = u16;
    type TargetExtension = sse<u16>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i16>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i16
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = i16;
    type TargetExtension = sse<i16>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i32>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = i32;
    type TargetExtension = sse<i32>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u32>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = u32;
    type TargetExtension = sse<u32>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i64>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = i64;
    type TargetExtension = sse<i64>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u64>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::89
     */

    type BaseType = u64;
    type TargetExtension = sse<u64>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<i8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<u8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<u16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<i16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<i32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<u32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<i64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f32>, sse<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::94
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<u64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castps_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i8>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i8
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = i8;
    type TargetExtension = sse<i8>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u8>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u8
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = u8;
    type TargetExtension = sse<u8>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u16>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u16
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = u16;
    type TargetExtension = sse<u16>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i16>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i16
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = i16;
    type TargetExtension = sse<i16>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i32>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = i32;
    type TargetExtension = sse<i32>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u32>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = u32;
    type TargetExtension = sse<u32>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<i64>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: i64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = i64;
    type TargetExtension = sse<i64>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<u64>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: u64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::99
     */

    type BaseType = u64;
    type TargetExtension = sse<u64>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castsi128_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<i8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<u8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<u16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<i16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<i32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<u32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<i64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, sse<f64>, sse<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::104
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<u64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_castpd_si128(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<i8>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: i8
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = i8;
    type TargetExtension = sse<i8>;
    type AdditionalParam = avx2<i8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using i8.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<u8>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: u8
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = u8;
    type TargetExtension = sse<u8>;
    type AdditionalParam = avx2<u8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using u8.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<i16>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: i16
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = i16;
    type TargetExtension = sse<i16>;
    type AdditionalParam = avx2<i16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using i16.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<u16>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: u16
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = u16;
    type TargetExtension = sse<u16>;
    type AdditionalParam = avx2<u16>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using u16.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<i32>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: i32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = i32;
    type TargetExtension = sse<i32>;
    type AdditionalParam = avx2<i32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using i32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<u32>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: u32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = u32;
    type TargetExtension = sse<u32>;
    type AdditionalParam = avx2<u32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using u32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<i64>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: i64
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = i64;
    type TargetExtension = sse<i64>;
    type AdditionalParam = avx2<i64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using i64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<u64>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: u64
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::279
     */

    type BaseType = u64;
    type TargetExtension = sse<u64>;
    type AdditionalParam = avx2<u64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm256_zextsi128_si256(data)
    }
} // end of struct cast for template specialization of cast for sse using u64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<f32>, sse<i32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse4_1', 'sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::312
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<i32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_cvtps_epi32(_mm_round_ps(data, _MM_FROUND_TO_ZERO))
    }
} // end of struct cast for template specialization of cast for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<f32>, sse<u32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: f32
     *  Extension Flags: ['sse4_1', 'sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::312
     */

    type BaseType = f32;
    type TargetExtension = sse<f32>;
    type AdditionalParam = sse<u32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_cvtps_epi32(_mm_round_ps(data, _MM_FROUND_TO_ZERO))
    }
} // end of struct cast for template specialization of cast for sse using f32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<f64>, sse<i64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse4_1', 'sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::317
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<i64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_cvtepi32_epi64(_mm_cvtps_epi32(_mm_round_ps(_mm_cvtpd_ps(_mm_round_pd(data, _MM_FROUND_TO_ZERO)), _MM_FROUND_TO_ZERO)))
    }
} // end of struct cast for template specialization of cast for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<f64>, sse<u64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: f64
     *  Extension Flags: ['sse4_1', 'sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::317
     */

    type BaseType = f64;
    type TargetExtension = sse<f64>;
    type AdditionalParam = sse<u64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_cvtepi32_epi64(_mm_cvtps_epi32(_mm_round_ps(_mm_cvtpd_ps(_mm_round_pd(data, _MM_FROUND_TO_ZERO)), _MM_FROUND_TO_ZERO)))
    }
} // end of struct cast for template specialization of cast for sse using f64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<i32>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: i32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::381
     */

    type BaseType = i32;
    type TargetExtension = sse<i32>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        _mm_cvtepi32_ps(data)
    }
} // end of struct cast for template specialization of cast for sse using i32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<u32>, sse<f32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: u32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::386
     */

    type BaseType = u32;
    type TargetExtension = sse<u32>;
    type AdditionalParam = sse<f32>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        let temp = _mm_cvtepi32_ps(data);

let mask = _mm_srai_epi32(data, 31);
let correction = _mm_castsi128_ps(_mm_and_si128(mask, _mm_set1_epi32(0x4f800000 as i32)));

_mm_add_ps(temp, correction)
    }
} // end of struct cast for template specialization of cast for sse using u32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, sse<u64>, sse<f64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: sse.
     *        Data Type: u64
     *  Extension Flags: ['sse2', 'sse4_1']
     *      Yaml Source: primitive_data/primitives/convert.yaml::412
     */

    type BaseType = u64;
    type TargetExtension = sse<u64>;
    type AdditionalParam = sse<f64>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = <Self::AdditionalParam as TargetExtension>::RegisterType;
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
        let (data) = args;
        let value: u64 = 0x0010000000000000;
let magic_no: f64 = mem::transmute(value);
// https://stackoverflow.com/a/41148578
let mut xH = _mm_srli_epi64(data, 32);
xH = _mm_or_si128(xH, _mm_castpd_si128(_mm_set1_pd(19342813113834066795298816.)));            //  2^84
let xL = _mm_blend_epi16(data, _mm_castpd_si128(_mm_set1_pd(magic_no)), 0xcc);  //  2^52
let f = _mm_sub_pd(_mm_castsi128_pd(xH), _mm_set1_pd(19342813118337666422669312.));       //  2^84 + 2^52
_mm_add_pd(f, _mm_castsi128_pd(xL))
    }
} // end of struct cast for template specialization of cast for sse using u64.


