pub type VmError = (u32, &'static str);
pub type VmResult<X> = Result<X, VmError>;
pub type VmEmpty = VmResult<()>;
