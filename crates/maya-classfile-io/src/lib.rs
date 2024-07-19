pub mod class_pool;

use class_pool::IOCpTag;
use maya_bytes::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IOClassfileError {
	#[error("First 4 bytes were not 0xCAFEBABE")]
	InvalidMagic,
	#[error("{0}")]
	Bytes(#[from] BytesError),
	#[error("IO Error: {0}")]
	IO(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct IOClassFile {
	pub magic: u32,
	pub minor_version: u16,
	pub major_version: u16,
	pub cp_count: u16,
	pub cp: Vec<IOCpTag>,
	pub access_flags: u16,
	pub this_class: u16,
	pub super_class: u16,
	pub interface_count: u16,
	pub interfaces: Vec<u16>,
	pub field_count: u16,
	pub fields: Vec<IOFieldInfo>,
	pub method_count: u16,
	pub methods: Vec<IOMethodInfo>,
	pub attribute_count: u16,
	pub attributes: Vec<IOAttributeInfo>,
}

impl IOClassFile {
	pub fn read<B: BytesExt>(
		buffer: &mut B,
	) -> Result<IOClassFile, IOClassfileError> {
		let magic = buffer.read_u32()?;
		if magic != 0xCAFEBABE {
			return Err(IOClassfileError::InvalidMagic);
		}

		let minor_version = buffer.read_u16()?;
		let major_version = buffer.read_u16()?;
		let cp_count = buffer.read_u16()?;
		let mut cp = Vec::with_capacity(cp_count as usize - 1);
		for _ in 0..cp_count - 1 {
			cp.push(IOCpTag::read(buffer)?);
		}
		let access_flags = buffer.read_u16()?;
		let this_class = buffer.read_u16()?;
		let super_class = buffer.read_u16()?;
		let interface_count = buffer.read_u16()?;
		let mut interfaces = Vec::with_capacity(interface_count as usize);
		for _ in 0..interface_count {
			interfaces.push(buffer.read_u16()?);
		}
		let field_count = buffer.read_u16()?;
		let mut fields = Vec::with_capacity(field_count as usize);
		for _ in 0..field_count {
			fields.push(IOFieldInfo::read(buffer)?);
		}
		let method_count = buffer.read_u16()?;
		let mut methods = Vec::with_capacity(method_count as usize);
		for _ in 0..method_count {
			methods.push(IOMethodInfo::read(buffer)?);
		}
		let attribute_count = buffer.read_u16()?;
		let mut attributes = Vec::with_capacity(attribute_count as usize);
		for _ in 0..attribute_count {
			attributes.push(IOAttributeInfo::read(buffer)?);
		}

		Ok(Self {
			magic,
			minor_version,
			major_version,
			cp_count,
			cp,
			access_flags,
			this_class,
			super_class,
			interface_count,
			interfaces,
			field_count,
			fields,
			method_count,
			methods,
			attribute_count,
			attributes,
		})
	}

	pub fn write<B: BytesExt>(
		&self,
		buffer: &mut B,
	) -> Result<(), IOClassfileError> {
		buffer.write_u32(self.magic)?;
		buffer.write_u16(self.minor_version)?;
		buffer.write_u16(self.major_version)?;
		buffer.write_u16(self.cp_count)?;
		for cp in &self.cp {
			cp.write(buffer)?;
		}
		buffer.write_u16(self.access_flags)?;
		buffer.write_u16(self.this_class)?;
		buffer.write_u16(self.super_class)?;
		buffer.write_u16(self.interface_count)?;
		for iface in &self.interfaces {
			buffer.write_u16(*iface)?;
		}
		buffer.write_u16(self.field_count)?;
		for field in &self.fields {
			field.write(buffer)?;
		}
		buffer.write_u16(self.method_count)?;
		for method in &self.methods {
			method.write(buffer)?;
		}
		buffer.write_u16(self.attribute_count)?;
		for attr in &self.attributes {
			attr.write(buffer)?;
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct IOAttributeInfo {
	pub attribute_name_index: u16,
	pub attribute_length: u32,
	pub info: Vec<u8>,
}

impl IOAttributeInfo {
	pub fn read<B: BytesExt>(
		buffer: &mut B,
	) -> Result<IOAttributeInfo, BytesError> {
		let attribute_name_index = buffer.read_u16()?;
		let attribute_length = buffer.read_u32()?;
		Ok(IOAttributeInfo {
			attribute_name_index,
			attribute_length,
			info: buffer.read_n_bytes_vec(attribute_length as usize)?,
		})
	}

	pub fn write<B: BytesExt>(
		&self,
		buffer: &mut B,
	) -> Result<(), IOClassfileError> {
		buffer.write_u16(self.attribute_name_index)?;
		buffer.write_u32(self.attribute_length)?;
		buffer.write_all(&self.info)?;
		Ok(())
	}
}

#[derive(Debug)]
pub struct IOFieldInfo {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Vec<IOAttributeInfo>,
}

impl IOFieldInfo {
	pub fn read<B: BytesExt>(
		buffer: &mut B,
	) -> Result<IOFieldInfo, BytesError> {
		let access_flags = buffer.read_u16()?;
		let name_index = buffer.read_u16()?;
		let descriptor_index = buffer.read_u16()?;
		let attributes_count = buffer.read_u16()?;
		let mut attributes = Vec::with_capacity(attributes_count as usize);
		for _ in 0..attributes_count {
			attributes.push(IOAttributeInfo::read(buffer)?);
		}

		Ok(IOFieldInfo {
			access_flags,
			name_index,
			descriptor_index,
			attributes_count,
			attributes,
		})
	}

	pub fn write<B: BytesExt>(
		&self,
		buffer: &mut B,
	) -> Result<(), IOClassfileError> {
		buffer.write_u16(self.access_flags)?;
		buffer.write_u16(self.name_index)?;
		buffer.write_u16(self.descriptor_index)?;
		buffer.write_u16(self.attributes_count)?;
		for attr in &self.attributes {
			attr.write(buffer)?;
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct IOMethodInfo {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Vec<IOAttributeInfo>,
}

impl IOMethodInfo {
	pub fn read<B: BytesExt>(
		buffer: &mut B,
	) -> Result<IOMethodInfo, IOClassfileError> {
		let access_flags = buffer.read_u16()?;
		let name_index = buffer.read_u16()?;
		let descriptor_index = buffer.read_u16()?;
		let attributes_count = buffer.read_u16()?;
		let mut attributes = Vec::with_capacity(attributes_count as usize);
		for _ in 0..attributes_count {
			attributes.push(IOAttributeInfo::read(buffer)?);
		}

		Ok(IOMethodInfo {
			access_flags,
			name_index,
			descriptor_index,
			attributes_count,
			attributes,
		})
	}

	pub fn write<B: BytesExt>(
		&self,
		buffer: &mut B,
	) -> Result<(), IOClassfileError> {
		buffer.write_u16(self.access_flags)?;
		buffer.write_u16(self.name_index)?;
		buffer.write_u16(self.descriptor_index)?;
		buffer.write_u16(self.attributes_count)?;
		for attr in &self.attributes {
			attr.write(buffer)?;
		}
		Ok(())
	}
}
