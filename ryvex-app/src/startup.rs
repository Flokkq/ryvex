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

	pub fn run_until_stopped(&self) -> Result<i32> {
		Ok(0)
	}
}
