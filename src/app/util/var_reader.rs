use crate::app::safe::*;
use core::mem::{size_of_val};

#[repr(C)]
pub struct VarReaderEx<const FLEXIBLE: bool> {
    pub handle: u32,
}

impl<const FLEXIBLE: bool> VarReaderEx<FLEXIBLE> {
    fn enum_internal<K1, K2>(&mut self, key1: &K1, key1_size: u32, key2: &K2, key2_size: u32) {
        self.handle = vars_enum(key1, key1_size, key2, key2_size);
    }

    fn close_internal(&self) {
        if FLEXIBLE {
            if self.handle == 0 {
                return;
            }
        }
        vars_close(self.handle);
    }

    pub fn new<K1, K2>(key1: &K1, key2: &K2) -> VarReaderEx<FLEXIBLE> {
        let mut reader: VarReaderEx<FLEXIBLE> = Default::default();
        reader.enum_internal(
            key1,
            size_of_val(key1) as u32,
            key2,
            size_of_val(key2) as u32,
        );
        reader
    }

    pub fn move_next<K, V>(
        &self,
        key: &mut K,
        key_size: &mut u32,
        val: &mut V,
        val_size: &mut u32,
        repeat: u8,
    ) -> bool {
        vars_move_next(self.handle, key, key_size, val, val_size, repeat) != 0
    }

    pub fn move_next_t<K, V>(&self, key: &mut K, value: &mut V) -> bool {
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

    pub fn move_all_t<K: Sized, V: Sized, TOnFind: Fn(&K, &V) -> ()>(&self, on_find: TOnFind) -> u32 {
        let mut found: u32 = 0;
        let mut key_holder: K = unsafe { core::mem::zeroed() };
        let mut value_holder: V = unsafe { core::mem::zeroed() };

        while self.move_next_t(&mut key_holder, &mut value_holder) {
            found += 1;
            on_find(&key_holder, &value_holder);
        }
        found
    }

    pub fn read_single<K, V>(key: &K, value: &mut V) -> bool {
        let mut r: VarReader = Default::default();
        let mut key_size: u32 = size_of_val(key) as u32;
        r.enum_internal(key, key_size, key, key_size);

        let mut val_size: u32 = size_of_val(value) as u32;
        key_size = 0;
        r.move_next(&mut 0, &mut key_size, value, &mut val_size, 0)
            && size_of_val(value) as u32 == val_size
    }

    pub fn r#enum<K, V>(&mut self, key: &K, value: &V) {
        self.close_internal();
        let key_size: u32 = size_of_val(key) as u32;
        let value_size: u32 = size_of_val(value) as u32;
        self.enum_internal(key, key_size, value, value_size);
    }
}

impl<const FLEXIBLE: bool> Drop for VarReaderEx<FLEXIBLE> {
    fn drop(&mut self) {
        self.close_internal()
    }
}

impl<const FLEXIBLE: bool> Default for VarReaderEx<FLEXIBLE> {
    fn default() -> Self {
        VarReaderEx {
            handle: Default::default(),
        }
    }
}

pub type VarReader = VarReaderEx<false>;
