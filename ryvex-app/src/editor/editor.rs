use alloc::{
	collections::BTreeMap,
	format,
	string::{
		String,
		ToString,
	},
	vec::Vec,
};
use core::num::NonZeroUsize;

use ryvex_target::{
	r#impl::{
		TargetContext,
		TargetFileSystem,
	},
	std::{
		process::{
			Exitstatus,
			Shell,
			ShellError,
		},
		StdError,
	},
};

use super::document::{
	Document,
	DocumentId,
	Mode,
};

use crate::error::Result;

#[derive(Debug)]
pub struct Editor {
	pub documents:        BTreeMap<DocumentId, Document>,
	pub active_document:  Option<DocumentId>,
	pub next_document_id: DocumentId,

	pub mode: Mode,

	command_buffer: String,
	last_message:   Option<LogMessage>,

	should_close: bool,
}

impl Default for Editor {
	fn default() -> Self {
		Self::new()
	}
}

impl Editor {
	pub fn new() -> Self {
		Self {
			documents:        BTreeMap::new(),
			active_document:  None,
			next_document_id: DocumentId::default(),
			mode:             Mode::Normal,
			command_buffer:   String::new(),
			last_message:     None,
			should_close:     false,
		}
	}

	pub fn new_document(&mut self, mut document: Document) -> DocumentId {
		let id = self.next_document_id;
		self.next_document_id = DocumentId(unsafe {
			NonZeroUsize::new_unchecked(self.next_document_id.0.get() + 1)
		});

		document.id = id;
		self.documents.insert(id, document);
		self.active_document = Some(id);

		id
	}

	pub fn get_active_document(&self) -> Option<&Document> {
		self.active_document.and_then(|id| self.documents.get(&id))
	}

	pub fn get_active_document_mut(&mut self) -> Option<&mut Document> {
		self.active_document
			.and_then(move |id| self.documents.get_mut(&id))
	}

	fn write_active_document(&mut self, fs: &TargetFileSystem) {
		if let Some(doc) = self.get_active_document() {
			match doc.save(fs) {
				Ok(_) => {
					let path = doc
						.diplay_path(fs)
						.unwrap_or_else(|| "[scratch]".into());

					let msg = format!(
						"\"{path}\" {}L, {}B written",
						doc.rows(),
						doc.len()
					);

					self.log_info(msg);
				}
				Err(e) => self.log_error(format!("write failed: {e}")),
			}
			return;
		}

		self.log_warn("No open document".to_string());
	}

	pub fn insert_character(&mut self, key: char) {
		if self.mode == Mode::Command {
			self.push_command_char(key);
		} else if let Some(document) = self.get_active_document_mut() {
			document.insert_character(key);
		}
	}

	pub fn delete_at_cursor(&mut self) {
		self.log_info("Deleting character");
		if let Some(d) = self.get_active_document_mut() {
			d.delete_at_cursor();
		}
	}

	pub fn enter_command_mode(&mut self) {
		self.command_buffer.clear();
		self.mode = Mode::Command;
	}

	pub fn exit_command_mode(&mut self) {
		self.command_buffer.clear();
		self.mode = Mode::Normal;
	}

	pub fn push_command_char(&mut self, ch: char) {
		self.command_buffer.push(ch);
	}

	pub fn pop_command_char(&mut self) {
		let _ = self.command_buffer.pop();
	}

	pub fn submit_command(
		&mut self,
		target: &TargetContext,
	) -> Result<Exitstatus> {
		let input: String = self.command_buffer.trim().to_string();

		if let Some(command) = input.strip_prefix('!') {
			let parts: Vec<&str> = command.split_whitespace().collect();

			if !parts.is_empty() {
				let status = target.shell.status(parts[0], &parts[1..])?;

				if status.failure() {
					return Err(StdError::Shell(ShellError::ExecutionFailed(
						input,
					))
					.into());
				}

				return Ok(status);
			}

			return Err(
				StdError::Shell(ShellError::CommandNotFound(input)).into()
			);
		}

		match input.as_str() {
			"q" | "quit" => self.quit(),
			"w" | "write" => self.write_active_document(&target.fs),
			_ => {
				return Err(
					StdError::Shell(ShellError::CommandNotFound(input)).into()
				)
			}
		}

		Ok(Exitstatus::Success)
	}

	pub fn enter_normal_mode(&mut self) {
		self.mode = Mode::Normal;
	}

	pub fn enter_insert_mode(&mut self) {
		self.mode = Mode::Insert;
	}

	pub fn quit(&mut self) {
		self.documents.clear();
		self.active_document = None;
	}

	pub fn should_close(&self) -> bool {
		self.should_close
	}

	pub fn log_info<S: Into<String>>(&mut self, msg: S) {
		self.last_message = Some(LogMessage {
			level: LogLevel::Info,
			text:  msg.into(),
		})
	}
	pub fn log_warn<I: Into<String>>(&mut self, msg: I) {
		self.last_message = Some(LogMessage {
			level: LogLevel::Warn,
			text:  msg.into(),
		})
	}

	pub fn log_error<S: Into<String>>(&mut self, msg: S) {
		self.last_message = Some(LogMessage {
			level: LogLevel::Error,
			text:  msg.into(),
		})
	}

	pub fn command_buffer(&self) -> &str {
		&self.command_buffer
	}

	pub fn last_message(&self) -> Option<&LogMessage> {
		self.last_message.as_ref()
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
	Info,
	Warn,
	Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogMessage {
	pub level: LogLevel,
	pub text:  String,
}
