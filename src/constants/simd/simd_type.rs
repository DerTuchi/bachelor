use std::{marker::PhantomData, usize};
use crate::constants::simd::TSLArithmetic;
use crate::constants::simd::simd_type_trait_bounds::*;
use crate::constants::utils::helper_traits::SelectOffsetBaseType;

pub struct Simd<BaseType, TargetExtension , const VectorSizeInBits : usize> 
where
    BaseType : TSLArithmetic,
    TargetExtension: TargetExtensionTypes<BaseType, VectorSizeInBits>
{
    pub _phantom_base: PhantomData<BaseType>,
    pub _phantom_ext: PhantomData<TargetExtension>,
}

impl<T: TSLArithmetic + SelectOffsetBaseType, 
U: TargetExtensionTypes<T, VectorSizeInBits> + TargetExtensionTypes<<T as SelectOffsetBaseType>::Output, VectorSizeInBits>,
const VectorSizeInBits: usize> VectorProcessingStyle for Simd<T,U, VectorSizeInBits> {
    type BaseType = T;
    type TargetExtension = U;

    type RegisterType = <U as TargetExtensionTypes<T, VectorSizeInBits>>::RegisterType;
    type MaskType = <U as TargetExtensionTypes<T, VectorSizeInBits>>::MaskType;
    type ImaskType = <U as TargetExtensionTypes<T, VectorSizeInBits>>::ImaskType;

    type OffsetBaseType = <T as SelectOffsetBaseType>::Output;
    type OffsetBaseRegisterType = <U as TargetExtensionTypes<Self::OffsetBaseType, VectorSizeInBits>>::RegisterType;

    // Idear: Outsourcing Transformation types to seperate Traits, to make it feasable.

    fn is_register_type_pointer(&self) -> bool{
        true    // TODO
    }

    fn vector_size_b(&self) -> usize {
        VectorSizeInBits
    }

    fn vector_size_B(&self) -> usize {
        self.vector_element_count() * std::mem::size_of::<T>()
    }

    fn vector_element_count(&self) -> usize {
        Self::vector_size_b(&self) / (std::mem::size_of::<T>() * 8)
    }

    fn vector_alignment(&self) -> usize {
        if Self::vector_size_B(&self) > 32 {
            64
        } else {
            Self::vector_size_B(&self)
        }
    }

    fn vector_mask_ratio(&self) -> usize {
        (std::mem::size_of::<Self::MaskType>() * 8) / Self::vector_element_count(&self)
    }

    fn mask_shift(&self) -> usize {
        Self::vector_element_count(&self)
    }
}

