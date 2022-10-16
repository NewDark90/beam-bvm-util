use core::mem::size_of_val;

use crate::app::safe;


/// https://github.com/BeamMW/shader-sdk/wiki/Vars_Enum
pub fn vars_enum<U, V>(key0: &U, key1: &V) -> u32 {
    safe::vars_enum(
        key0,
        size_of_val::<U>(key0) as u32,
        key1,
        size_of_val::<V>(key1) as u32
    )
}