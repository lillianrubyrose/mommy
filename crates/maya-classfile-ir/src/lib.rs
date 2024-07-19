use class_pool::{CPUtf8Ref, IRCpTag};
use maya_classfile_io::IOClassFile;

pub mod class_pool;

#[derive(Debug, PartialEq, Eq)]
pub struct ClassFileVersion {
	pub major: u16,
	pub minor: u16,
}

impl PartialOrd for ClassFileVersion {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ClassFileVersion {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self.major.cmp(&other.major) {
			std::cmp::Ordering::Less => std::cmp::Ordering::Less,
			std::cmp::Ordering::Equal => self.minor.cmp(&other.minor),
			std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
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
}

#[derive(Debug)]
pub struct IRClassFile {
	pub magic: u32,
	pub version: ClassFileVersion,
	pub cp: Vec<IRCpTag>,
	pub access_flags: u16,
	pub this_class: CPUtf8Ref,
	pub super_class: CPUtf8Ref,
	pub interfaces: Vec<CPUtf8Ref>,
}

impl IRClassFile {
	pub fn from_io(raw: IOClassFile) -> Self {
		let magic = raw.magic;
		let version = ClassFileVersion {
			major: raw.major_version,
			minor: raw.minor_version,
		};
		let cp = IRCpTag::from_io(raw.cp).unwrap();
		let access_flags = raw.access_flags;
		let this_class = CPUtf8Ref::new(raw.this_class, cp.get(raw.this_class as usize - 1).unwrap());
		let super_class = CPUtf8Ref::new(raw.super_class, cp.get(raw.super_class as usize - 1).unwrap());
		let interfaces = raw
			.interfaces
			.iter()
			.copied()
			.map(|idx| CPUtf8Ref::new(idx, cp.get(idx as usize - 1).unwrap()))
			.collect();

		Self {
			magic,
			version,
			cp,
			access_flags,
			this_class,
			super_class,
			interfaces,
		}
	}
}
