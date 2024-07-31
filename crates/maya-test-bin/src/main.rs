use std::{io::Cursor, path::Path};

use maya_classfile_io::IOClassFile;
use maya_classfile_ir::IRClassFile;

fn main() -> eyre::Result<()> {
	let path = Path::new("crates/maya-test-bin/data");

	fn compile_classes<'a>(dir: &'a Path) {
		println!("{dir:?}");
		let mut dir = dir.read_dir().unwrap();
		while let Some(Ok(entry)) = dir.next() {
			if entry.file_type().unwrap().is_dir() {
				compile_classes(&entry.path());
				continue;
			}

			let name = entry.file_name();
			if name.to_str().unwrap().ends_with(".class") {
				let class_content = std::fs::read(entry.path()).unwrap();
				let mut buffer = Cursor::new(class_content);

				println!("Parsing: {name:?}");
				let cf = IOClassFile::read(&mut buffer).unwrap();
				let cf = IRClassFile::from_io(cf).unwrap();
				println!("Parsed: {name:?}");
				println!("{:#?}", cf);
			}
		}
	}

	compile_classes(path);

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
