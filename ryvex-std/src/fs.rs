use std::{
	fs,
	io::Write,
	path::PathBuf,
};

use log::{
	debug,
	error,
	info,
};

use crate::error::Result;

pub fn write(content: &str, path: &PathBuf) -> Result<()> {
	let path_str = path.to_string_lossy();
	let line_count = content.lines().count();

	debug!("Attempting to write file: {}", path_str);

	let mut file = fs::File::create(path)?;
	match file.write(content.as_bytes()) {
		Ok(bytes_written) => {
			info!(
				"\"{}\" {}L, {}B written",
				path_str, line_count, bytes_written
			);
			Ok(())
		}
		Err(e) => {
			error!("Failed to write file: {}", path_str);
			Err(e.into())
		}
	}
}

pub fn read_from_file_if_exists(
	path: &PathBuf,
	buffer: &mut String,
) -> Result<()> {
	debug!("Attempting to read file: {}", path.display());
	if let Ok(_metadata) = fs::metadata(path) {
		*buffer = fs::read_to_string(path)?;
		return Ok(());
	}

	debug!(
		"File '{}' does not exist, creating new buffer",
		path.display()
	);
	*buffer = String::new();

	Ok(())
}
