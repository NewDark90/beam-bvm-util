
#[derive(Default)]
pub struct SizedResult<T> {
    size: u32,
    value: T
}

impl<T> SizedResult<T> {
    pub fn new(value: T, size: u32) -> SizedResult<T> {
        SizedResult::<T> {
            size,
            value
        }
    }

    pub fn size(&self) -> u32 { self.size }
    pub fn value(&self) -> &T { &self.value }
} 