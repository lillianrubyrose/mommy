use maya_bytes::*;

use crate::ClassfileIOError;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-5.html#jvms-5.4.3.5
#[repr(u8)]
pub enum MethodRefKind {
	GetField = 1,
	GetStatic,
	PutField,
	PutStatic,
	InvokeVirtual,
	InvokeStatic,
	InvokeSpecial,
	NewInvokeSpecial,
	InvokeInterface,
}

#[derive(Debug)]
#[repr(u8)]
pub enum CpTag {
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7
	Utf8 {
		length: u16,
		bytes: Vec<u8>,
	} = 1,
	Integer {
		bytes: [u8; 4],
	} = 3,
	Float {
		bytes: [u8; 4],
	} = 4,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5
	// All 8-byte constants take up two entries in the constant_pool table of the class file.
	// If a CONSTANT_Long_info or CONSTANT_Double_info structure is the item in the constant_pool table-
	// at index n, then the next usable item in the pool is located at index n+2.
	// The constant_pool index n+1 must be valid but is considered unusable.
	Long {
		high_bytes: [u8; 4],
		low_bytes: [u8; 4],
	} = 5,
	Double {
		high_bytes: [u8; 4],
		low_bytes: [u8; 4],
	} = 6,
	Class {
		name_index: u16,
	} = 7,
	String {
		utf8_index: u16,
	} = 8,
	FieldRef {
		class_index: u16,
		name_and_ty_index: u16,
	} = 9,
	MethodRef {
		class_index: u16,
		name_and_ty_index: u16,
	} = 10,
	InterfaceMethodRef {
		class_index: u16,
		name_and_ty_index: u16,
	} = 11,
	NameAndType {
		name_index: u16,
		descriptor_index: u16,
	} = 12,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8
	MethodHandle {
		reference_kind: u8,
		reference_index: u16,
	} = 15,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9
	MethodType {
		descriptor_index: u16,
	} = 16,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10
	InvokeDynamic {
		bootstrap_method_attr_index: u16,
		name_and_type_index: u16,
	} = 18,
}

impl CpTag {
	pub fn read<B: BytesExt>(
		buffer: &mut B,
	) -> Result<CpTag, ClassfileIOError> {
		let tag = buffer.read_u8()?;
		match tag {
			1 => {
				let len = buffer.read_u16()?;
				let mut bytes = Vec::with_capacity(len as usize);
				for _ in 0..len {
					bytes.push(buffer.read_u8()?);
				}

				Ok(CpTag::Utf8 { length: len, bytes })
			}
			3 => Ok(CpTag::Integer {
				bytes: buffer.read_n_bytes::<4>()?,
			}),
			4 => Ok(CpTag::Float {
				bytes: buffer.read_n_bytes::<4>()?,
			}),
			5 => Ok(CpTag::Long {
				high_bytes: buffer.read_n_bytes::<4>()?,
				low_bytes: buffer.read_n_bytes::<4>()?,
			}),
			6 => Ok(CpTag::Double {
				high_bytes: buffer.read_n_bytes::<4>()?,
				low_bytes: buffer.read_n_bytes::<4>()?,
			}),
			7 => Ok(CpTag::Class {
				name_index: buffer.read_u16()?,
			}),
			8 => Ok(CpTag::String {
				utf8_index: buffer.read_u16()?,
			}),
			9 => Ok(CpTag::FieldRef {
				class_index: buffer.read_u16()?,
				name_and_ty_index: buffer.read_u16()?,
			}),
			10 => Ok(CpTag::MethodRef {
				class_index: buffer.read_u16()?,
				name_and_ty_index: buffer.read_u16()?,
			}),
			11 => Ok(CpTag::InterfaceMethodRef {
				class_index: buffer.read_u16()?,
				name_and_ty_index: buffer.read_u16()?,
			}),
			12 => Ok(CpTag::NameAndType {
				name_index: buffer.read_u16()?,
				descriptor_index: buffer.read_u16()?,
			}),
			15 => Ok(CpTag::MethodHandle {
				reference_kind: buffer.read_u8()?,
				reference_index: buffer.read_u16()?,
			}),
			16 => Ok(CpTag::MethodType {
				descriptor_index: buffer.read_u16()?,
			}),
			18 => Ok(CpTag::InvokeDynamic {
				bootstrap_method_attr_index: buffer.read_u16()?,
				name_and_type_index: buffer.read_u16()?,
			}),
			_ => unimplemented!("unimplemented tag: {tag}"),
		}
	}

	pub fn id(&self) -> u8 {
		match self {
			CpTag::Utf8 {
				length: _,
				bytes: _,
			} => 1,
			CpTag::Integer { bytes: _ } => 3,
			CpTag::Float { bytes: _ } => 4,
			CpTag::Long {
				high_bytes: _,
				low_bytes: _,
			} => 5,
			CpTag::Double {
				high_bytes: _,
				low_bytes: _,
			} => 6,
			CpTag::Class { name_index: _ } => 7,
			CpTag::String { utf8_index: _ } => 8,
			CpTag::FieldRef {
				class_index: _,
				name_and_ty_index: _,
			} => 9,
			CpTag::MethodRef {
				class_index: _,
				name_and_ty_index: _,
			} => 10,
			CpTag::InterfaceMethodRef {
				class_index: _,
				name_and_ty_index: _,
			} => 11,
			CpTag::NameAndType {
				name_index: _,
				descriptor_index: _,
			} => 12,
			CpTag::MethodHandle {
				reference_kind: _,
				reference_index: _,
			} => 15,
			CpTag::MethodType {
				descriptor_index: _,
			} => 16,
			CpTag::InvokeDynamic {
				bootstrap_method_attr_index: _,
				name_and_type_index: _,
			} => 18,
		}
	}

	pub fn write<B: BytesExt>(
		&self,
		buffer: &mut B,
	) -> Result<(), ClassfileIOError> {
		buffer.write_u8(self.id())?;
		match self {
			CpTag::Utf8 { length, bytes } => {
				buffer.write_u16(*length)?;
				buffer.write_all(bytes)?;
			}
			CpTag::Integer { bytes } => buffer.write_all(bytes)?,
			CpTag::Float { bytes } => buffer.write_all(bytes)?,
			CpTag::Long {
				high_bytes,
				low_bytes,
			} => {
				buffer.write_all(high_bytes)?;
				buffer.write_all(low_bytes)?;
			}
			CpTag::Double {
				high_bytes,
				low_bytes,
			} => {
				buffer.write_all(high_bytes)?;
				buffer.write_all(low_bytes)?;
			}
			CpTag::Class { name_index } => {
				buffer.write_u16(*name_index)?
			}
			CpTag::String { utf8_index } => {
				buffer.write_u16(*utf8_index)?
			}
			CpTag::FieldRef {
				class_index,
				name_and_ty_index,
			} => {
				buffer.write_u16(*class_index)?;
				buffer.write_u16(*name_and_ty_index)?;
			}
			CpTag::MethodRef {
				class_index,
				name_and_ty_index,
			} => {
				buffer.write_u16(*class_index)?;
				buffer.write_u16(*name_and_ty_index)?;
			}
			CpTag::InterfaceMethodRef {
				class_index,
				name_and_ty_index,
			} => {
				buffer.write_u16(*class_index)?;
				buffer.write_u16(*name_and_ty_index)?;
			}
			CpTag::NameAndType {
				name_index,
				descriptor_index,
			} => {
				buffer.write_u16(*name_index)?;
				buffer.write_u16(*descriptor_index)?;
			}
			CpTag::MethodHandle {
				reference_kind,
				reference_index,
			} => {
				buffer.write_u8(*reference_kind)?;
				buffer.write_u16(*reference_index)?;
			}
			CpTag::MethodType { descriptor_index } => {
				buffer.write_u16(*descriptor_index)?;
			}
			CpTag::InvokeDynamic {
				bootstrap_method_attr_index,
				name_and_type_index,
			} => {
				buffer.write_u16(*bootstrap_method_attr_index)?;
				buffer.write_u16(*name_and_type_index)?;
			}
		}
		Ok(())
	}
}

pub struct AccessFlags;
impl AccessFlags {
	pub const ACC_PUBLIC: u16 = 0x001;
	pub const ACC_FINAL: u16 = 0x010;
	pub const ACC_SUPER: u16 = 0x020;
	pub const ACC_INTERFACE: u16 = 0x0200;
	pub const ACC_ABSTRACT: u16 = 0x0400;
	pub const ACC_SYNTHETIC: u16 = 0x1000;
	pub const ACC_ANNOTATION: u16 = 0x2000;
	pub const ACC_ENUM: u16 = 0x4000;
}
