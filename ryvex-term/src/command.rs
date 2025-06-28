use std::fmt;
use std::io::{
	self,
	Write,
};

pub trait Command {
	/// Write an ANSI representation of this command to the given writer.
	/// An ANSI code can manipulate the terminal by writing it to the terminal
	/// buffer. However, only Windows 10 and UNIX systems support this.
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result;

	/// Windows versions lower than windows 10 do not support ANSI escape codes,
	/// therefore a direct WinAPI call is made.
	#[cfg(windows)]
	fn execute_winapi(&self) -> io::Result<()>;

	/// Returns whether the ANSI code representation of this command is
	/// supported by windows.
	#[cfg(windows)]
	fn is_ansi_code_supported(&self) -> bool {
		use sys::windows;
		windows::supports_ansi()
	}
}

impl<T: Command + ?Sized> Command for &T {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		(**self).write_ansi(f)
	}

	#[inline]
	#[cfg(windows)]
	fn execute_winapi(&self) -> io::Result<()> {
		T::execute_winapi(self)
	}

	#[cfg(windows)]
	#[inline]
	fn is_ansi_code_supported(&self) -> bool {
		T::is_ansi_code_supported(self)
	}
}

pub trait QueueableCommand {
	fn queue(&mut self, command: impl Command) -> io::Result<&mut Self>;
}

pub trait ExecutableCommand {
	fn execute(&mut self, command: impl Command) -> io::Result<&mut Self>;
}

impl<T: Write + ?Sized> QueueableCommand for T {
	fn queue(&mut self, command: impl Command) -> io::Result<&mut Self> {
		#[cfg(windows)]
		if !command.is_ansi_code_supported() {
			// There may be queued commands in this writer, but `execute_winapi`
			// will execute the command immediately. To prevent commands
			// being executed out of order we flush the writer now.
			self.flush()?;
			command.execute_winapi()?;
			return Ok(self);
		}

		write_command_ansi(self, command)?;
		Ok(self)
	}
}

impl<T: Write + ?Sized> ExecutableCommand for T {
	fn execute(&mut self, command: impl Command) -> io::Result<&mut Self> {
		self.queue(command)?;
		self.flush()?;
		Ok(self)
	}
}

fn write_command_ansi<C: Command>(
	io: &mut (impl io::Write + ?Sized),
	command: C,
) -> io::Result<()> {
	struct Adapter<T> {
		inner: T,
		res:   io::Result<()>,
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
