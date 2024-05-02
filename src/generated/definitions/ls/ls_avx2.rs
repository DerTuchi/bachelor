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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/ls/ls_avx2.rs
 * \date 2024-05-02
 * \brief Load/Store primitives
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8 (7302664ad7b976795a660a3a21d6f31554148172)
 *
 */
use std::arch::x86_64::*;
use crate::generated::declarations::ls::*;
use crate::generated::extensions::simd::intel::avx2::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::106
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_si256( memory as *const __m256i )
    }
} // end of struct loadu for template specialization of loadu for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::110
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_ps(memory)
    }
} // end of struct loadu for template specialization of loadu for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::110
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*const Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (memory) = args;
        _mm256_loadu_pd(memory)
    }
} // end of struct loadu for template specialization of loadu for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using u64.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::319
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_si256( memory as *mut __m256i, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::323
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_ps(memory, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::323
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (*mut Self::BaseType, Self::RegisterType);
    type ReturnType = ();
    const is_native: bool = true;
    const check: () = (); // function is native so no check for Idof needed.

    fn parameters_queryable() -> bool{
        return true;
    }
    fn has_return_value() -> bool {
        return false;
    }
    fn native_supported() -> bool {
        return true;
    }

    #[must_use] 
    #[inline(always)]
    unsafe fn apply(args: Self::Args) -> Self::ReturnType{
        let _ = Self::check;
        let (memory, data) = args;
        _mm256_storeu_pd(memory, data);
    }
} // end of struct storeu for template specialization of storeu for avx2 using f64.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::522
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_epi8(value)
    }
} // end of struct set1 for template specialization of set1 for avx2 using i8.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::522
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_epi16(value)
    }
} // end of struct set1 for template specialization of set1 for avx2 using i16.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::522
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_epi32(value)
    }
} // end of struct set1 for template specialization of set1 for avx2 using i32.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::526
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_epi8(value as i8)
    }
} // end of struct set1 for template specialization of set1 for avx2 using u8.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::526
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_epi16(value as i16)
    }
} // end of struct set1 for template specialization of set1 for avx2 using u16.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::526
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_epi32(value as i32)
    }
} // end of struct set1 for template specialization of set1 for avx2 using u32.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::534
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_epi64x(value as i64)
    }
} // end of struct set1 for template specialization of set1 for avx2 using i64.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<f32>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::538
     */

    type BaseType = f32;
    type TargetExtension = avx2<f32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_ps(value)
    }
} // end of struct set1 for template specialization of set1 for avx2 using f32.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, avx2<f64>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: avx2.
     *        Data Type: f64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::538
     */

    type BaseType = f64;
    type TargetExtension = avx2<f64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::BaseType);
    type ReturnType = Self::RegisterType;
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
        let (value) = args;
        _mm256_set1_pd(value)
    }
} // end of struct set1 for template specialization of set1 for avx2 using f64.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<u8>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1733
     */

    type BaseType = u8;
    type TargetExtension = avx2<u8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        (_mm256_extract_epi8::<Index>(data)) as u8
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using u8.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<u16>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1733
     */

    type BaseType = u16;
    type TargetExtension = avx2<u16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        (_mm256_extract_epi16::<Index>(data)) as u16
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using u16.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<i8>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i8
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1733
     */

    type BaseType = i8;
    type TargetExtension = avx2<i8>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        (_mm256_extract_epi8::<Index>(data)) as i8
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using i8.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<i16>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i16
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1733
     */

    type BaseType = i16;
    type TargetExtension = avx2<i16>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        (_mm256_extract_epi16::<Index>(data)) as i16
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using i16.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<u32>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1733
     */

    type BaseType = u32;
    type TargetExtension = avx2<u32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        (_mm256_extract_epi32::<Index>(data)) as u32
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using u32.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<u64>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: u64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1733
     */

    type BaseType = u64;
    type TargetExtension = avx2<u64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        (_mm256_extract_epi64::<Index>(data)) as u64
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using u64.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<i32>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i32
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1737
     */

    type BaseType = i32;
    type TargetExtension = avx2<i32>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        _mm256_extract_epi32::<Index>(data)
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using i32.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, avx2<i64>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: avx2.
     *        Data Type: i64
     *  Extension Flags: ['avx']
     *      Yaml Source: primitive_data/primitives/ls.yaml::1737
     */

    type BaseType = i64;
    type TargetExtension = avx2<i64>;
    type AdditionalParam = ();
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;
    type Args = (Self::RegisterType);
    type ReturnType = Self::BaseType;
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
        let (data) = args;
        _mm256_extract_epi64::<Index>(data)
    }
} // end of struct extract_value for template specialization of extract_value for avx2 using i64.


