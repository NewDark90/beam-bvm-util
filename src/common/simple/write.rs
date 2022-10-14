use core::mem::size_of_val;

use crate::common::safe;

pub enum Stream {
    Out = 0,
    Error = 1,
}

/// https://github.com/BeamMW/shader-sdk/wiki/Write
pub fn write<T>(data: &T, stream: Stream) {
    safe::write(data, size_of_val::<T>(data) as u32, {
        match stream {
            Stream::Out => 0,
            Stream::Error => 1,
        }
    })
}
