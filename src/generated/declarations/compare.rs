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
 * \file /home/dertuchi/work/TSL/generated_tsl/generator_output/include/generated/declarations/compare.rs
 * \date 2024-04-28
 * \brief Compare primitives.
 * \note
 * Git-Local Url : /home/dertuchi/work/TSL
 * Git-Remote Url: https://github.com/DerTuchi/TSL.git
 * Git-Branch    : main
 * Git-Commit    : v0.0.6 (7e77c245b3b376caa65a2219fb685d487b96ec1a)
 *
 */
use std::marker::PhantomData;
use crate::static_files::TSLArithmetic;
use crate::static_files::simd_traits::*;


pub struct less_than<const Idof: bool, T: TargetExtension>(pub PhantomData<T>);

