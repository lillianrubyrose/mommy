use std::io::Cursor;

use maya_bytes::BytesReadExt;
use maya_classfile_io::{class_pool::IOCpTag, IOAttributeInfo};

use crate::class_pool::{CPClassRef, CPNameAndTypeRef, CPUtf8Ref, IRClassfileError, IRCpTag};

#[derive(Debug, Clone)]
pub enum ConstantValueAttribute {
	Long { cp_idx: u16, value: i64 },
	Float { cp_idx: u16, value: f32 },
	Double { cp_idx: u16, value: f64 },
	Int { cp_idx: u16, value: i32 },
	String(CPUtf8Ref),
}

#[derive(Debug, Clone)]
pub struct StackMapTableAttribute {
	pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum VerificationTypeInfo {
	TopVariableInfo = 0,
	IntegerVariableInfo = 1,
	FloatVariableInfo = 2,
	LongVariableInfo = 4,
	DoubleVariableInfo = 3,
	NullVariableInfo = 5,
	UninitializedThisVariableInfo = 6,
	ObjectVariableInfo { cpool_idx: u16 } = 7,
	UninitializedVariableInfo { offset: u16 } = 8,
}

impl VerificationTypeInfo {
	fn read<B: BytesReadExt>(buffer: &mut B) -> Result<VerificationTypeInfo, IRClassfileError> {
		let tag = buffer.read_u8()?;
		Ok(match tag {
			0 => Self::TopVariableInfo,
			1 => Self::IntegerVariableInfo,
			2 => Self::FloatVariableInfo,
			4 => Self::LongVariableInfo,
			3 => Self::DoubleVariableInfo,
			5 => Self::NullVariableInfo,
			6 => Self::UninitializedThisVariableInfo,
			7 => Self::ObjectVariableInfo {
				cpool_idx: buffer.read_u16()?,
			},
			8 => Self::UninitializedVariableInfo {
				offset: buffer.read_u16()?,
			},
			_ => unreachable!("invalid tag {tag}"),
		})
	}
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
	SameFrame {
		frame_type: u8,
		offset_delta: u16,
	},
	SameLocals1StackItemFrame {
		frame_type: u8,
		offset_delta: u16,
		stack: VerificationTypeInfo,
	},
	SameLocals1StackItemFrameExtended {
		frame_type: u8,
		offset_delta: u16,
		stack: VerificationTypeInfo,
	},
	/*
	   The frame type chop_frame is represented by tags in the range [248-250]. If the frame_type is chop_frame,-
	   it means that the operand stack is empty and the current locals are the same as the locals in the previous frame,-
	   except that the k last locals are absent. The value of k is given by the formula 251 - frame_type.
	*/
	// TODO: do we store `k` for convenience? wtf is this shit
	ChopFrame {
		frame_type: u8,
		offset_delta: u16,
	},
	SameFrameExtended {
		frame_type: u8,
		offset_delta: u16,
	},
	AppendFrame {
		frame_type: u8,
		offset_delta: u16,
		locals: Vec<VerificationTypeInfo>,
	},
	FullFrame {
		frame_type: u8,
		offset_delta: u16,
		// number_of_locals: u16,
		locals: Vec<VerificationTypeInfo>,
		// verification_type_info locals[number_of_locals];
		// number_of_stack_items: u16,
		stack: Vec<VerificationTypeInfo>,
		// verification_type_info stack[number_of_stack_items];
	},
}

impl StackMapFrame {
	pub fn new<B: BytesReadExt>(attribute_data: &mut B) -> Result<Self, IRClassfileError> {
		let frame_type = attribute_data.read_u8()?;
		Ok(match frame_type {
			0..=63 => Self::SameFrame {
				frame_type,
				offset_delta: frame_type as u16,
			},
			64..=127 => Self::SameLocals1StackItemFrame {
				frame_type,
				offset_delta: (64 - frame_type) as u16,
				stack: VerificationTypeInfo::read(attribute_data)?,
			},
			247 => Self::SameLocals1StackItemFrameExtended {
				frame_type,
				offset_delta: attribute_data.read_u16()?,
				stack: VerificationTypeInfo::read(attribute_data)?,
			},
			248..=250 => Self::ChopFrame {
				frame_type,
				offset_delta: attribute_data.read_u16()?,
			},
			251 => Self::SameFrameExtended {
				frame_type,
				offset_delta: attribute_data.read_u16()?,
			},
			252..=254 => {
				let offset_delta = attribute_data.read_u16()?;

				let n_locals = (frame_type - 251) as usize;
				let mut locals = Vec::with_capacity(n_locals);
				for _ in 0..n_locals {
					locals.push(VerificationTypeInfo::read(attribute_data)?);
				}

				Self::AppendFrame {
					frame_type,
					offset_delta,
					locals,
				}
			}
			255 => {
				let offset_delta = attribute_data.read_u16()?;

				let n_locals = attribute_data.read_u16()? as usize;
				let mut locals = Vec::with_capacity(n_locals);
				for _ in 0..n_locals {
					locals.push(VerificationTypeInfo::read(attribute_data)?);
				}

				let n_stack = attribute_data.read_u16()? as usize;
				let mut stack = Vec::with_capacity(n_stack);
				for _ in 0..n_stack {
					stack.push(VerificationTypeInfo::read(attribute_data)?);
				}

				Self::FullFrame {
					frame_type,
					offset_delta,
					locals,
					stack,
				}
			}

			_ => panic!("invalid frame tag {frame_type}"),
		})
	}
}

#[derive(Debug, Clone)]
pub struct InnerClassesAttributeClass {
	pub inner_class_info: CPClassRef,
	pub outer_class_info: Option<CPClassRef>,
	pub inner_name: Option<CPUtf8Ref>,
	pub inner_class_access_flags: u16,
}

impl InnerClassesAttributeClass {
	pub fn new<B: BytesReadExt>(buffer: &mut B) -> Result<Self, IRClassfileError> {
		todo!("InnerClassesAttributeClass::new")
	}
}

#[derive(Debug, Clone)]
pub struct InnerClassesAttribute {}

#[derive(Debug, Clone)]
pub struct CodeAttributeException {
	pub start_pc: u16,
	pub end_pc: u16,
	pub handler_pc: u16,
	pub catch_type: u16,
}

impl CodeAttributeException {
	fn new<B: BytesReadExt>(buffer: &mut B) -> Result<Self, IRClassfileError> {
		Ok(Self {
			start_pc: buffer.read_u16()?,
			end_pc: buffer.read_u16()?,
			handler_pc: buffer.read_u16()?,
			catch_type: buffer.read_u16()?,
		})
	}
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
	pub max_stack: u16,
	pub max_locals: u16,
	pub code: Vec<u8>,
	pub exception_table: Vec<CodeAttributeException>,
	pub attributes: Vec<Box<IRAttributeInfo>>,
}

impl CodeAttribute {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let max_stack = buffer.read_u16()?;
		let max_locals = buffer.read_u16()?;
		let code_len = buffer.read_u32()? as usize;
		let mut code = Vec::with_capacity(code_len);
		for _ in 0..code_len {
			code.push(buffer.read_u8()?);
		}

		let exception_table_len = buffer.read_u16()? as usize;
		let mut exception_table = Vec::with_capacity(exception_table_len);
		for _ in 0..exception_table_len {
			exception_table.push(CodeAttributeException::new(buffer)?);
		}

		let attribute_len = buffer.read_u16()? as usize;
		let mut attributes = Vec::with_capacity(attribute_len);
		for _ in 0..attribute_len {
			attributes.push(Box::new(IRAttributeInfo::from_io(cp, IOAttributeInfo::read(buffer)?)?));
		}
		Ok(Self {
			max_stack,
			max_locals,
			code,
			exception_table,
			attributes,
		})
	}
}

#[derive(Debug, Clone)]
pub struct IRAttributeInfo {
	pub name: CPUtf8Ref,
	pub length: u32,
	pub attr: IRAttribute,
}

impl IRAttributeInfo {
	pub fn from_io(cp: &[IRCpTag], raw: IOAttributeInfo) -> Result<Self, IRClassfileError> {
		let name = CPUtf8Ref::new(
			raw.attribute_name_index,
			cp.get(raw.attribute_name_index as usize - 1).expect("invalid index"),
		);

		let mut buffer = Cursor::new(raw.info);
		Ok(Self {
			length: raw.attribute_length,
			attr: IRAttribute::new(name.clone(), cp, &mut buffer)?,
			name,
		})
	}
}

#[derive(Debug, Clone)]
pub enum IRAttribute {
	ConstantValue(ConstantValueAttribute),
	Code(CodeAttribute),
	StackMapTable(StackMapTableAttribute),
	Exceptions {
		exception_index_table: Vec<CPUtf8Ref>,
	},
	InnerClasses(InnerClassesAttribute),
	EnclosingMethod {
		class_idx: u16,
		method: CPNameAndTypeRef,
	},
	Synthetic,
	Signature(CPUtf8Ref),
	SourceFile(CPUtf8Ref),
	SourceDebugExtension(
		/*TODO: What to put here? Maybe just a String? https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.11 */
	),
	LineNumberTable,
	LocalVariableTable,
	LocalVariableTypeTable,
	Deprecated,
	RuntimeVisibleAnnotations,
	RuntimeInvisibleAnnotations,
	RuntimeVisibleParameterAnnotations,
	RuntimeInvisibleParameterAnnotations,
	AnnotationDefault,
	BootstrapMethods,
}

impl IRAttribute {
	pub fn new<B: BytesReadExt>(name: CPUtf8Ref, cp: &[IRCpTag], data: &mut B) -> Result<Self, IRClassfileError> {
		Ok(match name.data.as_str() {
			"ConstantValue" => {
				let cp_idx = data.read_u16()?;
				let tag = cp.get(cp_idx as usize - 1).expect("invalid index fuck u");
				match tag {
					IRCpTag::Integer(value) => {
						Self::ConstantValue(ConstantValueAttribute::Int { cp_idx, value: *value })
					}
					IRCpTag::Float(value) => {
						Self::ConstantValue(ConstantValueAttribute::Float { cp_idx, value: *value })
					}
					IRCpTag::Long(value) => Self::ConstantValue(ConstantValueAttribute::Long { cp_idx, value: *value }),
					IRCpTag::Double(value) => {
						Self::ConstantValue(ConstantValueAttribute::Double { cp_idx, value: *value })
					}
					IRCpTag::String(value) => Self::ConstantValue(ConstantValueAttribute::String(value.clone())),
					_ => panic!("didnt expect tag: {tag:?}"),
				}
			}

			"Code" => Self::Code(CodeAttribute::new(cp, data)?),

			"StackMapTable" => {
				let n_entries = data.read_u16()? as usize;
				let mut entries = Vec::with_capacity(n_entries);

				for _ in 0..n_entries {
					entries.push(StackMapFrame::new(data)?);
				}

				Self::StackMapTable(StackMapTableAttribute { entries })
			}

			"Exceptions" => {
				let n_exceptions = data.read_u16()? as usize;
				let mut exception_index_table = Vec::with_capacity(n_exceptions);

				for _ in 0..n_exceptions {
					let idx = data.read_u16()?;
					exception_index_table.push(CPUtf8Ref::new(idx, cp.get(idx as usize).expect("expected utf8")));
				}

				Self::Exceptions { exception_index_table }
			}

			"LineNumberTable" => {
				todo!("LineNumberTable")
			}

			n => panic!("unparsed attribute: {n}"),
		})
	}

	pub const fn name(&self) -> &'static str {
		match self {
			Self::ConstantValue(_) => "ConstantValue",
			Self::Code(_) => "Code",
			Self::StackMapTable(_) => "StackMapTable",
			Self::Exceptions {
				exception_index_table: _,
			} => "Exceptions",
			Self::InnerClasses(_) => "InnerClasses",
			Self::EnclosingMethod {
				class_idx: _,
				method: _,
			} => "EnclosingMethod",
			Self::Synthetic => "Synthetic",
			Self::Signature(_) => "Signature",
			Self::SourceFile(_) => "SourceFile",
			Self::SourceDebugExtension() => "SourceDebugExtension",
			Self::LineNumberTable => "LineNumberTable",
			Self::LocalVariableTable => "LocalVariableTable",
			Self::LocalVariableTypeTable => "LocalVariableTypeTable",
			Self::Deprecated => "Deprecated",
			Self::RuntimeVisibleAnnotations => "RuntimeVisibleAnnotations",
			Self::RuntimeInvisibleAnnotations => "RuntimeInvisibleAnnotations",
			Self::RuntimeVisibleParameterAnnotations => "RuntimeVisibleParameterAnnotations",
			Self::RuntimeInvisibleParameterAnnotations => "RuntimeInvisibleParameterAnnotations",
			Self::AnnotationDefault => "AnnotationDefault",
			Self::BootstrapMethods => "BootstrapMethods",
		}
	}
}