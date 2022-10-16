#[derive(Default)]
pub struct SizedPointer<T> {
    size: u32,
    ptr: T
}

impl<T> SizedPointer<*mut T> {
    pub fn new(ptr: *mut T, size: u32) -> SizedPointer<*mut T> {
        SizedPointer::<*mut T> {
            ptr,
            size
        }
    }

    pub fn size(&self) -> u32 { self.size }
    pub fn ptr(&self) -> *mut T { self.ptr }
}

impl<T> SizedPointer<*const T> {
    pub fn new(ptr: *const T, size: u32) -> SizedPointer<*const T> {
        SizedPointer::<*const T> {
            ptr,
            size
        }
    }

    pub fn size(&self) -> u32 { self.size }
    pub fn ptr(&self) -> *const T { self.ptr }
}