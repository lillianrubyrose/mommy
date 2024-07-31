use std::cmp::Ordering;

use attribute::IRAttributeInfo;
use class_pool::{CPClassRef, CPUtf8Ref, IRClassfileError, IRCpTag};
use maya_classfile_io::{IOClassFile, IOFieldInfo, IOMethodInfo};

pub mod attribute;
pub mod class_pool;
pub mod code;

#[derive(Debug, PartialEq, Eq)]
pub struct ClassFileVersion {
	pub major: u16,
	pub minor: u16,
}

impl PartialOrd for ClassFileVersion {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ClassFileVersion {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.major.cmp(&other.major) {
			Ordering::Less => Ordering::Less,
			Ordering::Equal => self.minor.cmp(&other.minor),
			Ordering::Greater => Ordering::Greater,
		}
	}
}

pub struct AccessFlags;
impl AccessFlags {
	pub const PUBLIC: u16 = 0x001;
	pub const FINAL: u16 = 0x010;
	pub const SUPER: u16 = 0x020;
	pub const INTERFACE: u16 = 0x0200;
	pub const ABSTRACT: u16 = 0x0400;
	pub const SYNTHETIC: u16 = 0x1000;
	pub const ANNOTATION: u16 = 0x2000;
	pub const ENUM: u16 = 0x4000;
	pub const MODULE: u16 = 0x8000;
}

#[derive(Debug)]
pub struct IRFieldInfo {
	pub access_flags: u16,
	pub name: CPUtf8Ref,
	pub descriptor: CPUtf8Ref,
	pub attributes: Vec<IRAttributeInfo>,
}

impl IRFieldInfo {
	pub fn from_io(cp: &[IRCpTag], raw: IOFieldInfo) -> Result<Self, IRClassfileError> {
		let name = CPUtf8Ref::new(
			raw.name_index,
			cp.get(raw.name_index as usize - 1).expect("invalid idx"),
		);
		let descriptor = CPUtf8Ref::new(
			raw.descriptor_index,
			cp.get(raw.descriptor_index as usize - 1).expect("invalid idx"),
		);
		let attributes = raw
			.attributes
			.into_iter()
			.map(|attr| IRAttributeInfo::from_io(cp, attr))
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Self {
			access_flags: raw.access_flags,
			name,
			descriptor,
			attributes,
		})
	}
}

#[derive(Debug)]
pub struct IRMethodInfo {
	pub access_flags: u16,
	pub name: CPUtf8Ref,
	pub descriptor: CPUtf8Ref,
	pub attributes: Vec<IRAttributeInfo>,
}

impl IRMethodInfo {
	pub fn from_io(cp: &[IRCpTag], raw: IOMethodInfo) -> Result<Self, IRClassfileError> {
		let name = CPUtf8Ref::new(
			raw.name_index,
			cp.get(raw.name_index as usize - 1).expect("invalid idx"),
		);
		let descriptor = CPUtf8Ref::new(
			raw.descriptor_index,
			cp.get(raw.descriptor_index as usize - 1).expect("invalid idx"),
		);
		let attributes = raw
			.attributes
			.into_iter()
			.map(|attr| IRAttributeInfo::from_io(cp, attr))
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Self {
			access_flags: raw.access_flags,
			name,
			descriptor,
			attributes,
		})
	}
}

#[derive(Debug)]
pub struct IRClassFile {
	pub magic: u32,
	pub version: ClassFileVersion,
	pub cp: Vec<IRCpTag>,
	pub access_flags: u16,
	pub this_class: CPClassRef,
	pub super_class: CPClassRef,
	pub interfaces: Vec<CPClassRef>,
	pub fields: Vec<IRFieldInfo>,
	pub methods: Vec<IRMethodInfo>,
	pub attributes: Vec<IRAttributeInfo>,
}

impl IRClassFile {
	pub fn from_io(raw: IOClassFile) -> Result<Self, IRClassfileError> {
		let magic = raw.magic;
		let version = ClassFileVersion {
			major: raw.major_version,
			minor: raw.minor_version,
		};
		let cp = IRCpTag::from_io(raw.cp).unwrap();
		let access_flags = raw.access_flags;
		let this_class = CPClassRef::new(
			raw.this_class,
			cp.get(raw.this_class.saturating_sub(1) as usize).unwrap(),
		);
		let super_class = CPClassRef::new(
			raw.super_class,
			cp.get(raw.super_class.saturating_sub(1) as usize).unwrap(),
		);
		let interfaces = raw
			.interfaces
			.iter()
			.copied()
			.map(|idx| CPClassRef::new(idx, cp.get(idx as usize - 1).unwrap()))
			.collect();
		let fields = raw
			.fields
			.into_iter()
			.map(|f| IRFieldInfo::from_io(&cp, f))
			.collect::<Result<Vec<_>, _>>()?;
		let methods = raw
			.methods
			.into_iter()
			.map(|f| IRMethodInfo::from_io(&cp, f))
			.collect::<Result<Vec<_>, _>>()?;
		let attributes = raw
			.attributes
			.into_iter()
			.map(|attr| IRAttributeInfo::from_io(&cp, attr))
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Self {
			magic,
			version,
			cp,
			access_flags,
			this_class,
			super_class,
			interfaces,
			fields,
			methods,
			attributes,
		})
	}
}
