use ryvex_target::{
	target::TargetContext,
	target::{
		self,
		term::Handle,
	},
	term::event::Event,
};
use ryvex_tui::{
	backend::term::TerminalBackend,
	terminal::Terminal,
};

use alloc::boxed::Box;

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
	target_cx:  TargetContext,
}

impl Application {
	pub fn build(cx: TargetContext, args: Args) -> Result<Self> {
		let mut editor = Editor::new();
		let document = Document::new(args.file, &cx.fs)?;
		let _id = editor.new_document(document);

		let handle = Handle::from_default_tty(true, false)?;
		let area = target::term::get_terminal_size(&handle)?;
		let mut compositor = Compositor::new(area);

		let editor_view = Box::new(ui::EditorView::new());
		compositor.push(editor_view);
		compositor.push(Box::new(ui::StatusLine::new()));
		compositor.push(Box::new(ui::CommandLine::new()));

		let terminal = Terminal::new(TerminalBackend::new(handle))?;

		Ok(Application {
			editor,
			compositor,
			terminal,
			target_cx: cx,
		})
	}

	pub fn run_until_stopped<S>(&mut self, input_stream: &mut S) -> Result<i32>
	where
		S: Iterator<Item = ryvex_target::std::Result<Event>>,
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
		S: Iterator<Item = ryvex_target::std::Result<Event>>,
	{
		loop {
			if self.editor.should_close() {
				return Ok(false);
			};

			match input_stream.next() {
				Some(Ok(event)) => {
					self.handle_terminal_event(event);
				}
				Some(Err(_)) => {}
				_ => continue,
			}
		}
	}

	fn render(&mut self) {
		self.terminal.clear().expect("Failed to clear terminal");

		let mut cx = crate::compositor::Context {
			editor:    &mut self.editor,
			target_cx: &mut self.target_cx,
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
			editor:    &mut self.editor,
			target_cx: &mut self.target_cx,
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
