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
};
use crate::error::Result;

pub struct Editor {
	pub documents:        BTreeMap<DocumentId, Document>,
	pub active_document:  Option<DocumentId>,
	pub next_document_id: DocumentId,
}

impl Editor {
	pub fn new() -> Self {
		Self {
			documents:        BTreeMap::new(),
			active_document:  None,
			next_document_id: DocumentId::default(),
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
}
