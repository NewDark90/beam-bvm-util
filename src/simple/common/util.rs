use core::mem::size_of_val;

use crate::safe::common::util::write as write_safe;

enum Stream {
    Out = 0,
    Error = 1,
}

/// https://github.com/BeamMW/shader-sdk/wiki/Write
pub fn write<T>(data: &T, stream: Stream) {
    unsafe {
        write_safe(data, size_of_val::<T>(data) as u32, {
            match stream {
                Out => 0,
                Error => 1,
            }
        })
    }
}
