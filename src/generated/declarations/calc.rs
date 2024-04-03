use std::marker::PhantomData;
use crate::constants::simd::TSLArithmetic;
use crate::constants::simd::simd_type_trait_bounds::TargetExtensionTypes;

pub struct add<T: TSLArithmetic , U: TargetExtensionTypes<T, VectorSizeInBits>, const VectorSizeInBits: usize>(pub PhantomData<T>, pub PhantomData<U>);