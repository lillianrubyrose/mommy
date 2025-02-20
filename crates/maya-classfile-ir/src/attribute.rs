use std::{io::Cursor, rc::Rc};

use maya_bytes::BytesReadExt;
use maya_classfile_io::IOAttributeInfo;

use crate::class_pool::{
	CPClassRef, CPConstValueRef, CPMethodHandleRef, CPModuleInfoRef, CPNameAndTypeRef, CPPackageInfoRef, CPTagRef,
	CPUtf8Ref, IRClassfileError, IRCpTag,
};

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
				offset_delta: (frame_type - 64) as u16,
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
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let inner_info_idx = buffer.read_u16()?;
		let outer_info_idx = buffer.read_u16()?;
		let inner_name_idx = buffer.read_u16()?;
		let inner_class_access_flags = buffer.read_u16()?;

		let inner_info_tag = cp.get(inner_info_idx as usize - 1).expect("expected class");
		let outer_info_tag = if outer_info_idx == 0 {
			None
		} else {
			Some(cp.get(outer_info_idx as usize - 1).expect("expected class"))
		};
		let inner_name_tag = if inner_name_idx == 0 {
			None
		} else {
			Some(cp.get(inner_name_idx as usize - 1).expect("expected utf8"))
		};

		Ok(Self {
			inner_class_info: CPClassRef::new(inner_info_idx, inner_info_tag),
			outer_class_info: outer_info_tag.map(|tag| CPClassRef::new(outer_info_idx, tag)),
			inner_name: inner_name_tag.map(|tag| CPUtf8Ref::new(inner_name_idx, tag)),
			inner_class_access_flags,
		})
	}
}

#[derive(Debug, Clone)]
pub struct InnerClassesAttribute {
	pub classes: Vec<InnerClassesAttributeClass>,
}

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
pub struct LineNumberTableAttributeEntry {
	pub start_pc: u16,
	pub line_number: u16,
}

#[derive(Debug, Clone)]
pub struct LineNumberTableAttribute {
	pub line_number_table: Vec<LineNumberTableAttributeEntry>,
}

impl LineNumberTableAttribute {
	pub fn new<B: BytesReadExt>(buffer: &mut B) -> Result<Self, IRClassfileError> {
		let table_len = buffer.read_u16()? as usize;
		let mut line_number_table = Vec::with_capacity(table_len);

		for _ in 0..table_len {
			line_number_table.push(LineNumberTableAttributeEntry {
				start_pc: buffer.read_u16()?,
				line_number: buffer.read_u16()?,
			});
		}

		Ok(Self { line_number_table })
	}
}

#[derive(Debug, Clone)]
pub struct MethodParametersParam {
	pub name: Option<CPUtf8Ref>,
	pub access_flags: u16,
}

impl MethodParametersParam {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let name_index = buffer.read_u16()?;

		Ok(Self {
			name: if name_index == 0 {
				None
			} else {
				Some(CPUtf8Ref::new(
					name_index,
					cp.get(name_index as usize - 1).expect("expected utf8"),
				))
			},
			access_flags: buffer.read_u16()?,
		})
	}
}

#[derive(Debug, Clone)]
pub enum RuntimeAnnotationValue {
	ConstValueIndex(CPConstValueRef),
	EnumConstValue {
		type_name: CPUtf8Ref,
		const_name: CPUtf8Ref,
	},
	ClassInfoIndex(CPUtf8Ref),
	Annotation(Box<RuntimeAnnotation>),
	ArrayValue {
		values: Vec<RuntimeAnnotationValue>,
	},
}

impl RuntimeAnnotationValue {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let tag = buffer.read_u8()?;
		Ok(match tag {
			b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
				Self::ConstValueIndex(CPConstValueRef::from_cp(cp, buffer.read_u16()?))
			}

			b'e' => Self::EnumConstValue {
				type_name: CPUtf8Ref::from_cp(cp, buffer.read_u16()?),
				const_name: CPUtf8Ref::from_cp(cp, buffer.read_u16()?),
			},

			b'c' => Self::ClassInfoIndex(CPUtf8Ref::from_cp(cp, buffer.read_u16()?)),
			b'@' => Self::Annotation(Box::new(RuntimeAnnotation::new(cp, buffer)?)),
			b'[' => {
				let n_values = buffer.read_u16()? as usize;
				let mut values = Vec::with_capacity(n_values);

				for _ in 0..n_values {
					values.push(RuntimeAnnotationValue::new(cp, buffer)?);
				}

				Self::ArrayValue { values }
			}
			_ => panic!("invalid tag: {tag}"),
		})
	}
}

#[derive(Debug, Clone)]
pub struct RuntimeAnnotationEVPair {
	pub name: CPUtf8Ref,
	pub value: RuntimeAnnotationValue,
}

#[derive(Debug, Clone)]
pub struct RuntimeAnnotation {
	pub ty: CPUtf8Ref,
	pub pairs: Vec<RuntimeAnnotationEVPair>,
}

impl RuntimeAnnotation {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let ty_idx = buffer.read_u16()?;
		let ty = CPUtf8Ref::new(ty_idx, cp.get(ty_idx as usize - 1).expect("expected utf8"));

		let n_pairs = buffer.read_u16()? as usize;
		let mut pairs = Vec::with_capacity(n_pairs);

		for _ in 0..n_pairs {
			let name_idx = buffer.read_u16()?;
			let name = CPUtf8Ref::new(name_idx, cp.get(name_idx as usize - 1).expect("expected utf8"));

			pairs.push(RuntimeAnnotationEVPair {
				name,
				value: RuntimeAnnotationValue::new(cp, buffer)?,
			});
		}

		Ok(Self { ty, pairs })
	}
}

#[derive(Debug, Clone)]
pub struct RecordComponentInfo {
	pub name: CPUtf8Ref,
	pub descriptor: CPUtf8Ref,
	pub attributes: Vec<IRAttributeInfo>,
}

impl RecordComponentInfo {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let name_idx = buffer.read_u16()?;
		let descriptor_idx = buffer.read_u16()?;
		let n_attributes = buffer.read_u16()? as usize;
		let mut attributes = Vec::with_capacity(n_attributes);
		for _ in 0..n_attributes {
			attributes.push(IRAttributeInfo::from_io(cp, IOAttributeInfo::read(buffer)?)?);
		}

		Ok(Self {
			name: CPUtf8Ref::from_cp(cp, name_idx),
			descriptor: CPUtf8Ref::from_cp(cp, descriptor_idx),
			attributes,
		})
	}
}

#[derive(Debug, Clone)]
pub struct BootstrapMethodsMethod {
	pub method: CPMethodHandleRef,
	pub arguments: Vec<CPTagRef>,
}

impl BootstrapMethodsMethod {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let method_idx = buffer.read_u16()?;
		let n_args = buffer.read_u16()? as usize;
		let mut arguments = Vec::with_capacity(n_args);

		for _ in 0..n_args {
			arguments.push(buffer.read_u16()?);
		}

		Ok(Self {
			method: CPMethodHandleRef::from_cp(cp, method_idx),
			arguments: arguments.into_iter().map(|idx| CPTagRef::from_cp(cp, idx)).collect(),
		})
	}
}

#[derive(Debug, Clone)]
pub struct LocalVariableTableEntry {
	pub start_pc: u16,
	pub length: u16,
	pub name: CPUtf8Ref,
	pub descriptor: CPUtf8Ref,
	pub index: u16,
}

impl LocalVariableTableEntry {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let start_pc = buffer.read_u16()?;
		let length = buffer.read_u16()?;
		let name_idx = buffer.read_u16()?;
		let descriptor_idx = buffer.read_u16()?;
		let index = buffer.read_u16()?;

		Ok(Self {
			start_pc,
			length,
			name: CPUtf8Ref::from_cp(cp, name_idx),
			descriptor: CPUtf8Ref::from_cp(cp, descriptor_idx),
			index,
		})
	}
}

#[derive(Debug, Clone)]
pub struct LocalVariableTypeTableEntry {
	pub start_pc: u16,
	pub length: u16,
	pub name: CPUtf8Ref,
	pub signature: CPUtf8Ref,
	pub index: u16,
}

impl LocalVariableTypeTableEntry {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let start_pc = buffer.read_u16()?;
		let length = buffer.read_u16()?;
		let name_idx = buffer.read_u16()?;
		let signature_idx = buffer.read_u16()?;
		let index = buffer.read_u16()?;

		Ok(Self {
			start_pc,
			length,
			name: CPUtf8Ref::from_cp(cp, name_idx),
			signature: CPUtf8Ref::from_cp(cp, signature_idx),
			index,
		})
	}
}

#[derive(Debug, Clone)]
pub struct RuntimeTypeAnnotationLocalVarTargetTableEntry {
	pub start_pc: u16,
	pub length: u16,
	pub index: u16,
}

#[derive(Debug, Clone)]
pub enum RuntimeTypeAnnotationTargetInfo {
	TypeParameterTarget {
		type_param_index: u8,
	},
	SupertypeTarget {
		supertype_index: u16,
	},
	TypeParameterBoundTarget {
		type_param_index: u8,
		bound_index: u8,
	},
	EmptyTarget,
	FormalParameterTarget {
		formal_param_index: u8,
	},
	ThrowsTarget {
		throws_type_index: u16,
	},
	LocalvarTarget {
		table: Vec<RuntimeTypeAnnotationLocalVarTargetTableEntry>,
	},
	CatchTarget {
		exception_table_index: u16,
	},
	OffsetTarget {
		offset: u16,
	},
	TypeArgumentTarget {
		offset: u16,
		type_argument_index: u8,
	},
}

#[derive(Debug, Clone)]
pub struct RuntimeTypeAnnotationTypePathPart {
	// TODO: Parse this into Enum of some sort? Maybe.
	// see: https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.20
	pub type_path_kind: u8,
	pub type_argument_kind: u8,
}

#[derive(Debug, Clone)]
pub struct RuntimeTypeAnnotation {
	pub target_type: u8,
	pub target_info: RuntimeTypeAnnotationTargetInfo,
	pub target_path: Vec<RuntimeTypeAnnotationTypePathPart>,
	pub type_index: u16,
	pub pairs: Vec<RuntimeAnnotationEVPair>,
}

impl RuntimeTypeAnnotation {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let target_type = buffer.read_u8()?;
		let target_info = match target_type {
			// 4.7.20-A
			0x0 | 0x01 => RuntimeTypeAnnotationTargetInfo::TypeParameterTarget {
				type_param_index: buffer.read_u8()?,
			},
			0x10 => RuntimeTypeAnnotationTargetInfo::SupertypeTarget {
				supertype_index: buffer.read_u16()?,
			},
			0x11 | 0x12 => RuntimeTypeAnnotationTargetInfo::TypeParameterBoundTarget {
				type_param_index: buffer.read_u8()?,
				bound_index: buffer.read_u8()?,
			},
			0x13..=0x15 => RuntimeTypeAnnotationTargetInfo::EmptyTarget,
			0x16 => RuntimeTypeAnnotationTargetInfo::FormalParameterTarget {
				formal_param_index: buffer.read_u8()?,
			},
			0x17 => RuntimeTypeAnnotationTargetInfo::ThrowsTarget {
				throws_type_index: buffer.read_u16()?,
			},

			// 4.7.20-B
			0x40 | 0x41 => {
				let n_entries = buffer.read_u16()? as usize;
				let mut table = Vec::with_capacity(n_entries);

				for _ in 0..n_entries {
					table.push(RuntimeTypeAnnotationLocalVarTargetTableEntry {
						start_pc: buffer.read_u16()?,
						length: buffer.read_u16()?,
						index: buffer.read_u16()?,
					});
				}

				RuntimeTypeAnnotationTargetInfo::LocalvarTarget { table }
			}
			0x42 => RuntimeTypeAnnotationTargetInfo::CatchTarget {
				exception_table_index: buffer.read_u16()?,
			},
			0x43..=0x45 => RuntimeTypeAnnotationTargetInfo::OffsetTarget {
				offset: buffer.read_u16()?,
			},
			0x47..=0x4B => RuntimeTypeAnnotationTargetInfo::TypeArgumentTarget {
				offset: buffer.read_u16()?,
				type_argument_index: buffer.read_u8()?,
			},

			_ => panic!("unexpected target_type: {target_type}"),
		};

		let n_parts = buffer.read_u8()? as usize;
		let mut target_path = Vec::with_capacity(n_parts);
		for _ in 0..n_parts {
			target_path.push(RuntimeTypeAnnotationTypePathPart {
				type_path_kind: buffer.read_u8()?,
				type_argument_kind: buffer.read_u8()?,
			});
		}

		let type_index = buffer.read_u16()?;

		let n_pairs = buffer.read_u16()? as usize;
		let mut pairs = Vec::with_capacity(n_pairs);

		for _ in 0..n_pairs {
			let name_idx = buffer.read_u16()?;
			let name = CPUtf8Ref::new(name_idx, cp.get(name_idx as usize - 1).expect("expected utf8"));

			pairs.push(RuntimeAnnotationEVPair {
				name,
				value: RuntimeAnnotationValue::new(cp, buffer)?,
			});
		}

		Ok(Self {
			target_type,
			target_info,
			target_path,
			type_index,
			pairs,
		})
	}
}

#[derive(Debug, Clone)]
pub struct ModuleRequiresEntry {
	pub module: CPModuleInfoRef,
	pub flags: u16,
	pub version: Option<CPUtf8Ref>,
}

impl ModuleRequiresEntry {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let module_idx = buffer.read_u16()?;
		let flags = buffer.read_u16()?;
		let version_idx = buffer.read_u16()?;

		Ok(Self {
			module: CPModuleInfoRef::from_cp(cp, module_idx),
			flags,
			version: if version_idx == 0 {
				None
			} else {
				Some(CPUtf8Ref::from_cp(cp, version_idx))
			},
		})
	}
}

#[derive(Debug, Clone)]
pub struct ModuleExportsEntry {
	pub package: CPPackageInfoRef,
	pub flags: u16,
	pub exports: Vec<CPModuleInfoRef>,
}

impl ModuleExportsEntry {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let package_idx = buffer.read_u16()?;
		let flags = buffer.read_u16()?;

		let n_exports = buffer.read_u16()? as usize;
		let mut exports = Vec::with_capacity(n_exports);

		for _ in 0..n_exports {
			exports.push(CPModuleInfoRef::from_cp(cp, buffer.read_u16()?));
		}

		Ok(Self {
			package: CPPackageInfoRef::from_cp(cp, package_idx),
			flags,
			exports,
		})
	}
}

#[derive(Debug, Clone)]
pub struct ModuleOpensEntry {
	pub package: CPPackageInfoRef,
	pub flags: u16,
	pub opens: Vec<CPModuleInfoRef>,
}

impl ModuleOpensEntry {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let package_idx = buffer.read_u16()?;
		let flags = buffer.read_u16()?;

		let n_opens = buffer.read_u16()? as usize;
		let mut opens = Vec::with_capacity(n_opens);

		for _ in 0..n_opens {
			opens.push(CPModuleInfoRef::from_cp(cp, buffer.read_u16()?));
		}

		Ok(Self {
			package: CPPackageInfoRef::from_cp(cp, package_idx),
			flags,
			opens,
		})
	}
}

#[derive(Debug, Clone)]
pub struct ModuleProvidesEntry {
	pub class: CPClassRef,
	pub flags: u16,
	pub provides: Vec<CPClassRef>,
}

impl ModuleProvidesEntry {
	pub fn new<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		let package_idx = buffer.read_u16()?;
		let flags = buffer.read_u16()?;

		let n_exports = buffer.read_u16()? as usize;
		let mut provides = Vec::with_capacity(n_exports);

		for _ in 0..n_exports {
			provides.push(CPClassRef::from_cp(cp, buffer.read_u16()?));
		}

		Ok(Self {
			class: CPClassRef::from_cp(cp, package_idx),
			flags,
			provides,
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
		class: CPClassRef,
		method: Option<CPNameAndTypeRef>,
	},
	Synthetic,
	Signature(CPUtf8Ref),
	SourceFile(CPUtf8Ref),
	SourceDebugExtension(Rc<String>),
	LineNumberTable(LineNumberTableAttribute),
	LocalVariableTable {
		table: Vec<LocalVariableTableEntry>,
	},
	LocalVariableTypeTable {
		table: Vec<LocalVariableTypeTableEntry>,
	},
	Deprecated,
	RuntimeVisibleAnnotations {
		annotations: Vec<RuntimeAnnotation>,
	},
	RuntimeInvisibleAnnotations {
		annotations: Vec<RuntimeAnnotation>,
	},
	RuntimeVisibleParameterAnnotations {
		params: Vec<Vec<RuntimeAnnotation>>,
	},
	RuntimeInvisibleParameterAnnotations {
		params: Vec<Vec<RuntimeAnnotation>>,
	},
	AnnotationDefault {
		default_value: RuntimeAnnotationValue,
	},
	BootstrapMethods {
		methods: Vec<BootstrapMethodsMethod>,
	},
	NestMembers {
		classes: Vec<CPClassRef>,
	},
	NestHost(CPClassRef),
	MethodParameters {
		parameters: Vec<MethodParametersParam>,
	},
	Record {
		components: Vec<RecordComponentInfo>,
	},
	PermittedSubclasses {
		classes: Vec<CPClassRef>,
	},
	RuntimeVisibleTypeAnnotations {
		annotations: Vec<RuntimeTypeAnnotation>,
	},
	RuntimeInvisibleTypeAnnotations {
		annotations: Vec<RuntimeTypeAnnotation>,
	},
	Module {
		module_name: CPModuleInfoRef,
		module_flags: u16,
		module_version: Option<CPUtf8Ref>,

		requires: Vec<ModuleRequiresEntry>,
		exports: Vec<ModuleExportsEntry>,
		opens: Vec<ModuleOpensEntry>,

		uses: Vec<CPClassRef>,

		provides: Vec<ModuleProvidesEntry>,
	},
	ModulePackages {
		packages: Vec<CPPackageInfoRef>,
	},
	ModuleMainClass {
		class: CPClassRef,
	},
}

impl IRAttribute {
	pub fn new<B: BytesReadExt>(name: CPUtf8Ref, cp: &[IRCpTag], buffer: &mut B) -> Result<Self, IRClassfileError> {
		Ok(match name.data.as_str() {
			"ConstantValue" => {
				let cp_idx = buffer.read_u16()?;
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

			"Code" => Self::Code(CodeAttribute::new(cp, buffer)?),

			"StackMapTable" => {
				let n_entries = buffer.read_u16()? as usize;
				let mut entries = Vec::with_capacity(n_entries);

				for _ in 0..n_entries {
					entries.push(StackMapFrame::new(buffer)?);
				}

				Self::StackMapTable(StackMapTableAttribute { entries })
			}

			"Exceptions" => {
				let n_exceptions = buffer.read_u16()? as usize;
				let mut exception_index_table = Vec::with_capacity(n_exceptions);

				for _ in 0..n_exceptions {
					let idx = buffer.read_u16()?;
					exception_index_table.push(CPUtf8Ref::new(idx, cp.get(idx as usize).expect("expected utf8")));
				}

				Self::Exceptions { exception_index_table }
			}

			"LineNumberTable" => Self::LineNumberTable(LineNumberTableAttribute::new(buffer)?),
			"SourceFile" => {
				let index = buffer.read_u16()?;
				let tag = CPUtf8Ref::new(index, cp.get(index as usize - 1).expect("expected utf8"));
				Self::SourceFile(tag)
			}
			"NestMembers" => {
				let n_classes = buffer.read_u16()? as usize;
				let mut classes = Vec::with_capacity(n_classes);

				for _ in 0..n_classes {
					let index = buffer.read_u16()?;
					let tag = cp.get(index as usize - 1).expect("expected class");
					classes.push(CPClassRef::new(index, tag));
				}

				Self::NestMembers { classes }
			}
			"InnerClasses" => {
				let n_classes = buffer.read_u16()? as usize;
				let mut classes = Vec::with_capacity(n_classes);

				for _ in 0..n_classes {
					classes.push(InnerClassesAttributeClass::new(cp, buffer)?);
				}

				Self::InnerClasses(InnerClassesAttribute { classes })
			}
			"Synthetic" => Self::Synthetic,
			"Signature" => {
				let idx = buffer.read_u16()?;
				Self::Signature(CPUtf8Ref::new(idx, cp.get(idx as usize - 1).expect("expected utf8")))
			}
			"NestHost" => {
				let idx = buffer.read_u16()?;
				Self::NestHost(CPClassRef::new(idx, cp.get(idx as usize - 1).expect("expected class")))
			}
			"MethodParameters" => {
				let n_params = buffer.read_u8()? as usize;
				let mut parameters = Vec::with_capacity(n_params);

				for _ in 0..n_params {
					parameters.push(MethodParametersParam::new(cp, buffer)?);
				}

				Self::MethodParameters { parameters }
			}
			"Deprecated" => Self::Deprecated,
			"RuntimeVisibleAnnotations" => {
				let n_annotations = buffer.read_u16()? as usize;
				let mut annotations = Vec::with_capacity(n_annotations);

				for _ in 0..n_annotations {
					annotations.push(RuntimeAnnotation::new(cp, buffer)?);
				}

				Self::RuntimeVisibleAnnotations { annotations }
			}
			"RuntimeInvisibleAnnotations" => {
				let n_annotations = buffer.read_u16()? as usize;
				let mut annotations = Vec::with_capacity(n_annotations);

				for _ in 0..n_annotations {
					annotations.push(RuntimeAnnotation::new(cp, buffer)?);
				}

				Self::RuntimeInvisibleAnnotations { annotations }
			}
			"RuntimeVisibleParameterAnnotations" => {
				let n_params = buffer.read_u8()? as usize;
				let mut params = Vec::with_capacity(n_params);

				for _ in 0..n_params {
					let n_annotations = buffer.read_u16()? as usize;
					let mut annotations = Vec::with_capacity(n_annotations);

					for _ in 0..n_annotations {
						annotations.push(RuntimeAnnotation::new(cp, buffer)?);
					}

					params.push(annotations);
				}

				Self::RuntimeVisibleParameterAnnotations { params }
			}
			"RuntimeInvisibleParameterAnnotations" => {
				let n_params = buffer.read_u8()? as usize;
				let mut params = Vec::with_capacity(n_params);

				for _ in 0..n_params {
					let n_annotations = buffer.read_u16()? as usize;
					let mut annotations = Vec::with_capacity(n_annotations);

					for _ in 0..n_annotations {
						annotations.push(RuntimeAnnotation::new(cp, buffer)?);
					}

					params.push(annotations);
				}

				Self::RuntimeInvisibleParameterAnnotations { params }
			}
			"Record" => {
				let n_components = buffer.read_u16()? as usize;
				let mut components = Vec::with_capacity(n_components);

				for _ in 0..n_components {
					components.push(RecordComponentInfo::new(cp, buffer)?);
				}

				Self::Record { components }
			}
			"BootstrapMethods" => {
				let n_methods = buffer.read_u16()? as usize;
				let mut methods = Vec::with_capacity(n_methods);

				for _ in 0..n_methods {
					methods.push(BootstrapMethodsMethod::new(cp, buffer)?);
				}

				Self::BootstrapMethods { methods }
			}
			"PermittedSubclasses" => {
				let n_classes = buffer.read_u16()? as usize;
				let mut classes = Vec::with_capacity(n_classes);

				for _ in 0..n_classes {
					classes.push(CPClassRef::from_cp(cp, buffer.read_u16()?));
				}

				Self::PermittedSubclasses { classes }
			}
			"SourceDebugExtension" => Self::SourceDebugExtension(Rc::new(String::from_utf8(buffer.read_to_vec()?)?)),
			"LocalVariableTable" => {
				let n_entries = buffer.read_u16()? as usize;
				let mut table = Vec::with_capacity(n_entries);

				for _ in 0..n_entries {
					table.push(LocalVariableTableEntry::new(cp, buffer)?);
				}

				Self::LocalVariableTable { table }
			}
			"LocalVariableTypeTable" => {
				let n_entries = buffer.read_u16()? as usize;
				let mut table = Vec::with_capacity(n_entries);

				for _ in 0..n_entries {
					table.push(LocalVariableTypeTableEntry::new(cp, buffer)?);
				}

				Self::LocalVariableTypeTable { table }
			}
			"EnclosingMethod" => {
				let class_idx = buffer.read_u16()?;
				let method_idx = buffer.read_u16()?;
				Self::EnclosingMethod {
					class: CPClassRef::from_cp(cp, class_idx),
					method: if method_idx == 0 {
						None
					} else {
						Some(CPNameAndTypeRef::from_cp(cp, method_idx))
					},
				}
			}
			"RuntimeVisibleTypeAnnotations" => {
				let n_annotations = buffer.read_u16()? as usize;
				let mut annotations = Vec::with_capacity(n_annotations);

				for _ in 0..n_annotations {
					annotations.push(RuntimeTypeAnnotation::new(cp, buffer)?);
				}

				Self::RuntimeVisibleTypeAnnotations { annotations }
			}
			"RuntimeInvisibleTypeAnnotations" => {
				let n_annotations = buffer.read_u16()? as usize;
				let mut annotations = Vec::with_capacity(n_annotations);

				for _ in 0..n_annotations {
					annotations.push(RuntimeTypeAnnotation::new(cp, buffer)?);
				}

				Self::RuntimeInvisibleTypeAnnotations { annotations }
			}
			"AnnotationDefault" => Self::AnnotationDefault {
				default_value: RuntimeAnnotationValue::new(cp, buffer)?,
			},
			"Module" => {
				let module_name_idx = buffer.read_u16()?;
				let module_flags = buffer.read_u16()?;
				let module_version_idx = buffer.read_u16()?;

				let n_requires = buffer.read_u16()? as usize;
				let mut requires = Vec::with_capacity(n_requires);
				for _ in 0..n_requires {
					requires.push(ModuleRequiresEntry::new(cp, buffer)?);
				}

				let n_exports = buffer.read_u16()? as usize;
				let mut exports = Vec::with_capacity(n_exports);
				for _ in 0..n_exports {
					exports.push(ModuleExportsEntry::new(cp, buffer)?);
				}

				let n_opens = buffer.read_u16()? as usize;
				let mut opens = Vec::with_capacity(n_opens);
				for _ in 0..n_opens {
					opens.push(ModuleOpensEntry::new(cp, buffer)?);
				}

				let n_uses = buffer.read_u16()? as usize;
				let mut uses = Vec::with_capacity(n_uses);
				for _ in 0..n_uses {
					uses.push(CPClassRef::from_cp(cp, buffer.read_u16()?));
				}

				let n_provides = buffer.read_u16()? as usize;
				let mut provides = Vec::with_capacity(n_provides);
				for _ in 0..n_provides {
					provides.push(ModuleProvidesEntry::new(cp, buffer)?);
				}

				Self::Module {
					module_name: CPModuleInfoRef::from_cp(cp, module_name_idx),
					module_flags,
					module_version: if module_version_idx == 0 {
						None
					} else {
						Some(CPUtf8Ref::from_cp(cp, module_version_idx))
					},
					requires,
					exports,
					opens,
					uses,
					provides,
				}
			}
			"ModulePackages" => {
				let n_packages = buffer.read_u16()? as usize;
				let mut packages = Vec::with_capacity(n_packages);
				for _ in 0..n_packages {
					packages.push(CPPackageInfoRef::from_cp(cp, buffer.read_u16()?));
				}
				Self::ModulePackages { packages }
			}
			"ModuleMainClass" => Self::ModuleMainClass {
				class: CPClassRef::from_cp(cp, buffer.read_u16()?),
			},

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
			Self::EnclosingMethod { class: _, method: _ } => "EnclosingMethod",
			Self::Synthetic => "Synthetic",
			Self::Signature(_) => "Signature",
			Self::SourceFile(_) => "SourceFile",
			Self::SourceDebugExtension(_) => "SourceDebugExtension",
			Self::LineNumberTable(_) => "LineNumberTable",
			Self::LocalVariableTable { table: _ } => "LocalVariableTable",
			Self::LocalVariableTypeTable { table: _ } => "LocalVariableTypeTable",
			Self::Deprecated => "Deprecated",
			Self::RuntimeVisibleAnnotations { annotations: _ } => "RuntimeVisibleAnnotations",
			Self::RuntimeInvisibleAnnotations { annotations: _ } => "RuntimeInvisibleAnnotations",
			Self::RuntimeVisibleParameterAnnotations { params: _ } => "RuntimeVisibleParameterAnnotations",
			Self::RuntimeInvisibleParameterAnnotations { params: _ } => "RuntimeInvisibleParameterAnnotations",
			Self::AnnotationDefault { default_value: _ } => "AnnotationDefault",
			Self::BootstrapMethods { methods: _ } => "BootstrapMethods",
			Self::NestMembers { classes: _ } => "NestMembers",
			Self::NestHost(_) => "NestHost",
			Self::MethodParameters { parameters: _ } => "MethodParameters",
			Self::Record { components: _ } => "Record",
			Self::PermittedSubclasses { classes: _ } => "PermittedSubclasses",
			Self::RuntimeVisibleTypeAnnotations { annotations: _ } => "RuntimeVisibleTypeAnnotations",
			Self::RuntimeInvisibleTypeAnnotations { annotations: _ } => "RuntimeInvisibleTypeAnnotations",
			Self::Module {
				module_name: _,
				module_flags: _,
				module_version: _,
				requires: _,
				exports: _,
				opens: _,
				uses: _,
				provides: _,
			} => "Module",
			Self::ModulePackages { packages: _ } => "ModulePackages",
			Self::ModuleMainClass { class: _ } => "ModuleMainClass",
		}
	}
}
