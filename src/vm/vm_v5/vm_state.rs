
pub type VmResult<X> = Result<X, (u32, &'static str)>;
pub type VmEmpty = VmResult<()>;