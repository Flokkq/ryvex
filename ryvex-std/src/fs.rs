use std::{
	env::{
		self,
		home_dir,
	},
	fs,
	io::Write,
	path::PathBuf,
};

use log::{
	debug,
	error,
	info,
};

use crate::error::{
	Result,
	StdError,
};

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

/// Takes a Pathbuf and expands it into its full path
pub fn expand(mut path: PathBuf) -> Result<String> {
	if !path.exists() {
		return Err(StdError::IoError(std::io::Error::other(
			"Path does not exists",
		)));
	}

	if !path.is_absolute() {
		path = env::current_dir()?.join(path);
	}

	let mut s = path.canonicalize()?.to_string_lossy().into_owned();

	if let Some(home) = home_dir() {
		let home = home.to_string_lossy().into_owned();
		if s.starts_with(&home) {
			s.replace_range(..home.len(), "~");
		}
	}

	Ok(s)
}
