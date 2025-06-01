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
		for (row_idx, line) in doc.text().lines().enumerate() {
			let y = area.y.saturating_add(row_idx as u16);
			if y >= area.y + area.height {
				break;
			}

			let max_width = area.width as usize;
			let display = if line.len() > max_width {
				&line[..max_width]
			} else {
				line
			};

			frame.set_string(area.x, y, display);
		}
	}

	pub fn insert(&self, key: AsciiKeyCode, cx: &mut Context) {
		match key {
			AsciiKeyCode::Esc => cx.editor.enter_normal_mode(),
			control_char if key.is_control_character() => {}
			seperator if key.is_seperator() => {}
			_printable_character => cx.editor.insert_character(key),
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
			Event::Key(mut key) => {
				let mode = cx.editor.mode;

				match mode {
					Mode::Normal => todo!(),
					Mode::Visual => todo!(),
					Mode::Insert => self.insert(key, cx),
				}
			}
			Event::Resize(_, _) => todo!(),
		}

		EventResult::Ignored(None)
	}
}
