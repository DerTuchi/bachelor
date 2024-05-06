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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/convert/convert_scalar.rs
 * \date 2024-05-06
 * \brief Conversion primitives.
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8-1-ga6bbe75 (a6bbe756f616e7ae096743b00267b33d7e112930)
 *
 */
use crate::generated::extensions::simd::intel::sse::sse;
use std::arch::x86_64::*;
use crate::generated::declarations::convert::*;
use crate::generated::extensions::scalar::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<i8>, scalar<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = i8;
    type TargetExtension = scalar<i8>;
    type AdditionalParam = scalar<i8>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<u8>, scalar<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = u8;
    type TargetExtension = scalar<u8>;
    type AdditionalParam = scalar<u8>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<u16>, scalar<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = u16;
    type TargetExtension = scalar<u16>;
    type AdditionalParam = scalar<u16>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<i16>, scalar<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = i16;
    type TargetExtension = scalar<i16>;
    type AdditionalParam = scalar<i16>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<i32>, scalar<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = i32;
    type TargetExtension = scalar<i32>;
    type AdditionalParam = scalar<i32>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<u32>, scalar<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = u32;
    type TargetExtension = scalar<u32>;
    type AdditionalParam = scalar<u32>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<i64>, scalar<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = i64;
    type TargetExtension = scalar<i64>;
    type AdditionalParam = scalar<i64>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, scalar<u64>, scalar<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::110
     */

    type BaseType = u64;
    type TargetExtension = scalar<u64>;
    type AdditionalParam = scalar<u64>;
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
} // end of struct reinterpret for template specialization of reinterpret for scalar using u64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<i32>, sse<i32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::286
     */

    type BaseType = i32;
    type TargetExtension = scalar<i32>;
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
        _mm_cvtsi32_si128(data)
    }
} // end of struct cast for template specialization of cast for scalar using i32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<i64>, sse<i64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::286
     */

    type BaseType = i64;
    type TargetExtension = scalar<i64>;
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
        _mm_cvtsi64_si128(data)
    }
} // end of struct cast for template specialization of cast for scalar using i64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<u32>, sse<u32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u32
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::293
     */

    type BaseType = u32;
    type TargetExtension = scalar<u32>;
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
        _mm_cvtsi32_si128(data as i32)
    }
} // end of struct cast for template specialization of cast for scalar using u32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<u64>, sse<u64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u64
     *  Extension Flags: ['sse2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::293
     */

    type BaseType = u64;
    type TargetExtension = scalar<u64>;
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
        _mm_cvtsi64_si128(data as i64)
    }
} // end of struct cast for template specialization of cast for scalar using u64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<f32>, scalar<i32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::323
     */

    type BaseType = f32;
    type TargetExtension = scalar<f32>;
    type AdditionalParam = scalar<i32>;
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
        data as i32
    }
} // end of struct cast for template specialization of cast for scalar using f32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<f64>, scalar<i64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::323
     */

    type BaseType = f64;
    type TargetExtension = scalar<f64>;
    type AdditionalParam = scalar<i64>;
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
        data as i64
    }
} // end of struct cast for template specialization of cast for scalar using f64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<u32>, scalar<f32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::427
     */

    type BaseType = u32;
    type TargetExtension = scalar<u32>;
    type AdditionalParam = scalar<f32>;
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
        data as f32
    }
} // end of struct cast for template specialization of cast for scalar using u32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<u64>, scalar<f64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::427
     */

    type BaseType = u64;
    type TargetExtension = scalar<u64>;
    type AdditionalParam = scalar<f64>;
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
        data as f64
    }
} // end of struct cast for template specialization of cast for scalar using u64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<i32>, scalar<f32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::427
     */

    type BaseType = i32;
    type TargetExtension = scalar<i32>;
    type AdditionalParam = scalar<f32>;
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
        data as f32
    }
} // end of struct cast for template specialization of cast for scalar using i32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, scalar<i64>, scalar<f64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/convert.yaml::427
     */

    type BaseType = i64;
    type TargetExtension = scalar<i64>;
    type AdditionalParam = scalar<f64>;
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
        data as f64
    }
} // end of struct cast for template specialization of cast for scalar using i64.


