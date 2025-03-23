use std::io::stdout;

use log::info;
use ryvex_term::event::Event;

use crate::{
	args::Args,
	editor::{
		document::Document,
		editor::Editor,
	},
	error::Result,
};

pub struct Application {
	pub editor: Editor,
}

impl Application {
	pub fn build(args: Args) -> Result<Self> {
		let mut editor = Editor::new();
		let document = Document::new(args.file)?;
		let _id = editor.new_document(document);

		Ok(Application { editor })
	}

	pub fn run_until_stopped<S>(&self, input_stream: &mut S) -> Result<i32>
	where
		S: Iterator<Item = ryvex_term::error::Result<Event>>,
	{
		let mut stdout = stdout().lock();
		self.editor.render(&mut stdout)?;

		loop {
			match input_stream.next() {
				Some(Ok(event)) => {
					info!("Recieved terminal event: '{:?}'", event);
					break Ok(42);
				}
				_ => continue,
			}
		}
	}
}
