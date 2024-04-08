use super::super::TSLArithmetic;
use super::super::simd_traits::SimdPrimitiveImpl;

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

// pub fn is_ambiguos<T: SimdPrimitiveImpl, U: SimdPrimitiveImpl>() -> bool {  // Ist nur Runtime. Alternative: Nach Generator outsorucen.
//     (std::any::TypeId::of::<T::ReturnType>() == std::any::TypeId::of::<U::ReturnType>()) & (std::any::TypeId::of::<T::Args>() == std::any::TypeId::of::<U::Args>())
// }