use std::io::stdout;

use log::{
	info,
	warn,
};
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
			if !self.main_loop(input_stream)? {
				break Ok(0);
			}
		}
	}

	pub fn main_loop<S>(&self, input_stream: &mut S) -> Result<bool>
	where
		S: Iterator<Item = ryvex_term::error::Result<Event>>,
	{
		loop {
			match input_stream.next() {
				Some(Ok(event)) => {
					self.handle_terminal_event(event);
				}
				Some(Err(e)) => {
					warn!("Could not recieve terminal event: '{:?}'", e);
				}
				_ => continue,
			}
		}
	}

	pub fn handle_terminal_event(&self, event: Event) {
		match event {
			Event::Key(ascii_key_code) => {
				todo!("I dont know how to handle '{}' yet :/", ascii_key_code)
			}
			Event::Resize(_, _) => {
				todo!("I dont know how to handle resize event yet :/")
			}
		}
	}
}
