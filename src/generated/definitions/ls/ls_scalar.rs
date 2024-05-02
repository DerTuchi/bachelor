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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/definitions/ls/ls_scalar.rs
 * \date 2024-05-02
 * \brief Load/Store primitives
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8 (7302664ad7b976795a660a3a21d6f31554148172)
 *
 */
use crate::generated::declarations::ls::*;
use crate::generated::extensions::scalar::*;
use crate::static_files::simd_traits::*;



impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<u8>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = u8;
    type TargetExtension = scalar<u8>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using u8.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<u16>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = u16;
    type TargetExtension = scalar<u16>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using u16.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<u32>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = u32;
    type TargetExtension = scalar<u32>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using u32.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<u64>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = u64;
    type TargetExtension = scalar<u64>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using u64.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<i8>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = i8;
    type TargetExtension = scalar<i8>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using i8.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<i16>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = i16;
    type TargetExtension = scalar<i16>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using i16.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<i32>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = i32;
    type TargetExtension = scalar<i32>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using i32.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<i64>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = i64;
    type TargetExtension = scalar<i64>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using i64.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<f32>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = f32;
    type TargetExtension = scalar<f32>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using f32.


impl< const Idof: bool > SimdPrimitiveImpl for loadu<Idof, scalar<f64>> {
    /**
     * @brief: Template specialization of implementation for "loadu" (primitive loadu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::128
     */

    type BaseType = f64;
    type TargetExtension = scalar<f64>;
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
        *memory
    }
} // end of struct loadu for template specialization of loadu for scalar using f64.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<u8>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = u8;
    type TargetExtension = scalar<u8>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using u8.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<u16>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = u16;
    type TargetExtension = scalar<u16>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using u16.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<u32>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = u32;
    type TargetExtension = scalar<u32>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using u32.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<u64>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = u64;
    type TargetExtension = scalar<u64>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using u64.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<i8>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = i8;
    type TargetExtension = scalar<i8>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using i8.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<i16>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = i16;
    type TargetExtension = scalar<i16>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using i16.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<i32>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = i32;
    type TargetExtension = scalar<i32>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using i32.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<i64>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = i64;
    type TargetExtension = scalar<i64>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using i64.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<f32>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = f32;
    type TargetExtension = scalar<f32>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using f32.


impl< const Idof: bool > SimdPrimitiveImpl for storeu<Idof, scalar<f64>> {
    /**
     * @brief: Template specialization of implementation for "storeu" (primitive storeu).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::341
     */

    type BaseType = f64;
    type TargetExtension = scalar<f64>;
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
        *memory = data;
    }
} // end of struct storeu for template specialization of storeu for scalar using f64.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<u8>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = u8;
    type TargetExtension = scalar<u8>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using u8.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<u16>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = u16;
    type TargetExtension = scalar<u16>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using u16.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<u32>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = u32;
    type TargetExtension = scalar<u32>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using u32.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<u64>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = u64;
    type TargetExtension = scalar<u64>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using u64.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<i8>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = i8;
    type TargetExtension = scalar<i8>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using i8.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<i16>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = i16;
    type TargetExtension = scalar<i16>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using i16.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<i32>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = i32;
    type TargetExtension = scalar<i32>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using i32.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<i64>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = i64;
    type TargetExtension = scalar<i64>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using i64.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<f32>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = f32;
    type TargetExtension = scalar<f32>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using f32.


impl< const Idof: bool > SimdPrimitiveImpl for set1<Idof, scalar<f64>> {
    /**
     * @brief: Template specialization of implementation for "set1" (primitive set1).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::568
     */

    type BaseType = f64;
    type TargetExtension = scalar<f64>;
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
        value
    }
} // end of struct set1 for template specialization of set1 for scalar using f64.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<u8>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = u8;
    type TargetExtension = scalar<u8>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using u8.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<i8>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i8
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = i8;
    type TargetExtension = scalar<i8>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using i8.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<u16>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = u16;
    type TargetExtension = scalar<u16>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using u16.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<i16>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i16
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = i16;
    type TargetExtension = scalar<i16>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using i16.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<u32>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = u32;
    type TargetExtension = scalar<u32>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using u32.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<i32>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = i32;
    type TargetExtension = scalar<i32>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using i32.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<u64>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: u64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = u64;
    type TargetExtension = scalar<u64>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using u64.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<i64>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: i64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = i64;
    type TargetExtension = scalar<i64>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using i64.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<f32>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f32
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = f32;
    type TargetExtension = scalar<f32>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using f32.


impl< const Idof: bool , const Index : i32> SimdPrimitiveImpl for extract_value<Idof, Index, scalar<f64>> {
    /**
     * @brief: Template specialization of implementation for "extract_value" (primitive extract_value).
     * @details:
     * Target Extension: scalar.
     *        Data Type: f64
     *  Extension Flags: []
     *      Yaml Source: primitive_data/primitives/ls.yaml::1751
     */

    type BaseType = f64;
    type TargetExtension = scalar<f64>;
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
        return data;
    }
} // end of struct extract_value for template specialization of extract_value for scalar using f64.


