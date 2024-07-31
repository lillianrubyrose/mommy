use std::{rc::Rc, string::FromUtf8Error};

use maya_bytes::BytesError;
use maya_classfile_io::class_pool::IOCpTag;
use maya_mutf8::MUTFError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IRClassfileError {
	#[error("{0}")]
	Mutf8(#[from] MUTFError),
	#[error("{0}")]
	Bytes(#[from] BytesError),
	#[error("{0}")]
	Utf8(#[from] FromUtf8Error),
}

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-5.html#jvms-5.4.3.5
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum IRMethodRefKind {
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

impl IRMethodRefKind {
	pub fn from(value: u8) -> IRMethodRefKind {
		match value {
			1 => Self::GetField,
			2 => Self::GetStatic,
			3 => Self::PutField,
			4 => Self::PutStatic,
			5 => Self::InvokeVirtual,
			6 => Self::InvokeStatic,
			7 => Self::InvokeSpecial,
			8 => Self::NewInvokeSpecial,
			9 => Self::InvokeInterface,
			_ => panic!("fuck you"),
		}
	}
}

#[derive(Debug, Clone)]
pub enum CPConstValueRefKind {
	Double(f64),
	Float(f32),
	Int(i32),
	Long(i64),
	String(Rc<String>),
}

#[derive(Debug, Clone)]
pub struct CPConstValueRef {
	pub index: u16,
	pub kind: CPConstValueRefKind,
}

impl CPConstValueRef {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::Double(data) => Self {
				kind: CPConstValueRefKind::Double(*data),
				index,
			},
			IRCpTag::Float(data) => Self {
				kind: CPConstValueRefKind::Float(*data),
				index,
			},
			IRCpTag::Integer(data) => Self {
				kind: CPConstValueRefKind::Int(*data),
				index,
			},
			IRCpTag::Long(data) => Self {
				kind: CPConstValueRefKind::Long(*data),
				index,
			},
			IRCpTag::Utf8(data) => Self {
				kind: CPConstValueRefKind::String(data.clone()),
				index,
			},
			_ => panic!("trying to make CPConstValueRef from non-const tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPUtf8Ref {
	pub data: Rc<String>,
	pub index: u16,
}

impl CPUtf8Ref {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::Utf8(data) => Self {
				data: data.clone(),
				index,
			},
			_ => panic!("trying to make CPUtf8Ref from non-utf8 tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index.saturating_sub(1) as usize).expect("expected tag");
		Self::new(index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPClassRef {
	pub data: CPUtf8Ref,
	pub index: u16,
}

impl CPClassRef {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::Class(this) => Self {
				data: this.clone(),
				index,
			},
			_ => panic!("trying to make CPUtf8Ref from non-utf8 tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPNameAndTypeRef {
	pub index: u16,
	pub name: CPUtf8Ref,
	pub ty: CPUtf8Ref,
}

impl CPNameAndTypeRef {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::NameAndType { name, descriptor } => Self {
				name: name.clone(),
				ty: descriptor.clone(),
				index,
			},
			_ => panic!("trying to make CPNameAndTypeRef from non-NameAndType tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(index, tag)
	}
}

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8
#[derive(Debug, Clone)]
pub struct CPMethodHandleRef {
	pub ref_kind: IRMethodRefKind,
	pub ref_tag: Box<IRCpTag>,
	pub ref_index: u16,
	pub index: u16,
}

impl CPMethodHandleRef {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::MethodHandle {
				ref_kind,
				ref_index,
				ref_tag,
			} => Self {
				ref_kind: ref_kind.clone(),
				ref_tag: ref_tag.clone(),
				ref_index: *ref_index,
				index,
			},
			_ => panic!("trying to make CPMethodHandleRef from non-MethodHandle tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPModuleInfoRef {
	pub data: CPUtf8Ref,
	pub index: u16,
}

impl CPModuleInfoRef {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::Module { name } => Self {
				data: name.clone(),
				index,
			},
			_ => panic!("trying to make CPModuleInfoRef from non-CPModuleInfoRef tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPPackageInfoRef {
	pub data: CPUtf8Ref,
	pub index: u16,
}

impl CPPackageInfoRef {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::Package { name } => Self {
				data: name.clone(),
				index,
			},
			_ => panic!("trying to make CPUtf8Ref from non-utf8 tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(index, tag)
	}
}

#[macro_export]
macro_rules! get_from_cp {
	($cp:ident, $idx:ident, $ty:ident) => {{
		match $cp.get(*$idx as usize - 1).expect("fuck") {
			IRCpTag::$ty(v) => v,
			t => panic!("expected different type: {} | got: {t:?}", stringify!($ty)),
		}
		.clone()
	}};
}

#[derive(Debug, Clone)]
pub struct CPFieldRef {
	pub class: CPClassRef,
	pub name_and_ty: CPNameAndTypeRef,
	pub index: u16,
}

impl CPFieldRef {
	pub fn new(cp: &[IRCpTag], index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::FieldRef {
				class_index,
				name_and_ty,
			} => {
				let class_tag = cp.get(class_index.saturating_sub(1) as usize).expect("fuck");
				Self {
					class: CPClassRef::new(*class_index, class_tag),
					name_and_ty: name_and_ty.clone(),
					index,
				}
			}
			_ => panic!("trying to make CPUtf8Ref from non-utf8 tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(cp, index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPMethodRef {
	pub class: CPClassRef,
	pub name_and_ty: CPNameAndTypeRef,
	pub index: u16,
}

impl CPMethodRef {
	pub fn new(cp: &[IRCpTag], index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::MethodRef {
				class_index,
				name_and_ty,
			} => {
				let class_tag = cp.get(class_index.saturating_sub(1) as usize).expect("fuck");
				Self {
					class: CPClassRef::new(*class_index, class_tag),
					name_and_ty: name_and_ty.clone(),
					index,
				}
			}
			_ => panic!("trying to make CPUtf8Ref from non-utf8 tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(cp, index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPInvokeDynamicRef {
	pub bootstrap_method_attr_index: u16,
	pub name_and_ty: CPNameAndTypeRef,
	pub index: u16,
}

impl CPInvokeDynamicRef {
	pub fn new(cp: &[IRCpTag], index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::InvokeDynamic {
				bootstrap_method_attr_index,
				name_and_ty,
			} => Self {
				bootstrap_method_attr_index: *bootstrap_method_attr_index,
				name_and_ty: name_and_ty.clone(),
				index,
			},
			_ => panic!("trying to make CPUtf8Ref from non-utf8 tag. {utf8_tag:?}"),
		}
	}

	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self::new(cp, index, tag)
	}
}

#[derive(Debug, Clone)]
pub struct CPTagRef {
	pub tag: IRCpTag,
	pub index: u16,
}

impl CPTagRef {
	pub fn from_cp(cp: &[IRCpTag], index: u16) -> Self {
		let tag = cp.get(index as usize - 1).expect("expected tag");
		Self {
			tag: tag.clone(),
			index,
		}
	}
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum IRCpTag {
	Utf8(Rc<String>) = 1,
	Integer(i32) = 3,
	Float(f32) = 4,
	Long(i64) = 5,
	Double(f64) = 6,
	Class(CPUtf8Ref) = 7,
	String(CPUtf8Ref) = 8,
	FieldRef {
		class_index: u16,
		name_and_ty: CPNameAndTypeRef,
	} = 9,
	MethodRef {
		class_index: u16,
		name_and_ty: CPNameAndTypeRef,
	} = 10,
	InterfaceMethodRef {
		class_index: u16,
		name_and_ty: CPNameAndTypeRef,
	} = 11,
	NameAndType {
		name: CPUtf8Ref,
		descriptor: CPUtf8Ref,
	} = 12,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8
	MethodHandle {
		ref_kind: IRMethodRefKind,
		ref_index: u16,
		ref_tag: Box<IRCpTag>,
	} = 15,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9
	MethodType(CPUtf8Ref) = 16,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10
	InvokeDynamic {
		bootstrap_method_attr_index: u16,
		name_and_ty: CPNameAndTypeRef,
	} = 18,
	Module {
		name: CPUtf8Ref,
	} = 19,
	Package {
		name: CPUtf8Ref,
	} = 20,
}

macro_rules! parse_tag_idx {
	($idx:ident, $raw_tags:ident, $formed_tags:ident) => {
		$formed_tags.get(*$idx as usize - 1).cloned().or(Some(Self::parse_tag(
			&$raw_tags[*$idx as usize - 1],
			$raw_tags,
			$formed_tags,
		)?))
	};
}

impl IRCpTag {
	fn parse_tag(tag: &IOCpTag, raw_tags: &[IOCpTag], formed_tags: &[IRCpTag]) -> Result<IRCpTag, IRClassfileError> {
		Ok(match tag {
			IOCpTag::Utf8 { length: _, bytes } => IRCpTag::Utf8(Rc::new(maya_mutf8::decode(bytes)?)),
			IOCpTag::Integer { bytes } => IRCpTag::Integer(i32::from_be_bytes(*bytes)),
			IOCpTag::Float { bytes } => IRCpTag::Float(f32::from_be_bytes(*bytes)),
			IOCpTag::Long { bytes } => IRCpTag::Long(i64::from_be_bytes(*bytes)),
			IOCpTag::Double { bytes } => IRCpTag::Double(f64::from_be_bytes(*bytes)),
			IOCpTag::Class { name_index } => {
				let utf8_tag = parse_tag_idx!(name_index, raw_tags, formed_tags).expect("invalid Class name_index");
				IRCpTag::Class(CPUtf8Ref::new(*name_index, &utf8_tag))
			}
			IOCpTag::String { utf8_index } => {
				let utf8_tag = parse_tag_idx!(utf8_index, raw_tags, formed_tags).expect("invalid String utf8_index");
				IRCpTag::String(CPUtf8Ref::new(*utf8_index, &utf8_tag))
			}
			IOCpTag::FieldRef {
				class_index,
				name_and_ty_index,
			} => {
				let (name, ty) = match parse_tag_idx!(name_and_ty_index, raw_tags, formed_tags)
					.expect("invalid FieldRef name_and_ty_index")
				{
					IRCpTag::NameAndType { name, descriptor } => (name, descriptor),
					t => panic!("expected NameAndType. got {t:?}"),
				};
				IRCpTag::FieldRef {
					class_index: *class_index,
					name_and_ty: CPNameAndTypeRef {
						index: *name_and_ty_index,
						name: name.clone(),
						ty: ty.clone(),
					},
				}
			}
			IOCpTag::MethodRef {
				class_index,
				name_and_ty_index,
			} => {
				let (name, ty) = match parse_tag_idx!(name_and_ty_index, raw_tags, formed_tags)
					.expect("invalid MethodRef name_and_ty_index")
				{
					IRCpTag::NameAndType { name, descriptor } => (name, descriptor),
					t => panic!("expected NameAndType. got {t:?}"),
				};
				IRCpTag::MethodRef {
					class_index: *class_index,
					name_and_ty: CPNameAndTypeRef {
						index: *name_and_ty_index,
						name: name.clone(),
						ty: ty.clone(),
					},
				}
			}
			IOCpTag::InterfaceMethodRef {
				class_index,
				name_and_ty_index,
			} => {
				let (name, ty) = match parse_tag_idx!(name_and_ty_index, raw_tags, formed_tags)
					.expect("invalid InterfaceMethodRef name_and_ty_index")
				{
					IRCpTag::NameAndType { name, descriptor } => (name, descriptor),
					t => panic!("expected NameAndType. got {t:?}"),
				};
				IRCpTag::InterfaceMethodRef {
					class_index: *class_index,
					name_and_ty: CPNameAndTypeRef {
						index: *name_and_ty_index,
						name: name.clone(),
						ty: ty.clone(),
					},
				}
			}
			IOCpTag::NameAndType {
				name_index,
				descriptor_index,
			} => {
				let name_tag = parse_tag_idx!(name_index, raw_tags, formed_tags).expect("expected utf8 tag");
				let descriptor_tag =
					parse_tag_idx!(descriptor_index, raw_tags, formed_tags).expect("expected utf8 tag");
				IRCpTag::NameAndType {
					name: CPUtf8Ref::new(*name_index, &name_tag),
					descriptor: CPUtf8Ref::new(*descriptor_index, &descriptor_tag),
				}
			}
			IOCpTag::MethodHandle {
				reference_kind: reference_kind_idx,
				reference_index,
			} => {
				let kind = IRMethodRefKind::from(*reference_kind_idx);
				let tag = parse_tag_idx!(reference_index, raw_tags, formed_tags).expect("expected tag");
				IRCpTag::MethodHandle {
					ref_kind: kind,
					ref_tag: Box::new(tag.clone()),
					ref_index: *reference_index,
				}
			}
			IOCpTag::MethodType { descriptor_index } => {
				let tag = parse_tag_idx!(descriptor_index, raw_tags, formed_tags).expect("expected utf8 tag");
				IRCpTag::MethodType(CPUtf8Ref::new(*descriptor_index, &tag))
			}
			IOCpTag::InvokeDynamic {
				bootstrap_method_attr_index,
				name_and_ty_index,
			} => {
				let (name, ty) = match parse_tag_idx!(name_and_ty_index, raw_tags, formed_tags)
					.expect("invalid InvokeDynamic name_and_ty_index")
				{
					IRCpTag::NameAndType { name, descriptor } => (name, descriptor),
					t => panic!("expected NameAndType. got {t:?}"),
				};
				IRCpTag::InvokeDynamic {
					bootstrap_method_attr_index: *bootstrap_method_attr_index,
					name_and_ty: CPNameAndTypeRef {
						index: *name_and_ty_index,
						name: name.clone(),
						ty: ty.clone(),
					},
				}
			}
			IOCpTag::Module { name_index } => {
				let name_tag = parse_tag_idx!(name_index, raw_tags, formed_tags).expect("expected utf8 tag");
				IRCpTag::Module {
					name: CPUtf8Ref::new(*name_index, &name_tag),
				}
			}
			IOCpTag::Package { name_index } => {
				let name_tag = parse_tag_idx!(name_index, raw_tags, formed_tags).expect("expected utf8 tag");
				IRCpTag::Package {
					name: CPUtf8Ref::new(*name_index, &name_tag),
				}
			}
		})
	}

	pub fn from_io(raw_tags: Vec<IOCpTag>) -> Result<Vec<IRCpTag>, IRClassfileError> {
		let mut res = Vec::with_capacity(raw_tags.len());

		for raw_tag in &raw_tags {
			let tag = Self::parse_tag(raw_tag, &raw_tags, &res)?;
			res.push(tag);
		}

		Ok(res)
	}
}
