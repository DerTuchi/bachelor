
use crate::constants::simd::TSLArithmetic;

pub trait TargetExtensionTypes<BaseType: TSLArithmetic, const VectorSizeInBits : usize>{ // VectorSizeInBits needs to be speciefied every time (ugly) TODO
    const DEFAULT_SIZE_IN_BITS : usize = VectorSizeInBits;
    type RegisterType;
    type MaskType;
    type ImaskType;
}

pub trait VectorProcessingStyle{
    type BaseType : TSLArithmetic;
    type TargetExtension;
    type RegisterType;
    type MaskType;
    type ImaskType;
    type OffsetBaseType;
    type OffsetBaseRegisterType;

    fn is_register_type_pointer(&self) -> bool;
    fn vector_size_b(&self) -> usize;
    fn vector_size_B(&self) -> usize;
    fn vector_element_count(&self) -> usize;
    fn vector_alignment(&self) -> usize;
    fn vector_mask_ratio(&self) -> usize;
    fn mask_shift(&self) -> usize;
}

pub trait ImplementationDegreeOfFreedom{    //TODO
    fn is_native(&self) -> bool;
}
struct Native;
struct Workaround;
impl ImplementationDegreeOfFreedom for Native{
    fn is_native(&self) -> bool{
        true
    }
}
impl ImplementationDegreeOfFreedom for Workaround{
    fn is_native(&self) -> bool{
        false
    }
}