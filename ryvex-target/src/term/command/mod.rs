use core::fmt;

use crate::{
	std::{
		error::IoError,
		write::Write,
	},
	target,
};

pub mod cursor;
pub mod terminal;

pub trait Command: WriteAnsi + ExecuteApi {}

impl<T: WriteAnsi + ExecuteApi> Command for T {}

pub trait WriteAnsi {
	/// Write an ANSI representation of this command to the given writer.
	/// An ANSI code can manipulate the terminal by writing it to the terminal
	/// buffer. Since a target can also only sometimes support ANSI escape
	/// codes, implementing both paths has to be possible
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result;
}

pub trait ExecuteApi {
	/// Execute this command using platform-specific APIs.
	#[inline]
	fn execute_api(&self) -> Result<(), IoError> {
		Ok(())
	}

	/// Returns whether the ANSI code representation of this command is
	/// supported by the target.
	fn is_ansi_code_supported(&self) -> bool {
		target::term::supports_ansi()
	}
}

impl<T: WriteAnsi + ?Sized> WriteAnsi for &T {
	#[inline]
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		(**self).write_ansi(f)
	}
}

impl<T: ExecuteApi + ?Sized> ExecuteApi for &T {
	#[inline]
	fn execute_api(&self) -> Result<(), IoError> {
		(**self).execute_api()
	}

	#[inline]
	fn is_ansi_code_supported(&self) -> bool {
		(**self).is_ansi_code_supported()
	}
}

pub trait QueueableCommand {
	fn queue(&mut self, command: impl Command) -> Result<&mut Self, IoError>;
}

pub trait ExecutableCommand {
	fn execute(&mut self, command: impl Command) -> Result<&mut Self, IoError>;
}

impl<T: Write + ?Sized> QueueableCommand for T {
	fn queue(&mut self, command: impl Command) -> Result<&mut Self, IoError> {
		if !command.is_ansi_code_supported() {
			// There may be queued commands in this writer, but `execute_api`
			// will execute the command immediately. To prevent commands
			// being executed out of order we flush the writer now.
			self.flush()?;
			command.execute_api()?;
			return Ok(self);
		}

		write_command_ansi(self, command)?;
		Ok(self)
	}
}

impl<T: Write + ?Sized> ExecutableCommand for T {
	fn execute(&mut self, command: impl Command) -> Result<&mut Self, IoError> {
		self.queue(command)?;
		self.flush()?;
		Ok(self)
	}
}

fn write_command_ansi<C: Command>(
	io: &mut (impl Write + ?Sized),
	command: C,
) -> Result<(), IoError> {
	struct Adapter<T> {
		inner: T,
		res:   Result<(), IoError>,
	}

	impl<T: Write> fmt::Write for Adapter<T> {
		fn write_str(&mut self, s: &str) -> fmt::Result {
			self.inner.write_all(s.as_bytes()).map_err(|e| {
				self.res = Err(e);
				fmt::Error
			})
		}
	}

	let mut adapter = Adapter {
		inner: io,
		res:   Ok(()),
	};

	command
		.write_ansi(&mut adapter)
		.map_err(|fmt::Error| match adapter.res {
			Ok(()) => panic!(
				"<{}>::write_ansi incorrectly errored",
				std::any::type_name::<C>()
			),
			Err(e) => e,
		})
}
