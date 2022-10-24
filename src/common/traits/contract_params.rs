pub trait ContractParams {
    fn method_id() -> u32;
    fn as_method_id(&self) -> u32 {
        Self::method_id()
    }
}