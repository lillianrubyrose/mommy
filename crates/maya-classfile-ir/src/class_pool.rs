use std::rc::Rc;

use maya_classfile_io::class_pool::IOCpTag;
use maya_mutf8::MUTFError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IRClassfileError {
	#[error("{0}")]
	Mutf8(#[from] MUTFError),
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
pub struct CPUtf8Ref {
	pub data: Rc<String>,
	pub index: u16,
}

impl CPUtf8Ref {
	pub fn new(index: u16, utf8_tag: &IRCpTag) -> Self {
		match utf8_tag {
			IRCpTag::Class(this) => Self {
				data: this.data.clone(),
				index,
			},
			IRCpTag::Utf8(data) => Self {
				data: data.clone(),
				index,
			},
			_ => panic!("trying to make CPUtf8Ref from non-utf8 tag. {utf8_tag:?}"),
		}
	}
}

#[derive(Debug, Clone)]
pub struct CPNameAndTypeRef {
	pub index: u16,
	pub name: CPUtf8Ref,
	pub ty: CPUtf8Ref,
}

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.8
#[derive(Debug, Clone)]
pub struct CPMethodHandleRef {
	pub kind: IRMethodRefKind,
	pub ref_tag: Box<IRCpTag>,
	pub ref_index: u16,
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
	MethodHandle(CPMethodHandleRef) = 15,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.9
	MethodType(CPUtf8Ref) = 16,
	// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.10
	InvokeDynamic {
		bootstrap_method_attr_index: u16,
		name_and_ty: CPNameAndTypeRef,
	} = 18,
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
			IOCpTag::Utf8 { length: _, bytes } => IRCpTag::Utf8(Rc::new(maya_mutf8::decode(&bytes)?)),
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
					.expect(&format!("invalid FieldRef name_and_ty_index"))
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
					.expect(&format!("invalid MethodRef name_and_ty_index"))
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
					.expect(&format!("invalid InterfaceMethodRef name_and_ty_index"))
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
				IRCpTag::MethodHandle(CPMethodHandleRef {
					kind,
					ref_tag: Box::new(tag.clone()),
					ref_index: *reference_index,
				})
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
					.expect(&format!("invalid InvokeDynamic name_and_ty_index"))
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