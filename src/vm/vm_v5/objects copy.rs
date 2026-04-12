use crate::vm::vm_v5::vm_state::{VmRef, VmResult};

// All the types with data
#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Boolean(bool),
    UByte(u8),
    SByte(i8),

    UInt(u32),
    SInt(i32),

    ULong(u64),
    SLong(i64),

    Float(f32),
    Double(f64),

    Array { data: Vec<Option<ReferentialObject>>, typ: ObjectType },

    OReference { data: Vec<ReferentialObject>, id: u32, vm: VmRef },
    OValue { data: Vec<ReferentialObject>, id: u32, vm: VmRef },

    String(String),
}

// Very basic enum of all the possible types
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ObjectType {
    Boolean,
    UByte,
    SByte,

    UInt,
    SInt,

    ULong,
    SLong,

    Float,
    Double,

    Array,

    OReference,
    OValue,

    String,
}

// Basic layout for how Objects will reference each other
#[derive(PartialEq, Debug, Clone)]
pub struct ReferentialObject {
    pub typ: ObjectType,  // Type that this Object is
    pub index: u32,       // The "pointer"/index it is in, in the VM
    pub vm: VmRef         // The VM that holds this Object
}

impl ObjectType {
    // If `None` is returned, that means the Object Type's size can not be known
    pub fn get_size(&self) -> Option<u32> {
        return match self {
            ObjectType::Boolean => Some(1),
            ObjectType::UByte | ObjectType::SByte => Some(1),
            ObjectType::UInt  | ObjectType::SInt  | ObjectType::Float => Some(4),
            ObjectType::ULong | ObjectType::SLong | ObjectType::Double => Some(8),
            _ => None
        };
    }
}

impl Object {
    pub fn create_basic_number(typ: ObjectType) -> VmResult<Object> {
        return match typ {
            ObjectType::Boolean => Ok(Object::Boolean(false)),
            ObjectType::UByte => Ok(Object::UByte(0)),
            ObjectType::SByte => Ok(Object::SByte(0)),
            ObjectType::UInt => Ok(Object::UInt(0)),
            ObjectType::SInt => Ok(Object::SInt(0)),
            ObjectType::ULong => Ok(Object::ULong(0)),
            ObjectType::SLong => Ok(Object::SLong(0)),
            ObjectType::Float => Ok(Object::Float(0.0)),
            ObjectType::Double => Ok(Object::Double(0.0)),
            _ => Err((0, "Object::create_basic_object"))
        };
    }

    pub fn to_type(&self) -> ObjectType {
        return match self {
            Object::Boolean(_) => ObjectType::Boolean,
            Object::UByte(_) => ObjectType::UByte,
            Object::SByte(_) => ObjectType::SByte,
            Object::UInt(_) => ObjectType::UInt,
            Object::SInt(_) => ObjectType::SInt,
            Object::ULong(_) => ObjectType::ULong,
            Object::SLong(_) => ObjectType::SLong,
            Object::Float(_) => ObjectType::Float,
            Object::Double(_) => ObjectType::Double,
            Object::Array { data: _, typ: _ } => ObjectType::Array,
            Object::OReference { data: _, id: _, vm: _ } => ObjectType::OReference,
            Object::OValue { data: _, id: _, vm: _ } => ObjectType::OValue,
            Object::String(_) => ObjectType::String,
        }
    }

    // Returns the size of the object in bytes
    pub fn get_size(&self) -> u32 {
        return match self {
            Object::Boolean(_) => 1,
            Object::UByte(_) | Object::SByte(_) => 1,
            Object::UInt(_)  | Object::SInt(_)  | Object::Float(_) => 4,
            Object::ULong(_) | Object::SLong(_) | Object::Double(_) => 8,

            Object::Array { data: data, typ: typ } =>
                    data.len() as u32, //TODO: Uhh... get the size of the type?
            Object::OReference { data: data, id: _, vm: _ } =>
                data.iter().fold(0, |val, var| -> u32 {val + var.vm.borrow().get_object_size(var).unwrap_or(0)}),
            Object::OValue { data: data, id: _, vm: _ } => 
                data.iter().fold(0, |val, var| -> u32 {val + var.vm.borrow().get_object_size(var).unwrap_or(0)}),
            Object::String(x) => (x.len()*8) as u32,
        }
        
    }

    pub fn cast_to_bool(&self) -> VmResult<bool>{
        return match self {
            Object::Boolean(x) => Ok(*x),
            Object::UByte(x) => Ok(*x == 0),
            Object::SByte(x) => Ok(*x == 0),
            Object::UInt(x) => Ok(*x == 0),
            Object::SInt(x) => Ok(*x == 0),
            Object::ULong(x) => Ok(*x == 0),
            Object::SLong(x) => Ok(*x == 0),
            Object::Float(x) => Ok(*x == 0.0),
            Object::Double(x) => Ok(*x == 0.0),
            Object::String(x) => Ok(x.len() != 0),
            Object::Array {data: x, typ: _} => Ok(x.len() != 0),
            _ => Err((0, "Object::cast_to_bool"))
        };
    }

    pub fn cast_to_ubyte(&self) -> VmResult<u8>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1} else {0}),
            Object::UByte(x) => Ok(*x),
            Object::SByte(x) => Ok(*x as u8),
            Object::UInt(x) => Ok(*x as u8),
            Object::SInt(x) => Ok(*x as u8),
            Object::ULong(x) => Ok(*x as u8),
            Object::SLong(x) => Ok(*x as u8),
            Object::Float(x) => Ok(x.round() as u8),
            Object::Double(x) => Ok(x.round() as u8),
            _ => Err((0, "Object::cast_to_ubyte"))
        };
    }

    pub fn cast_to_sbyte(&self) -> VmResult<i8>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1} else {0}),
            Object::UByte(x) => Ok(*x as i8),
            Object::SByte(x) => Ok(*x),
            Object::UInt(x) => Ok(*x as i8),
            Object::SInt(x) => Ok(*x as i8),
            Object::ULong(x) => Ok(*x as i8),
            Object::SLong(x) => Ok(*x as i8),
            Object::Float(x) => Ok(x.round() as i8),
            Object::Double(x) => Ok(x.round() as i8),
            _ => Err((0, "Object::cast_to_sbyte"))
        };
    }

    pub fn cast_to_uint(&self) -> VmResult<u32>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1} else {0}),
            Object::UByte(x) => Ok(*x as u32),
            Object::SByte(x) => Ok(*x as u32),
            Object::UInt(x) => Ok(*x),
            Object::SInt(x) => Ok(*x as u32),
            Object::ULong(x) => Ok(*x as u32),
            Object::SLong(x) => Ok(*x as u32),
            Object::Float(x) => Ok(x.round() as u32),
            Object::Double(x) => Ok(x.round() as u32),
            _ => Err((0, "Object::cast_to_uint"))
        };
    }

    pub fn cast_to_sint(&self) -> VmResult<i32>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1} else {0}),
            Object::UByte(x) => Ok(*x as i32),
            Object::SByte(x) => Ok(*x as i32),
            Object::UInt(x) => Ok(*x as i32),
            Object::SInt(x) => Ok(*x),
            Object::ULong(x) => Ok(*x as i32),
            Object::SLong(x) => Ok(*x as i32),
            Object::Float(x) => Ok(x.round() as i32),
            Object::Double(x) => Ok(x.round() as i32),
            _ => Err((0, "Object::cast_to_sint"))
        };
    }

    pub fn cast_to_ulong(&self) -> VmResult<u64>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1} else {0}),
            Object::UByte(x) => Ok(*x as u64),
            Object::SByte(x) => Ok(*x as u64),
            Object::UInt(x) => Ok(*x as u64),
            Object::SInt(x) => Ok(*x as u64),
            Object::ULong(x) => Ok(*x),
            Object::SLong(x) => Ok(*x as u64),
            Object::Float(x) => Ok(x.round() as u64),
            Object::Double(x) => Ok(x.round() as u64),
            _ => Err((0, "Object::cast_to_ulong"))
        };
    }

    pub fn cast_to_slong(&self) -> VmResult<i64>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1} else {0}),
            Object::UByte(x) => Ok(*x as i64),
            Object::SByte(x) => Ok(*x as i64),
            Object::UInt(x) => Ok(*x as i64),
            Object::SInt(x) => Ok(*x as i64),
            Object::ULong(x) => Ok(*x as i64),
            Object::SLong(x) => Ok(*x),
            Object::Float(x) => Ok(x.round() as i64),
            Object::Double(x) => Ok(x.round() as i64),
            _ => Err((0, "Object::cast_to_slong"))
        };
    }

    pub fn cast_to_float(&self) -> VmResult<f32>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1.0} else {0.0}),
            Object::UByte(x) => Ok(*x as f32),
            Object::SByte(x) => Ok(*x as f32),
            Object::UInt(x) => Ok(*x as f32),
            Object::SInt(x) => Ok(*x as f32),
            Object::ULong(x) => Ok(*x as f32),
            Object::SLong(x) => Ok(*x as f32),
            Object::Float(x) => Ok(*x),
            Object::Double(x) => Ok(*x as f32),
            _ => Err((0, "Object::cast_to_float"))
        };
    }


    pub fn cast_to_double(&self) -> VmResult<f64>{
        return match self {
            Object::Boolean(x) => Ok(if *x {1.0} else {0.0}),
            Object::UByte(x) => Ok(*x as f64),
            Object::SByte(x) => Ok(*x as f64),
            Object::UInt(x) => Ok(*x as f64),
            Object::SInt(x) => Ok(*x as f64),
            Object::ULong(x) => Ok(*x as f64),
            Object::SLong(x) => Ok(*x as f64),
            Object::Float(x) => Ok(*x as f64),
            Object::Double(x) => Ok(*x),
            _ => Err((0, "Object::cast_to_double"))
        };
    }
}