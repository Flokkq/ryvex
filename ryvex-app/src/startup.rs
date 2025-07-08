use log::warn;
use ryvex_term::{
	event::Event,
	sys::target::fd::TtyFd,
};
use ryvex_tui::{
	backend::term::TerminalBackend,
	terminal::Terminal,
};

use crate::{
	args::Args,
	compositor::{
		self,
		Compositor,
	},
	editor::{
		document::Document,
		editor::Editor,
	},
	error::Result,
	ui,
};

pub struct Application {
	editor:     Editor,
	compositor: Compositor,
	terminal:   Terminal<TerminalBackend>,
}

impl Application {
	pub fn build(args: Args) -> Result<Self> {
		let mut editor = Editor::new();
		let document = Document::new(args.file)?;
		let _id = editor.new_document(document);

		let fd = TtyFd::read()?;
		let area = ryvex_term::sys::target::get_terminal_size(&fd)?;
		let mut compositor = Compositor::new(area);

		let editor_view = Box::new(ui::EditorView::new());
		compositor.push(editor_view);
		compositor.push(Box::new(ui::StatusLine::new()));

		let terminal = Terminal::new(TerminalBackend::new(fd))?;

		Ok(Application {
			editor,
			compositor,
			terminal,
		})
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
			if self.editor.should_close() {
				return Ok(false);
			};

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
		self.terminal.clear().expect("Failed to clear terminal");

		let mut cx = crate::compositor::Context {
			editor: &mut self.editor,
		};

		let area = self.terminal.size().expect("Failed to get terminal size");
		let surface = self.terminal.current_buffer_mut();

		self.compositor.render(area, surface, &mut cx);
		let _ = self
			.terminal
			.draw(None, ryvex_ui::graphics::CursorKind::Block);
	}

	fn handle_terminal_event(&mut self, event: Event) {
		let mut cx = compositor::Context {
			editor: &mut self.editor,
		};

		let should_redraw = match event {
			Event::Resize(_, _) => {
				todo!("I dont know how to handle resize event yet :/")
			}
			e => self.compositor.handle_event(&e, &mut cx),
		};

		if should_redraw && !self.editor.should_close() {
			self.render();
		}
	}
}
