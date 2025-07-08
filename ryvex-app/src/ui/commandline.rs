use ryvex_term::{
	event::Event,
	key::AsciiKeyCode,
};
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

impl CommandLine {
	pub fn new() -> Self {
		Self
	}
}

impl Component for CommandLine {
	fn render(&mut self, area: Rect, frame: &mut Buffer, cx: &mut Context) {
		let y = area.y + area.height.saturating_sub(1);
		let width = area.width as usize;
		frame.set_string(0, y, " ".repeat(width));

		match cx.editor.mode {
			Mode::Command => {
				let text = format!(":{}", cx.editor.command_buffer());
				frame.set_string(0, y, &text[..text.len().min(width)]);
			}

			_ => {
				if let Some(msg) = &cx.editor.last_message() {
					frame.set_string(
						0,
						y,
						&msg.text[..msg.text.len().min(width)],
					);
				}
			}
		}
	}

	fn handle_event(&mut self, event: &Event, cx: &mut Context) -> EventResult {
		if cx.editor.mode != Mode::Command {
			return EventResult::Ignored(None);
		}

		match event {
			Event::Key(key) => match key {
				AsciiKeyCode::Esc => cx.editor.exit_command_mode(),
				AsciiKeyCode::CarriageReturn => {
					let _ = cx
						.editor
						.submit_command()
						.map_err(|err| cx.editor.log_error(err.to_string()));

					cx.editor.enter_normal_mode();
				}
				AsciiKeyCode::Backspace => cx.editor.pop_command_char(),
				_other
					if !key.is_control_character() && !key.is_seperator() =>
				{
					cx.editor.push_command_char(key.to_char());
				}
				_ => {}
			},
			_ => {}
		}

		EventResult::Consumed(None)
	}

	fn should_update(&self) -> bool {
		true
	}
}
