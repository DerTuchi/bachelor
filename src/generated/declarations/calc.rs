use std::marker::PhantomData;
use crate::constants::TSLArithmetic;
use crate::constants::simd_traits::{SimdPrimitiveImpl, TargetExtension};

pub struct Add<T: TargetExtension>(pub PhantomData<T>);
impl<T: TargetExtension> Add<T> {
    // Constructor function that hides the PhantomData from the user.
    pub fn new() -> Self {
        Add(PhantomData)
    }
}