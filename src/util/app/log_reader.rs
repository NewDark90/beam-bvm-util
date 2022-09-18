
use core::mem::size_of_val;
use beam_bvm_interface::root::HeightPos;
use crate::safe::app::log::*;


#[repr(C)]
pub struct LogReader {
    handle: u32,
    pub pos: HeightPos,
}

impl LogReader {
    pub fn new<K1, K2>(
        key1: &K1,
        key2: &K2,
        pos_min: *const HeightPos,
        pos_max: *const HeightPos,
    ) -> LogReader {
        LogReader {
            handle: logs_enum(
                key1,
                size_of_val(key1) as u32,
                key2,
                size_of_val(key2) as u32,
                pos_min,
                pos_max,
            ),
            pos: HeightPos {
                m_Height: 0,
                m_Pos: 0
            },
        }
    }

    pub fn move_next<K, V>(
        &mut self,
        key: *mut K,
        key_size: &mut u32,
        val: *mut V,
        val_size: &mut u32,
        repeat: u8,
    ) -> bool {
        logs_move_next(
            self.handle,
            key,
            key_size,
            val,
            val_size,
            &mut self.pos,
            repeat,
        ) != 0
    }

    pub fn move_next_t<K, V>(&mut self, key: &mut K, value: &mut V) -> bool {
        loop {
            let mut key_size: u32 = size_of_val(key) as u32;
            let mut value_size: u32 = size_of_val(value) as u32;
            if !self.move_next(key, &mut key_size, value, &mut value_size, 0) {
                return false;
            }
            if size_of_val(key) as u32 == key_size && size_of_val(value) as u32 == value_size {
                break;
            }
        }
        true
    }
}

impl Drop for LogReader {
    fn drop(&mut self) {
        logs_close(self.handle);
    }
}