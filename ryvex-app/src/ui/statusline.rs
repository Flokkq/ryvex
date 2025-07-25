use alloc::{
	format,
	string::ToString,
};
use ryvex_target::term::event::Event;
use ryvex_tui::buffer::Buffer;
use ryvex_ui::graphics::Rect;

use crate::compositor::{
	Component,
	Context,
	EventResult,
};

pub struct StatusLine;

impl Default for StatusLine {
	fn default() -> Self {
		Self::new()
	}
}

impl StatusLine {
	pub fn new() -> Self {
		Self
	}
}

impl Component for StatusLine {
	fn render(&mut self, area: Rect, frame: &mut Buffer, cx: &mut Context) {
		let y = area.y + area.height.saturating_sub(2);
		let width = area.width as usize;

		frame.set_string(0, y, " ".repeat(width));

		let path = if let Some(doc) = cx.editor.get_active_document() {
			doc.diplay_path(&cx.target_cx.fs)
				.unwrap_or("[No Name]".into())
		} else {
			"[No Document]".to_string()
		};

		let file = format!(" {} | {}", cx.editor.mode, path);
		frame.set_string(0, y, &file[..file.len().min(width)]);

		if let Some(doc) = cx.editor.get_active_document() {
			let row = doc.current_row();
			let col = doc.current_col();

			let right = format!("{row}|{col}");
			let start = area.width.saturating_sub(right.len() as u16);

			frame.set_string(start, y, right);
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
