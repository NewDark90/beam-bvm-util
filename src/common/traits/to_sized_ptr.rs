use core::{mem::size_of_val, ptr::null};

use crate::common::types::{sized_ptr::SizedPointer};

pub trait ToSizedPtr<T> {
    fn to_sized_or_null_ptr(self: &Self) -> SizedPointer<*const T>;
}

pub trait ToSizedPtrMut<T> {
    fn to_sized_or_null_ptr_mut(self: &Self) -> SizedPointer<*mut T>;
}

impl<T> ToSizedPtr<T> for Option<&T> {
    fn to_sized_or_null_ptr(self: &Self) -> SizedPointer<*const T> {
        match self {
            Some(v) => {
                SizedPointer::<*const T>::new(
                    *v as *const T,
                    size_of_val::<T>(v) as u32
                )
            }, 
            None => {
                SizedPointer::<*const T>::new(
                    null::<T>(),
                    0
                )
            } 
        }
    }
}

impl<T> ToSizedPtrMut<T> for Option<&mut T> {
    fn to_sized_or_null_ptr_mut(self: &Self) -> SizedPointer<*mut T> {
        match self {
            Some(v) => {
                SizedPointer::<*mut T>::new(
                    *v as *const T as *mut T,
                    size_of_val::<T>(v) as u32
                )
            }, 
            None => {
                SizedPointer::<*mut T>::new(
                    null::<T>() as *mut T,
                    0
                )
            } 
        }
    }
}