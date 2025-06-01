use std::{
	collections::BTreeMap,
	io::{
		StdoutLock,
		Write,
	},
	num::NonZeroUsize,
};

use ryvex_std::error::StdError;

use super::document::{
	Document,
	DocumentId,
	Mode,
};
use crate::error::Result;

pub struct Editor {
	pub documents:        BTreeMap<DocumentId, Document>,
	pub active_document:  Option<DocumentId>,
	pub next_document_id: DocumentId,

	pub mode: Mode,

	should_close: bool,
}

impl Editor {
	pub fn new() -> Self {
		Self {
			documents:        BTreeMap::new(),
			active_document:  None,
			next_document_id: DocumentId::default(),
			mode:             Mode::Normal,
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

	#[deprecated]
	pub fn render(&self, stdout: &mut StdoutLock) -> Result<()> {
		self.write(stdout, "\x1B[0m")?;
		self.write(stdout, "\x1B[2J")?;
		self.write(stdout, "\x1B[H")?;

		if let Some(document) = self.get_active_document() {
			self.write(stdout, document.text())?;
		}

		stdout.flush().map_err(|e| StdError::IoError(e).into())
	}

	fn write(&self, stdout: &mut StdoutLock, text: &str) -> Result<()> {
		stdout
			.write_all(text.as_bytes())
			.map_err(|e| StdError::IoError(e).into())
	}

	pub fn insert_character(&mut self, key: ryvex_term::key::AsciiKeyCode) {
		if let Some(document) = self.get_active_document_mut() {
			document.insert_character(key);
		}
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
}
