
#[derive(Default)]
pub struct SizedResult<T> {
    pub size: u32,
    pub value: T
}