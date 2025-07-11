use alloc::string::String;
use core::{
	fmt,
	marker::PhantomData,
};
use std::{
	convert::Infallible,
	str::FromStr,
};

pub trait PathScheme {
	const MAIN_SEPARATOR: char;
	const ALT_SEPARATOR: &'static [char];
	const EXTENSION_SEPARATOR: char;
	const CURRENT_DIR: &'static str;
	const PARENT_DIR: &'static str;
}

#[derive(Clone, PartialEq, Eq)]
pub struct Path<S: PathScheme> {
	inner:   String,
	_scheme: PhantomData<S>,
}

impl<S: PathScheme> Default for Path<S> {
	fn default() -> Self {
		Self::new()
	}
}

impl<S: PathScheme> Path<S> {
	/// Empty path.
	pub fn new() -> Self {
		Path {
			inner:   String::new(),
			_scheme: PhantomData,
		}
	}

	pub fn as_str(&self) -> &str {
		&self.inner
	}

	/// Push a segment, inserting `MAIN_SEPARATOR` if needed.
	pub fn push(&mut self, segment: &str) {
		if let Some(last) = self.inner.chars().next_back() {
			if last != S::MAIN_SEPARATOR && !S::ALT_SEPARATOR.contains(&last) {
				self.inner.push(S::MAIN_SEPARATOR);
			}
		}

		let trimmed = segment.trim_start_matches(|c| {
			c == S::MAIN_SEPARATOR || S::ALT_SEPARATOR.contains(&c)
		});
		self.inner.push_str(trimmed);
	}

	/// Pop the final component; return true if anything was removed.
	pub fn pop(&mut self) -> bool {
		let is_sep =
			|c| c == S::MAIN_SEPARATOR || S::ALT_SEPARATOR.contains(&c);
		if let Some(idx) = self.inner.rfind(is_sep) {
			self.inner.truncate(idx);
			true
		} else if !self.inner.is_empty() {
			self.inner.clear();
			true
		} else {
			false
		}
	}

	/// Final segment, if any.
	pub fn file_name(&self) -> Option<&str> {
		let is_sep =
			|c| c == S::MAIN_SEPARATOR || S::ALT_SEPARATOR.contains(&c);
		self.inner.rsplit(is_sep).next().filter(|s| !s.is_empty())
	}

	/// Portion after the last `EXTENSION_SEPARATOR`, if any.
	pub fn extension(&self) -> Option<&str> {
		self.file_name().and_then(|name| {
			name.rfind(S::EXTENSION_SEPARATOR).map(|i| &name[i + 1..])
		})
	}

	/// The parent path, if any.
	pub fn parent(&self) -> Option<Path<S>> {
		let is_sep =
			|c| c == S::MAIN_SEPARATOR || S::ALT_SEPARATOR.contains(&c);
		if let Some(idx) = self.inner.rfind(is_sep) {
			Some(Path {
				inner:   self.inner[..idx].to_owned(),
				_scheme: PhantomData,
			})
		} else if !self.inner.is_empty() {
			Some(Path::new())
		} else {
			None
		}
	}
}

impl<S: PathScheme> FromStr for Path<S> {
	type Err = Infallible;

	/// Infallible
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Path {
			inner:   s.to_owned(),
			_scheme: PhantomData,
		})
	}
}

impl<S: PathScheme> fmt::Debug for Path<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&self.inner, f)
	}
}
impl<S: PathScheme> fmt::Display for Path<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.inner)
	}
}
