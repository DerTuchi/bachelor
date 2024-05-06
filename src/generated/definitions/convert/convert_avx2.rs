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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/convert/convert_avx2.rs
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
use crate::generated::declarations::convert::*;
use crate::generated::extensions::simd::intel::avx2::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
    type AdditionalParam = avx2<f32>;
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
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::18
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
    type AdditionalParam = avx2<f64>;
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
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::23
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::23
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::23
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::23
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::23
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::23
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::23
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::28
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::28
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::28
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::28
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::28
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::28
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::28
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::33
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::33
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::33
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::33
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::33
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::33
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::33
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::38
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::38
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::38
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::38
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::38
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::38
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::38
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::43
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::43
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::43
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::43
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::43
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::43
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::43
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::48
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::48
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::48
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::48
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::48
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::48
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::48
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::53
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::53
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::53
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::53
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::53
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::53
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::53
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::58
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::58
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::58
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::58
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::58
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::58
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::58
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
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
        data
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::63
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_castsi256_ps(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f32>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::68
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_castps_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i8>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u8>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u16>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i16>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i32>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u32>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<i64>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<u64>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::73
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
    type AdditionalParam = avx2<f64>;
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
        _mm256_castsi256_pd(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for reinterpret<Idof, avx2<f64>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "reinterpret" (primitive reinterpret).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::78
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_castpd_si256(data)
    }
} // end of struct reinterpret for template specialization of reinterpret for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, avx2<f32>, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::301
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_cvtps_epi32(_mm256_round_ps(data, _MM_FROUND_TO_ZERO))
    }
} // end of struct cast for template specialization of cast for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, avx2<f32>, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::301
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
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
        _mm256_cvtps_epi32(_mm256_round_ps(data, _MM_FROUND_TO_ZERO))
    }
} // end of struct cast for template specialization of cast for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, avx2<f64>, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx2', 'sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::306
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_cvtepi32_epi64(_mm_cvtps_epi32(_mm_round_ps(_mm256_cvtpd_ps(_mm256_round_pd(data, _MM_FROUND_TO_ZERO)), _MM_FROUND_TO_ZERO)))
    }
} // end of struct cast for template specialization of cast for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, avx2<f64>, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx2', 'sse']
     *      Yaml Source: primitive_data/primitives/convert.yaml::306
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
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
        _mm256_cvtepi32_epi64(_mm_cvtps_epi32(_mm_round_ps(_mm256_cvtpd_ps(_mm256_round_pd(data, _MM_FROUND_TO_ZERO)), _MM_FROUND_TO_ZERO)))
    }
} // end of struct cast for template specialization of cast for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, avx2<i32>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/convert.yaml::331
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = avx2<f32>;
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
        _mm256_cvtepi32_ps(data)
    }
} // end of struct cast for template specialization of cast for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, avx2<u32>, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx', 'avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::336
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = avx2<f32>;
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
        let output = _mm256_cvtepi32_ps(data);
let mask = _mm256_srai_epi32(data, 31);
let correction = _mm256_castsi256_ps(_mm256_and_si256(mask, _mm256_set1_epi32(0x4f800000 as i32)));

_mm256_add_ps(output, correction)
    }
} // end of struct cast for template specialization of cast for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for cast<Idof, avx2<u64>, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "cast" (primitive cast).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx', 'avx2']
     *      Yaml Source: primitive_data/primitives/convert.yaml::363
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
    type AdditionalParam = avx2<f64>;
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
        // https://stackoverflow.com/a/41223013
let magic_i_lo   = _mm256_set1_epi64x(0x4330000000000000);                /* 2^52        encoded as floating-point  */
let magic_i_hi32 = _mm256_set1_epi64x(0x4530000000000000);                /* 2^84        encoded as floating-point  */
let magic_i_all  = _mm256_set1_epi64x(0x4530000000100000);                /* 2^84 + 2^52 encoded as floating-point  */
let magic_d_all  = _mm256_castsi256_pd(magic_i_all);

let v_lo         = _mm256_blend_epi32(magic_i_lo, data, 0b01010101);      /* Blends the 32 lowest significant bits of v with magic_int_lo                                                   */
let mut v_hi         = _mm256_srli_epi64(data, 32);                           /* Extract the 32 most significant bits of v                                                                     */
        v_hi         = _mm256_xor_si256(v_hi, magic_i_hi32);                  /* Blends v_hi with 0x45300000                                                                                    */
let v_hi_dbl     = _mm256_sub_pd(_mm256_castsi256_pd(v_hi), magic_d_all); /* Compute in double precision:                                                                                  */
_mm256_add_pd(v_hi_dbl, _mm256_castsi256_pd(v_lo))    /* (v_hi - magic_d_all) + v_lo  Do not assume associativity of floating point addition !!                        */
    }
} // end of struct cast for template specialization of cast for avx2 using u64.


