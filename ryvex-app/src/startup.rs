use std::io::stdout;

use log::{
	info,
	warn,
};
use ryvex_term::{
	event::Event,
	sys::unix::fd::TtyFd,
};
use ryvex_tui::buffer::Buffer;

use crate::{
	args::Args,
	compositor::Compositor,
	editor::{
		document::Document,
		editor::Editor,
	},
	error::Result,
	ui,
};

pub struct Application {
	pub editor:     Editor,
	pub compositor: Compositor,
}

impl Application {
	pub fn build(args: Args) -> Result<Self> {
		let mut editor = Editor::new();
		let document = Document::new(args.file)?;
		let _id = editor.new_document(document);

		let fd = TtyFd::read()?;
		let area = ryvex_term::get_terminal_size(fd)?;
		let mut compositor = Compositor::new(area);

		let editor_view = Box::new(ui::EditorView::new());
		compositor.push(editor_view);

		Ok(Application { editor, compositor })
	}

	pub fn run_until_stopped<S>(&mut self, input_stream: &mut S) -> Result<i32>
	where
		S: Iterator<Item = ryvex_term::error::Result<Event>>,
	{
		self.render();

		loop {
			if !self.main_loop(input_stream)? {
				break Ok(0);
			}
		}
	}

	fn main_loop<S>(&mut self, input_stream: &mut S) -> Result<bool>
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

	fn render(&mut self) {
		let size = self.compositor.size();
		let mut buffer = Buffer::empty(size);
		self.compositor.render(size, &mut buffer);
	}

	fn handle_terminal_event(&self, event: Event) {
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
