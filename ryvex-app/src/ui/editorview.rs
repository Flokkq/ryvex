use crate::{
	compositor::Component,
	editor::{
		document::Document,
		editor::Editor,
	},
};

pub struct EditorView {}

impl EditorView {
	pub fn new() -> Self {
		Self {}
	}

	pub fn render_view(&self, editor: &Editor, doc: &Document) {}
}

impl Component for EditorView {
	fn render(
		&mut self,
		area: ryvex_ui::graphics::Rect,
		frame: &mut ryvex_tui::buffer::Buffer,
	) {
		todo!()
	}
}
