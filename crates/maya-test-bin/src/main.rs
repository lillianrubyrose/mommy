use std::{
	fs::File,
	io::{Cursor, Read},
	path::Path,
};

use maya_classfile_io::IOClassFile;
use maya_classfile_ir::IRClassFile;

const CLASS_BYTES: &[u8] = include_bytes!("../data/Hello.class");

fn main() -> eyre::Result<()> {
	let path = Path::new("crates/maya-test-bin/data");
	let mut dir = path.read_dir().unwrap();
	while let Some(Ok(entry)) = dir.next() {
		let name = entry.file_name();
		if name.to_str().unwrap().ends_with(".class") {
			let class_content = std::fs::read(entry.path()).unwrap();
			let mut buffer = Cursor::new(class_content);

			println!("Parsing: {name:?}");
			let cf = IOClassFile::read(&mut buffer)?;
			let cf = IRClassFile::from_io(cf);
			println!("Parsed: {name:?}");
			println!("{:#?}", cf);
		}
	}

	// let buffer = CLASS_BYTES.to_vec();
	// let mut buffer = Cursor::new(buffer);

	// let cf = IOClassFile::read(&mut buffer)?;
	// let cf = IRClassFile::from_io(cf);

	// let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
	// cf.write(&mut buffer)?;

	// let path = PathBuf::from("out.class");
	// std::fs::write(path, buffer.get_ref())?;

	// println!("wrote classfile");

	Ok(())
}
