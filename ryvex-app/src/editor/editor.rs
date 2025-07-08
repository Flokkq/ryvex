use std::{
	collections::BTreeMap,
	num::NonZeroUsize,
	process::{
		Command,
		ExitStatus,
		Stdio,
	},
};

use super::{
	document::{
		Document,
		DocumentId,
		Mode,
	},
	error::CommandError,
};

use crate::error::{
	Result,
	RyvexError,
};

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

	fn write_active_document(&mut self) {
		if let Some(doc) = self.get_active_document() {
			match doc.save() {
				Ok(_) => {
					let path =
						doc.diplay_path().unwrap_or_else(|| "[scratch]".into());

					let msg = format!(
						"\"{path}\" {}L, {}B written",
						doc.text().lines().count(),
						doc.text().len()
					);

					self.log_info(msg);
				}
				Err(e) => self.log_error(format!("write failed: {e}")),
			}
			return;
		}

		self.log_warn("No open document".to_string());
	}

	pub fn insert_character(&mut self, key: ryvex_term::key::AsciiKeyCode) {
		if self.mode == Mode::Command {
			self.push_command_char(key.to_char());
		} else if let Some(document) = self.get_active_document_mut() {
			document.insert_character(key);
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

	pub fn submit_command(&mut self) -> Result<ExitStatus> {
		let input: String = self.command_buffer.trim().to_string();

		if input.starts_with('!') {
			let command = &input[1..];

			let parts: Vec<&str> = command.split_whitespace().collect();

			if !parts.is_empty() {
				let status = Command::new(parts[0])
					.args(&parts[1..])
					.stdin(Stdio::null())
					.stdout(Stdio::null())
					.status()
					.map_err(|_| CommandError::ExecutionFailed)
					.map_err(RyvexError::from)?;

				if !status.success() {
					return Err(CommandError::ExecutionFailed.into());
				}

				return Ok(status);
			}

			return Err(CommandError::InvalidCommand.into());
		}

		match input.as_str() {
			"q" | "quit" => self.quit(),
			"w" | "write" => self.write_active_document(),
			_ => return Err(CommandError::InvalidCommand.into()),
		}

		Ok(ExitStatus::default())
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
