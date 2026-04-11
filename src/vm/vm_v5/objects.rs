use crate::vm::vm_v5::vm_state::VmResult;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PassBy {
    Value,    // Does a shallow copy
    Reference // Returns the pointer
}

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    BOOLEAN(bool),
    UBYTE(u8),
    SBYTE(i8),

    UINT(u32),
    SINT(i32),

    ULONG(u64),
    SLONG(i64),

    FLOAT(f32),
    DOUBLE(f64),

    // TODO: Fuck. Meta Objects
    //OBJECT_REFERNCE { types: Vec<&Object> },
    //OBJECT_VALUE { types: Vec<&Object> },

    // UTF16 based strings
    STRING(Vec<u16>),
}

impl Object {
    fn cast_to_bool(&self) -> VmResult<bool>{
        return match self {
            Object::BOOLEAN(x) => Ok(*x),
            Object::UBYTE(x) => Ok(*x == 0),
            Object::SBYTE(x) => Ok(*x == 0),
            Object::UINT(x) => Ok(*x == 0),
            Object::SINT(x) => Ok(*x == 0),
            Object::ULONG(x) => Ok(*x == 0),
            Object::SLONG(x) => Ok(*x == 0),
            Object::FLOAT(x) => Ok(*x == 0.0),
            Object::DOUBLE(x) => Ok(*x == 0.0),
            Object::STRING(x) => Ok(x.len() != 0),
            _ => Err((0, "Object::castToBool"))
        };
    }

    fn cast_to_ubyte(&self) -> VmResult<u8>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1} else {0}),
            Object::UBYTE(x) => Ok(*x),
            Object::SBYTE(x) => Ok(*x as u8),
            Object::UINT(x) => Ok(*x as u8),
            Object::SINT(x) => Ok(*x as u8),
            Object::ULONG(x) => Ok(*x as u8),
            Object::SLONG(x) => Ok(*x as u8),
            Object::FLOAT(x) => Ok(x.round() as u8),
            Object::DOUBLE(x) => Ok(x.round() as u8),
            _ => Err((0, "Object::castToUByte"))
        };
    }

    fn cast_to_sbyte(&self) -> VmResult<i8>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1} else {0}),
            Object::UBYTE(x) => Ok(*x as i8),
            Object::SBYTE(x) => Ok(*x),
            Object::UINT(x) => Ok(*x as i8),
            Object::SINT(x) => Ok(*x as i8),
            Object::ULONG(x) => Ok(*x as i8),
            Object::SLONG(x) => Ok(*x as i8),
            Object::FLOAT(x) => Ok(x.round() as i8),
            Object::DOUBLE(x) => Ok(x.round() as i8),
            _ => Err((0, "Object::castToUByte"))
        };
    }

    fn cast_to_uint(&self) -> VmResult<u32>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1} else {0}),
            Object::UBYTE(x) => Ok(*x as u32),
            Object::SBYTE(x) => Ok(*x as u32),
            Object::UINT(x) => Ok(*x),
            Object::SINT(x) => Ok(*x as u32),
            Object::ULONG(x) => Ok(*x as u32),
            Object::SLONG(x) => Ok(*x as u32),
            Object::FLOAT(x) => Ok(x.round() as u32),
            Object::DOUBLE(x) => Ok(x.round() as u32),
            _ => Err((0, "Object::castToUByte"))
        };
    }

    fn cast_to_sint(&self) -> VmResult<i32>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1} else {0}),
            Object::UBYTE(x) => Ok(*x as i32),
            Object::SBYTE(x) => Ok(*x as i32),
            Object::UINT(x) => Ok(*x as i32),
            Object::SINT(x) => Ok(*x),
            Object::ULONG(x) => Ok(*x as i32),
            Object::SLONG(x) => Ok(*x as i32),
            Object::FLOAT(x) => Ok(x.round() as i32),
            Object::DOUBLE(x) => Ok(x.round() as i32),
            _ => Err((0, "Object::castToUByte"))
        };
    }

    fn cast_to_ulong(&self) -> VmResult<u64>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1} else {0}),
            Object::UBYTE(x) => Ok(*x as u64),
            Object::SBYTE(x) => Ok(*x as u64),
            Object::UINT(x) => Ok(*x as u64),
            Object::SINT(x) => Ok(*x as u64),
            Object::ULONG(x) => Ok(*x),
            Object::SLONG(x) => Ok(*x as u64),
            Object::FLOAT(x) => Ok(x.round() as u64),
            Object::DOUBLE(x) => Ok(x.round() as u64),
            _ => Err((0, "Object::castToUByte"))
        };
    }

    fn cast_to_slong(&self) -> VmResult<i64>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1} else {0}),
            Object::UBYTE(x) => Ok(*x as i64),
            Object::SBYTE(x) => Ok(*x as i64),
            Object::UINT(x) => Ok(*x as i64),
            Object::SINT(x) => Ok(*x as i64),
            Object::ULONG(x) => Ok(*x as i64),
            Object::SLONG(x) => Ok(*x),
            Object::FLOAT(x) => Ok(x.round() as i64),
            Object::DOUBLE(x) => Ok(x.round() as i64),
            _ => Err((0, "Object::castToUByte"))
        };
    }

    fn cast_to_float(&self) -> VmResult<f32>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1.0} else {0.0}),
            Object::UBYTE(x) => Ok(*x as f32),
            Object::SBYTE(x) => Ok(*x as f32),
            Object::UINT(x) => Ok(*x as f32),
            Object::SINT(x) => Ok(*x as f32),
            Object::ULONG(x) => Ok(*x as f32),
            Object::SLONG(x) => Ok(*x as f32),
            Object::FLOAT(x) => Ok(*x),
            Object::DOUBLE(x) => Ok(*x as f32),
            _ => Err((0, "Object::castToUByte"))
        };
    }


    fn cast_to_double(&self) -> VmResult<f64>{
        return match self {
            Object::BOOLEAN(x) => Ok(if *x {1.0} else {0.0}),
            Object::UBYTE(x) => Ok(*x as f64),
            Object::SBYTE(x) => Ok(*x as f64),
            Object::UINT(x) => Ok(*x as f64),
            Object::SINT(x) => Ok(*x as f64),
            Object::ULONG(x) => Ok(*x as f64),
            Object::SLONG(x) => Ok(*x as f64),
            Object::FLOAT(x) => Ok(*x as f64),
            Object::DOUBLE(x) => Ok(*x),
            _ => Err((0, "Object::castToUByte"))
        };
    }
}