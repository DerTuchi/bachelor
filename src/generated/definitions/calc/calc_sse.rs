use crate::generated::declarations::calc::*;
use crate::generated::extensions::sse::Sse;

use crate::constants::simd::simd_type_trait_bounds::VectorProcessingStyle;
use crate::constants::simd::simd_type::Simd;
use crate::constants::simd::simd_primitive_trait_bounds::SimdPrimitiveImpl;

use std::arch::x86_64::*;

impl SimdPrimitiveImpl for add<u8, Sse, 128>{
    type Vec = Simd<u8, Sse, 128>;
    type RegisterType = <Self::Vec as VectorProcessingStyle>::RegisterType;
    type BaseType = u8;
    type TargetExtension = Sse;
    type Args = (Self::RegisterType, Self::RegisterType);
    type ReturnType = Self::RegisterType;

    fn parameters_queryable(&self) -> bool{
        return true;
    }
    fn has_return_value(&self) -> bool {
        return true;
    }
    fn native_supported(&self) -> bool {
        return true;
    }

    #[must_use]
    #[inline(always)]
    fn apply(&self, args: Self::Args) -> Self::ReturnType{
        let (vec_a, vec_b) = args;
        return unsafe{_mm_adds_epu8(vec_a, vec_b)};
    }
}