use super::error::{
	IoError,
	IoErrorKind,
};

pub trait Write {
	fn write(&mut self, buf: &[u8]) -> Result<usize, IoError>;

	fn flush(&mut self) -> Result<(), IoError>;

	fn write_all(&mut self, mut buf: &[u8]) -> Result<(), IoError> {
		while !buf.is_empty() {
			match self.write(buf) {
				Ok(0) => {
					return Err(IoError(IoErrorKind::UnexpectedEof));
				}
				Ok(n) => buf = &buf[n..],
				Err(ref e) if e.0 == IoErrorKind::Interrupted => {}
				Err(e) => return Err(e),
			}
		}
		Ok(())
	}

	fn by_ref(&mut self) -> &mut Self
	where
		Self: Sized,
	{
		self
	}
}

impl<W: Write + ?Sized> Write for &mut W {
	fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
		(*self).write(buf)
	}

	fn flush(&mut self) -> Result<(), IoError> {
		(*self).flush()
	}
}
