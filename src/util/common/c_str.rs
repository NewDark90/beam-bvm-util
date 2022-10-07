#[macro_export]
macro_rules! c_str {
    ($data:literal) => {
        &concat!($data, "\0")
    }
}