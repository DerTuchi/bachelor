use super::TSLArithmetic;

pub trait SimdPrimitiveImpl{
    type Args;
    type ReturnType;
    type BaseType;
    type TargetExtension;
    type RegisterType;
    type ImaskType;
    type MaskType;
    type OffsetBaseType;
    type OffsetBaseRegisterType;

    fn parameters_queryable(&self) -> bool;
    fn has_return_value(&self) -> bool;
    fn native_supported(&self) -> bool;
    fn apply(self, args: Self::Args) -> Self::ReturnType; // No "&self" so that the instance destroys itself after calling apply.
}

pub trait TargetExtension{
    const DEFAULT_SIZE_IN_BITS : usize;
    type BaseType: TSLArithmetic;
    type RegisterType;
    type MaskType;
    type ImaskType;
    type OffsetBaseType: TSLArithmetic;
    type OffsetBaseRegisterType;
}