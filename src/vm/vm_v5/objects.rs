use super::vm_state::VmError;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PassBy {
	Value,     // Does a shallow copy
	Reference, // Returns the pointer
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

// TRAIT IMPLEMENTATIONS:
// TODO?: when all types are added, convert `TryFrom` to `From` for impls which wont be able to fail
use Object::*;

impl TryFrom<Object> for bool {
	type Error = VmError;

	#[allow(unreachable_patterns)] // wildcard might become reachable when more types are added
	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(x),
			UBYTE(x) => Ok(x == 0),
			SBYTE(x) => Ok(x == 0),
			UINT(x) => Ok(x == 0),
			SINT(x) => Ok(x == 0),
			ULONG(x) => Ok(x == 0),
			SLONG(x) => Ok(x == 0),
			FLOAT(x) => Ok(x == 0.0),
			DOUBLE(x) => Ok(x == 0.0),
			STRING(x) => Ok(x.len() != 0),
			_ => Err((0, "Object::castToBool")),
		}
	}
}

impl TryFrom<Object> for u8 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1 } else { 0 }),
			UBYTE(x) => Ok(x),
			SBYTE(x) => Ok(x as u8),
			UINT(x) => Ok(x as u8),
			SINT(x) => Ok(x as u8),
			ULONG(x) => Ok(x as u8),
			SLONG(x) => Ok(x as u8),
			FLOAT(x) => Ok(x.round() as u8),
			DOUBLE(x) => Ok(x.round() as u8),
			_ => Err((0, "Object::castToUByte")),
		}
	}
}

impl TryFrom<Object> for i8 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1 } else { 0 }),
			UBYTE(x) => Ok(x as i8),
			SBYTE(x) => Ok(x),
			UINT(x) => Ok(x as i8),
			SINT(x) => Ok(x as i8),
			ULONG(x) => Ok(x as i8),
			SLONG(x) => Ok(x as i8),
			FLOAT(x) => Ok(x.round() as i8),
			DOUBLE(x) => Ok(x.round() as i8),
			_ => Err((0, "Object::castToSByte")),
		}
	}
}

impl TryFrom<Object> for u32 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1 } else { 0 }),
			UBYTE(x) => Ok(x as u32),
			SBYTE(x) => Ok(x as u32),
			UINT(x) => Ok(x),
			SINT(x) => Ok(x as u32),
			ULONG(x) => Ok(x as u32),
			SLONG(x) => Ok(x as u32),
			FLOAT(x) => Ok(x.round() as u32),
			DOUBLE(x) => Ok(x.round() as u32),
			_ => Err((0, "Object::castToUInt")),
		}
	}
}

impl TryFrom<Object> for i32 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1 } else { 0 }),
			UBYTE(x) => Ok(x as i32),
			SBYTE(x) => Ok(x as i32),
			UINT(x) => Ok(x as i32),
			SINT(x) => Ok(x),
			ULONG(x) => Ok(x as i32),
			SLONG(x) => Ok(x as i32),
			FLOAT(x) => Ok(x.round() as i32),
			DOUBLE(x) => Ok(x.round() as i32),
			_ => Err((0, "Object::castToSInt")),
		}
	}
}

impl TryFrom<Object> for u64 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1 } else { 0 }),
			UBYTE(x) => Ok(x as u64),
			SBYTE(x) => Ok(x as u64),
			UINT(x) => Ok(x as u64),
			SINT(x) => Ok(x as u64),
			ULONG(x) => Ok(x),
			SLONG(x) => Ok(x as u64),
			FLOAT(x) => Ok(x.round() as u64),
			DOUBLE(x) => Ok(x.round() as u64),
			_ => Err((0, "Object::castToULong")),
		}
	}
}

impl TryFrom<Object> for i64 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1 } else { 0 }),
			UBYTE(x) => Ok(x as i64),
			SBYTE(x) => Ok(x as i64),
			UINT(x) => Ok(x as i64),
			SINT(x) => Ok(x as i64),
			ULONG(x) => Ok(x as i64),
			SLONG(x) => Ok(x),
			FLOAT(x) => Ok(x.round() as i64),
			DOUBLE(x) => Ok(x.round() as i64),
			_ => Err((0, "Object::castToSLong")),
		}
	}
}

impl TryFrom<Object> for f32 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1.0 } else { 0.0 }),
			UBYTE(x) => Ok(x as f32),
			SBYTE(x) => Ok(x as f32),
			UINT(x) => Ok(x as f32),
			SINT(x) => Ok(x as f32),
			ULONG(x) => Ok(x as f32),
			SLONG(x) => Ok(x as f32),
			FLOAT(x) => Ok(x),
			DOUBLE(x) => Ok(x as f32),
			_ => Err((0, "Object::castToFloat")),
		}
	}
}

impl TryFrom<Object> for f64 {
	type Error = VmError;

	fn try_from(val: Object) -> Result<Self, Self::Error> {
		match val {
			BOOLEAN(x) => Ok(if x { 1.0 } else { 0.0 }),
			UBYTE(x) => Ok(x as f64),
			SBYTE(x) => Ok(x as f64),
			UINT(x) => Ok(x as f64),
			SINT(x) => Ok(x as f64),
			ULONG(x) => Ok(x as f64),
			SLONG(x) => Ok(x as f64),
			FLOAT(x) => Ok(x as f64),
			DOUBLE(x) => Ok(x),
			_ => Err((0, "Object::castToDouble")),
		}
	}
}
