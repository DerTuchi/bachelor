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
 * \file /home/dertuchi/TSL/generated_tsl/generator_output/include/generated/declarations/ls.rs
 * \date 2024-05-02
 * \brief Load/Store primitives
 * \note
 * Git-Local Url : /home/dertuchi/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.8 (7302664ad7b976795a660a3a21d6f31554148172)
 *
 */
use std::marker::PhantomData;
use crate::static_files::TSLArithmetic;
use crate::static_files::simd_traits::*;


pub struct loadu<const Idof: bool, T: TargetExtension>(pub PhantomData<T>);

pub struct storeu<const Idof: bool, T: TargetExtension>(pub PhantomData<T>);

pub struct set1<const Idof: bool, T: TargetExtension>(pub PhantomData<T>);

pub struct extract_value<const Idof: bool, const Index : i32, T: TargetExtension>(pub PhantomData<T>);


