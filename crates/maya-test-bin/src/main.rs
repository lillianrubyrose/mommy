use std::{io::Cursor, path::PathBuf};

use maya_classfile_io::ClassFile;

const CLASS_BYTES: &[u8] = include_bytes!("../data/Hello.class");

fn main() -> eyre::Result<()> {
	let buffer = CLASS_BYTES.to_vec();
	let mut buffer = Cursor::new(buffer);

	let cf = ClassFile::read(&mut buffer)?;
	println!("{:#?}", cf);

	let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
	cf.write(&mut buffer)?;

	let path = PathBuf::from("out.class");
	std::fs::write(path, buffer.get_ref())?;

	println!("wrote classfile");

	Ok(())
}
