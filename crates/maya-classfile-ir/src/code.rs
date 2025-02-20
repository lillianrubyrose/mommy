use maya_bytes::BytesReadExt;

use crate::class_pool::{
	CPClassRef, CPFieldRef, CPInterfaceMethodRef, CPInvokeDynamicRef, CPMethodHandleRef, CPMethodRef, CPUtf8Ref,
	IRClassfileError, IRCpTag,
};

#[allow(non_camel_case_types)]
// https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-6.html
pub struct Opcodes {}

impl Opcodes {
	const NOP: u8 = 0;
	const ACONST_NULL: u8 = 1;
	const ICONST_M1: u8 = 2;
	const ICONST_0: u8 = 3;
	const ICONST_1: u8 = 4;
	const ICONST_2: u8 = 5;
	const ICONST_3: u8 = 6;
	const ICONST_4: u8 = 7;
	const ICONST_5: u8 = 8;
	const LCONST_0: u8 = 9;
	const LCONST_1: u8 = 10;
	const FCONST_0: u8 = 11;
	const FCONST_1: u8 = 12;
	const FCONST_2: u8 = 13;
	const DCONST_0: u8 = 14;
	const DCONST_1: u8 = 15;
	const BIPUSH: u8 = 16;
	const SIPUSH: u8 = 17;
	const LDC: u8 = 18;
	const ILOAD: u8 = 21;
	const LLOAD: u8 = 22;
	const FLOAD: u8 = 23;
	const DLOAD: u8 = 24;
	const ALOAD: u8 = 25;
	const IALOAD: u8 = 46;
	const LALOAD: u8 = 47;
	const FALOAD: u8 = 48;
	const DALOAD: u8 = 49;
	const AALOAD: u8 = 50;
	const BALOAD: u8 = 51;
	const CALOAD: u8 = 52;
	const SALOAD: u8 = 53;
	const ISTORE: u8 = 54;
	const LSTORE: u8 = 55;
	const FSTORE: u8 = 56;
	const DSTORE: u8 = 57;
	const ASTORE: u8 = 58;
	const IASTORE: u8 = 79;
	const LASTORE: u8 = 80;
	const FASTORE: u8 = 81;
	const DASTORE: u8 = 82;
	const AASTORE: u8 = 83;
	const BASTORE: u8 = 84;
	const CASTORE: u8 = 85;
	const SASTORE: u8 = 86;
	const POP: u8 = 87;
	const POP2: u8 = 88;
	const DUP: u8 = 89;
	const DUP_X1: u8 = 90;
	const DUP_X2: u8 = 91;
	const DUP2: u8 = 92;
	const DUP2_X1: u8 = 93;
	const DUP2_X2: u8 = 94;
	const SWAP: u8 = 95;
	const IADD: u8 = 96;
	const LADD: u8 = 97;
	const FADD: u8 = 98;
	const DADD: u8 = 99;
	const ISUB: u8 = 100;
	const LSUB: u8 = 101;
	const FSUB: u8 = 102;
	const DSUB: u8 = 103;
	const IMUL: u8 = 104;
	const LMUL: u8 = 105;
	const FMUL: u8 = 106;
	const DMUL: u8 = 107;
	const IDIV: u8 = 108;
	const LDIV: u8 = 109;
	const FDIV: u8 = 110;
	const DDIV: u8 = 111;
	const IREM: u8 = 112;
	const LREM: u8 = 113;
	const FREM: u8 = 114;
	const DREM: u8 = 115;
	const INEG: u8 = 116;
	const LNEG: u8 = 117;
	const FNEG: u8 = 118;
	const DNEG: u8 = 119;
	const ISHL: u8 = 120;
	const LSHL: u8 = 121;
	const ISHR: u8 = 122;
	const LSHR: u8 = 123;
	const IUSHR: u8 = 124;
	const LUSHR: u8 = 125;
	const IAND: u8 = 126;
	const LAND: u8 = 127;
	const IOR: u8 = 128;
	const LOR: u8 = 129;
	const IXOR: u8 = 130;
	const LXOR: u8 = 131;
	const IINC: u8 = 132;
	const I2L: u8 = 133;
	const I2F: u8 = 134;
	const I2D: u8 = 135;
	const L2I: u8 = 136;
	const L2F: u8 = 137;
	const L2D: u8 = 138;
	const F2I: u8 = 139;
	const F2L: u8 = 140;
	const F2D: u8 = 141;
	const D2I: u8 = 142;
	const D2L: u8 = 143;
	const D2F: u8 = 144;
	const I2B: u8 = 145;
	const I2C: u8 = 146;
	const I2S: u8 = 147;
	const LCMP: u8 = 148;
	const FCMPL: u8 = 149;
	const FCMPG: u8 = 150;
	const DCMPL: u8 = 151;
	const DCMPG: u8 = 152;
	const IFEQ: u8 = 153;
	const IFNE: u8 = 154;
	const IFLT: u8 = 155;
	const IFGE: u8 = 156;
	const IFGT: u8 = 157;
	const IFLE: u8 = 158;
	const IF_ICMPEQ: u8 = 159;
	const IF_ICMPNE: u8 = 160;
	const IF_ICMPLT: u8 = 161;
	const IF_ICMPGE: u8 = 162;
	const IF_ICMPGT: u8 = 163;
	const IF_ICMPLE: u8 = 164;
	const IF_ACMPEQ: u8 = 165;
	const IF_ACMPNE: u8 = 166;
	const GOTO: u8 = 167;
	const JSR: u8 = 168;
	const RET: u8 = 169;
	const TABLESWITCH: u8 = 170;
	const LOOKUPSWITCH: u8 = 171;
	const IRETURN: u8 = 172;
	const LRETURN: u8 = 173;
	const FRETURN: u8 = 174;
	const DRETURN: u8 = 175;
	const ARETURN: u8 = 176;
	const RETURN: u8 = 177;
	const GETSTATIC: u8 = 178;
	const PUTSTATIC: u8 = 179;
	const GETFIELD: u8 = 180;
	const PUTFIELD: u8 = 181;
	const INVOKEVIRTUAL: u8 = 182;
	const INVOKESPECIAL: u8 = 183;
	const INVOKESTATIC: u8 = 184;
	const INVOKEINTERFACE: u8 = 185;
	const INVOKEDYNAMIC: u8 = 186;
	const NEW: u8 = 187;
	const NEWARRAY: u8 = 188;
	const ANEWARRAY: u8 = 189;
	const ARRAYLENGTH: u8 = 190;
	const ATHROW: u8 = 191;
	const CHECKCAST: u8 = 192;
	const INSTANCEOF: u8 = 193;
	const MONITORENTER: u8 = 194;
	const MONITOREXIT: u8 = 195;
	const MULTIANEWARRAY: u8 = 197;
	const IFNULL: u8 = 198;
	const IFNONNULL: u8 = 199;
}

#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
/// An 'Instructions' variant represents an Opcode with the data it contains, if any.
pub enum Instructions {
	NOP = 0,
	ACONST_NULL = 1,
	ICONST_M1 = 2,
	ICONST_0 = 3,
	ICONST_1 = 4,
	ICONST_2 = 5,
	ICONST_3 = 6,
	ICONST_4 = 7,
	ICONST_5 = 8,
	LCONST_0 = 9,
	LCONST_1 = 10,
	FCONST_0 = 11,
	FCONST_1 = 12,
	FCONST_2 = 13,
	DCONST_0 = 14,
	DCONST_1 = 15,
	BIPUSH(u8) = 16,
	SIPUSH(u16) = 17,
	LDC(IRCpTag) = 18,
	ILOAD(u8) = 21,
	LLOAD(u8) = 22,
	FLOAD(u8) = 23,
	DLOAD(u8) = 24,
	ALOAD(u8) = 25,
	IALOAD = 46,
	LALOAD = 47,
	FALOAD = 48,
	DALOAD = 49,
	AALOAD = 50,
	BALOAD = 51,
	CALOAD = 52,
	SALOAD = 53,
	ISTORE(u8) = 54,
	LSTORE(u8) = 55,
	FSTORE(u8) = 56,
	DSTORE(u8) = 57,
	ASTORE(u8) = 58,
	IASTORE = 79,
	LASTORE = 80,
	FASTORE = 81,
	DASTORE = 82,
	AASTORE = 83,
	BASTORE = 84,
	CASTORE = 85,
	SASTORE = 86,
	POP = 87,
	POP2 = 88,
	DUP = 89,
	DUP_X1 = 90,
	DUP_X2 = 91,
	DUP2 = 92,
	DUP2_X1 = 93,
	DUP2_X2 = 94,
	SWAP = 95,
	IADD = 96,
	LADD = 97,
	FADD = 98,
	DADD = 99,
	ISUB = 100,
	LSUB = 101,
	FSUB = 102,
	DSUB = 103,
	IMUL = 104,
	LMUL = 105,
	FMUL = 106,
	DMUL = 107,
	IDIV = 108,
	LDIV = 109,
	FDIV = 110,
	DDIV = 111,
	IREM = 112,
	LREM = 113,
	FREM = 114,
	DREM = 115,
	INEG = 116,
	LNEG = 117,
	FNEG = 118,
	DNEG = 119,
	ISHL = 120,
	LSHL = 121,
	ISHR = 122,
	LSHR = 123,
	IUSHR = 124,
	LUSHR = 125,
	IAND = 126,
	LAND = 127,
	IOR = 128,
	LOR = 129,
	IXOR = 130,
	LXOR = 131,
	// https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-6.html#jvms-6.5.areturn
	IINC { index: u8, r#const: u8 } = 132,
	I2L = 133,
	I2F = 134,
	I2D = 135,
	L2I = 136,
	L2F = 137,
	L2D = 138,
	F2I = 139,
	F2L = 140,
	F2D = 141,
	D2I = 142,
	D2L = 143,
	D2F = 144,
	I2B = 145,
	I2C = 146,
	I2S = 147,
	LCMP = 148,
	FCMPL = 149,
	FCMPG = 150,
	DCMPL = 151,
	DCMPG = 152,
	IFEQ(u16) = 153,
	IFNE(u16) = 154,
	IFLT(u16) = 155,
	IFGE(u16) = 156,
	IFGT(u16) = 157,
	IFLE(u16) = 158,
	IF_ICMPEQ(u16) = 159,
	IF_ICMPNE(u16) = 160,
	IF_ICMPLT(u16) = 161,
	IF_ICMPGE(u16) = 162,
	IF_ICMPGT(u16) = 163,
	IF_ICMPLE(u16) = 164,
	IF_ACMPEQ(u16) = 165,
	IF_ACMPNE(u16) = 166,
	GOTO(u16) = 167,
	JSR = 168,
	RET = 169,
	TABLESWITCH = 170,
	LOOKUPSWITCH = 171,
	IRETURN = 172,
	LRETURN = 173,
	FRETURN = 174,
	DRETURN = 175,
	ARETURN = 176,
	RETURN = 177,
	GETSTATIC(CPFieldRef) = 178,
	PUTSTATIC(CPFieldRef) = 179,
	GETFIELD(CPFieldRef) = 180,
	PUTFIELD(CPFieldRef) = 181,
	INVOKEVIRTUAL(CPMethodRef) = 182,
	INVOKESPECIAL(CPMethodRef) = 183,
	INVOKESTATIC(CPMethodRef) = 184,
	INVOKEINTERFACE { method: CPInterfaceMethodRef, count: u8 } = 185,
	INVOKEDYNAMIC(CPInvokeDynamicRef) = 186,
	NEW(CPClassRef) = 187,
	// https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-6.html#jvms-6.5.areturn
	NEWARRAY(u8) = 188,
	ANEWARRAY(CPClassRef) = 189,
	ARRAYLENGTH = 190,
	ATHROW = 191,
	CHECKCAST(CPClassRef) = 192,
	INSTANCEOF(CPClassRef) = 193,
	MONITORENTER = 194,
	MONITOREXIT = 195,
	MULTIANEWARRAY = 197,
	IFNULL(u16) = 198,
	IFNONNULL(u16) = 199,
}

impl Instructions {
	pub fn read<B: BytesReadExt>(cp: &[IRCpTag], buffer: &mut B) -> Result<Instructions, IRClassfileError> {
		Ok(match buffer.read_u8()? {
			Opcodes::GETFIELD => Instructions::GETFIELD(CPFieldRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::GETSTATIC => Instructions::GETSTATIC(CPFieldRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::PUTSTATIC => Instructions::PUTSTATIC(CPFieldRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::LDC => Instructions::LDC(
				cp.get(buffer.read_u8()?.saturating_sub(1) as usize)
					.cloned()
					.expect("fuck"),
			),
			/* LDC_W */
			0x13 => Instructions::LDC(
				cp.get(buffer.read_u16()?.saturating_sub(1) as usize)
					.cloned()
					.expect("fuck"),
			),
			Opcodes::INVOKEVIRTUAL => Instructions::INVOKEVIRTUAL(CPMethodRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::INVOKESPECIAL => Instructions::INVOKESPECIAL(CPMethodRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::INVOKESTATIC => Instructions::INVOKESTATIC(CPMethodRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::INVOKEINTERFACE => {
				let s = Instructions::INVOKEINTERFACE {
					method: CPInterfaceMethodRef::from_cp(cp, buffer.read_u16()?),
					count: buffer.read_u8()?,
				};
				buffer.read_u8()?;
				s
			}
			Opcodes::INVOKEDYNAMIC => {
				let s = Instructions::INVOKEDYNAMIC(CPInvokeDynamicRef::from_cp(cp, buffer.read_u16()?));
				buffer.read_u16()?;
				s
			}
			Opcodes::RETURN => Instructions::RETURN,
			Opcodes::ALOAD => Instructions::ALOAD(buffer.read_u8()?),
			/* aload_0 */ 0x2A => Instructions::ALOAD(0),
			/* aload_1 */ 0x2B => Instructions::ALOAD(1),
			/* aload_2 */ 0x2C => Instructions::ALOAD(2),
			/* aload_3 */ 0x2D => Instructions::ALOAD(3),

			Opcodes::ASTORE => Instructions::ASTORE(buffer.read_u8()?),
			/* astore_0 */ 0x4B => Instructions::ASTORE(0),
			/* astore_1 */ 0x4C => Instructions::ASTORE(1),
			/* astore_2 */ 0x4D => Instructions::ASTORE(2),
			/* astore_3 */ 0x4E => Instructions::ASTORE(3),
			Opcodes::NEW => Instructions::NEW(CPClassRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::DUP => Instructions::DUP,
			Opcodes::PUTFIELD => Instructions::PUTFIELD(CPFieldRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::ICONST_M1 => Instructions::ICONST_M1,
			Opcodes::ICONST_0 => Instructions::ICONST_0,
			Opcodes::ICONST_1 => Instructions::ICONST_1,
			Opcodes::ICONST_2 => Instructions::ICONST_2,
			Opcodes::ICONST_3 => Instructions::ICONST_3,
			Opcodes::ICONST_4 => Instructions::ICONST_4,
			Opcodes::ICONST_5 => Instructions::ICONST_5,
			Opcodes::IRETURN => Instructions::IRETURN,
			Opcodes::ARETURN => Instructions::ARETURN,
			Opcodes::FRETURN => Instructions::FRETURN,
			Opcodes::ILOAD => Instructions::ILOAD(buffer.read_u8()?),
			/* iload_0 */ 0x1A => Instructions::ILOAD(0),
			/* iload_1 */ 0x1B => Instructions::ILOAD(1),
			/* iload_2 */ 0x1C => Instructions::ILOAD(2),
			/* iload_3 */ 0x1D => Instructions::ILOAD(3),

			Opcodes::ISTORE => Instructions::ISTORE(buffer.read_u8()?),
			/* istore_0 */ 0x3B => Instructions::ISTORE(0),
			/* istore_1 */ 0x3C => Instructions::ISTORE(1),
			/* istore_2 */ 0x3D => Instructions::ISTORE(2),
			/* istore_3 */ 0x3E => Instructions::ISTORE(3),

			Opcodes::IFEQ => Instructions::IFEQ(buffer.read_u16()?),
			Opcodes::IFNE => Instructions::IFNE(buffer.read_u16()?),
			Opcodes::IFLT => Instructions::IFLT(buffer.read_u16()?),
			Opcodes::IFGE => Instructions::IFGE(buffer.read_u16()?),
			Opcodes::IFGT => Instructions::IFGT(buffer.read_u16()?),
			Opcodes::IFLE => Instructions::IFLE(buffer.read_u16()?),
			Opcodes::IF_ICMPEQ => Instructions::IF_ICMPEQ(buffer.read_u16()?),
			Opcodes::IF_ICMPNE => Instructions::IF_ICMPNE(buffer.read_u16()?),
			Opcodes::IF_ICMPLT => Instructions::IF_ICMPLT(buffer.read_u16()?),
			Opcodes::IF_ICMPGE => Instructions::IF_ICMPGE(buffer.read_u16()?),
			Opcodes::IF_ICMPGT => Instructions::IF_ICMPGT(buffer.read_u16()?),
			Opcodes::IF_ICMPLE => Instructions::IF_ICMPLE(buffer.read_u16()?),
			Opcodes::IF_ACMPEQ => Instructions::IF_ACMPEQ(buffer.read_u16()?),
			Opcodes::IF_ACMPNE => Instructions::IF_ACMPNE(buffer.read_u16()?),
			Opcodes::IFNULL => Instructions::IFNULL(buffer.read_u16()?),
			Opcodes::IFNONNULL => Instructions::IFNONNULL(buffer.read_u16()?),
			Opcodes::GOTO => Instructions::GOTO(buffer.read_u16()?),

			Opcodes::LCONST_0 => Instructions::LCONST_0,
			Opcodes::LCONST_1 => Instructions::LCONST_1,

			Opcodes::AASTORE => Instructions::AASTORE,

			Opcodes::FLOAD => Instructions::FLOAD(buffer.read_u8()?),
			/* fload_0 */ 0x22 => Instructions::FLOAD(0),
			/* fload_1 */ 0x23 => Instructions::FLOAD(1),
			/* fload_2 */ 0x24 => Instructions::FLOAD(2),
			/* fload_3 */ 0x25 => Instructions::FLOAD(3),

			Opcodes::LLOAD => Instructions::LLOAD(buffer.read_u8()?),
			/* lload_0 */ 0x1E => Instructions::LLOAD(0),
			/* lload_1 */ 0x1F => Instructions::LLOAD(1),
			/* lload_2 */ 0x20 => Instructions::LLOAD(2),
			/* lload_3 */ 0x21 => Instructions::LLOAD(3),

			Opcodes::LSTORE => Instructions::LSTORE(buffer.read_u8()?),
			/* lstore_0 */ 0x3F => Instructions::LSTORE(0),
			/* lstore_1 */ 0x40 => Instructions::LSTORE(1),
			/* lstore_2 */ 0x41 => Instructions::LSTORE(2),
			/* lstore_3 */ 0x42 => Instructions::LSTORE(3),

			Opcodes::LADD => Instructions::LADD,
			Opcodes::BIPUSH => Instructions::BIPUSH(buffer.read_u8()?),
			Opcodes::SIPUSH => Instructions::SIPUSH(buffer.read_u16()?),
			Opcodes::ATHROW => Instructions::ATHROW,
			Opcodes::POP => Instructions::POP,
			Opcodes::CHECKCAST => Instructions::CHECKCAST(CPClassRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::INSTANCEOF => Instructions::INSTANCEOF(CPClassRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::SWAP => Instructions::SWAP,
			Opcodes::NOP => Instructions::NOP,
			Opcodes::ANEWARRAY => Instructions::ANEWARRAY(CPClassRef::from_cp(cp, buffer.read_u16()?)),
			Opcodes::IAND => Instructions::IAND,
			Opcodes::ACONST_NULL => Instructions::ACONST_NULL,
			Opcodes::ARRAYLENGTH => Instructions::ARRAYLENGTH,
			Opcodes::IASTORE => Instructions::IASTORE,
			Opcodes::IALOAD => Instructions::IALOAD,
			Opcodes::AALOAD => Instructions::AALOAD,
			Opcodes::IINC => Instructions::IINC {
				index: buffer.read_u8()?,
				r#const: buffer.read_u8()?,
			},
			Opcodes::NEWARRAY => Instructions::NEWARRAY(buffer.read_u8()?),
			Opcodes::TABLESWITCH => {
				todo!("https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-6.html#jvms-6.5.tableswitch")
			}

			b => todo!("unparsed opcode 0x{b:02X}"),
		})
	}
}
