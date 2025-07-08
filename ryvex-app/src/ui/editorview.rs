use ryvex_term::{
	event::Event,
	key::AsciiKeyCode,
};

use crate::{
	compositor::{
		Component,
		Context,
		EventResult,
	},
	editor::{
		document::{
			Document,
			Mode,
		},
		editor::Editor,
	},
};

pub struct EditorView {}

impl Default for EditorView {
	fn default() -> Self {
		Self::new()
	}
}

impl EditorView {
	pub fn new() -> Self {
		Self {}
	}

	pub fn render_view(
		&self,
		frame: &mut ryvex_tui::buffer::Buffer,
		_editor: &Editor,
		doc: &Document,
		area: ryvex_ui::graphics::Rect,
	) {
		let max_rows = area.height.saturating_sub(2);
		for (row_idx, line) in doc.text().lines().enumerate() {
			if row_idx as u16 >= max_rows {
				break;
			}
			let y = area.y + row_idx as u16;
			let slice = &line[..line.len().min(area.width as usize)];
			frame.set_string(area.x, y, slice);
		}
	}

	pub fn insert(&self, key: AsciiKeyCode, cx: &mut Context) {
		match key {
			AsciiKeyCode::Esc => cx.editor.enter_normal_mode(),
			_control_char if key.is_control_character() => {}
			_seperator if key.is_seperator() => {}
			_printable_character => cx.editor.insert_character(key),
		}
	}

	pub fn normal(&self, key: AsciiKeyCode, cx: &mut Context) {
		match key {
			AsciiKeyCode::LowerI => cx.editor.enter_insert_mode(),
			AsciiKeyCode::LowerQ => cx.editor.quit(),
			AsciiKeyCode::Colon => cx.editor.enter_command_mode(),
			_ => {}
		}
	}
}

impl Component for EditorView {
	fn render(
		&mut self,
		area: ryvex_ui::graphics::Rect,
		frame: &mut ryvex_tui::buffer::Buffer,
		cx: &mut Context,
	) {
		let doc = cx.editor.get_active_document().expect("");
		self.render_view(frame, cx.editor, doc, area);
	}

	fn handle_event(
		&mut self,
		event: &ryvex_term::event::Event,
		cx: &mut crate::compositor::Context,
	) -> crate::compositor::EventResult {
		match event {
			Event::Key(key) => {
				let mode = cx.editor.mode;

				match mode {
					Mode::Normal => self.normal(*key, cx),
					Mode::Visual => todo!(),
					Mode::Insert => self.insert(*key, cx),
					Mode::Command => return EventResult::Ignored(None),
				}
			}
			Event::Resize(_, _) => todo!(),
		}

		EventResult::Consumed(None)
	}

	fn should_update(&self) -> bool {
		true
	}
}
