use crate::generated::declarations::calc::*;
use crate::generated::extensions::sse::Sse;

use crate::constants::simd_traits::*;

use std::arch::x86_64::*;


impl SimdPrimitiveImpl for Add<Sse<u8>>{
    type BaseType = u8;
    type TargetExtension = Sse<u8>;
    type RegisterType = <Self::TargetExtension as TargetExtension>::RegisterType;
    type ImaskType = <Self::TargetExtension as TargetExtension>::ImaskType;
    type MaskType = <Self::TargetExtension as TargetExtension>::MaskType;
    type OffsetBaseType = <Self::TargetExtension as TargetExtension>::OffsetBaseType;
    type OffsetBaseRegisterType = <Self::TargetExtension as TargetExtension>::OffsetBaseRegisterType;

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
    fn apply(self, args: Self::Args) -> Self::ReturnType{
        let vec = Self::TargetExtension::new();    //If not used, gets detected and optimized by compiler, so no problem.
        let (vec_a, vec_b) = args;
        return unsafe{_mm_adds_epu8(vec_a, vec_b)};
    }
}