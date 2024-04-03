pub trait SimdPrimitiveImpl<>{
    type Args;
    type ReturnType;
    type BaseType;
    type TargetExtension;
    type RegisterType;
    type Vec;

    fn parameters_queryable(&self) -> bool;
    fn has_return_value(&self) -> bool;
    fn native_supported(&self) -> bool;
    fn apply(&self, args: Self::Args) -> Self::ReturnType;
}