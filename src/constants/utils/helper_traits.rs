use crate::{constants::simd::{simd_type_trait_bounds::TargetExtensionTypes, TSLArithmetic}, generated::extensions::sse::Sse};

// Trait to select OffsetBaseType
pub trait SelectOffsetBaseType {
    type Output : TSLArithmetic;
}
macro_rules! impl_signed_to_unsigned {
    ($($signed:ty => $unsigned:ty),*; $($selfmap:ty),*) => {
        // Map signed types to their unsigned equivalents
        $(impl SelectOffsetBaseType for $signed where $signed : TSLArithmetic {
            type Output = $unsigned;
        })*

        // Map unsigned types to themselves
        $(impl SelectOffsetBaseType for $selfmap where $selfmap : TSLArithmetic{
            type Output = $selfmap;
        })*
    };
}
impl_signed_to_unsigned!{
    i8 => u8,
    i16 => u16,
    i32 => u32,
    i64 => u64,
    i128 => u128,
    isize => usize,
    f32 => u32,
    f64 => u64;
    u8, u16, u32, u64, u128, usize
}

// Trait to get transformed Register_Type
pub trait TransformType <OtherType: TSLArithmetic, const VectorSizeInBits : usize>{
    type Output;
}