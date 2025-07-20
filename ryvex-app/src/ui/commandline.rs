use alloc::format;
use ryvex_target::term::event::Event;
use ryvex_tui::buffer::Buffer;
use ryvex_ui::graphics::Rect;

use crate::{
	compositor::{
		Component,
		Context,
		EventResult,
	},
	editor::document::Mode,
};

pub struct CommandLine;

impl Default for CommandLine {
	fn default() -> Self {
		Self::new()
	}
}

impl CommandLine {
	pub fn new() -> Self {
		Self
	}
}

impl Component for CommandLine {
	fn render(&mut self, area: Rect, frame: &mut Buffer, cx: &mut Context) {
		let y = area.y + area.height.saturating_sub(1);
		let width = area.width as usize;

		if cx.editor.mode == Mode::Command {
			frame.set_string(0, y, " ".repeat(width));
			let text = format!(":{}", cx.editor.command_buffer());
			frame.set_string(0, y, &text[..text.len().min(width)]);
		}
	}

	fn handle_event(
		&mut self,
		_event: &Event,
		_cx: &mut Context,
	) -> EventResult {
		EventResult::Ignored(None)
	}

	fn should_update(&self) -> bool {
		true
	}
}
