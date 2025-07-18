use ryvex_core::motion::{
	self,
	Motion,
	MotionType,
	NavigationMotion,
};
use ryvex_target::{
	key::AsciiKeyCode,
	term::event::Event,
};

use crate::{
	compositor::{
		Component,
		Context,
		EventResult,
	},
	define_keymaps,
	editor::{
		document::{
			Document,
			Mode,
		},
		editor::Editor,
	},
	keymap::{
		EditorCommand,
		KeyParser,
	},
};

pub struct EditorView {
	parser: KeyParser<'static>,
	km:     &'static KeyMaps,
}

impl Default for EditorView {
	fn default() -> Self {
		Self::new()
	}
}

define_keymaps! {
			normal {
				"i"  => EditorCommand::Static {
					fun: |cx| { cx.editor.enter_insert_mode(); EventResult::Consumed(None) },
					doc: "insert mode"
				},
				// "u"  => EditorCommand::Static { fun: commands::undo,          doc: "undo" },
				// "gd" => EditorCommand::Static { fun: lsp::goto_definition,    doc: "goto definition" },

				"h"  => EditorCommand::Motion(
					Motion::NavigationOnly { nav: NavigationMotion::CharBackward, count: 1 }),
				"l"  => EditorCommand::Motion(
					Motion::NavigationOnly { nav: NavigationMotion::CharForward,  count: 1 }),
				"dw" => EditorCommand::Motion(
					Motion::OperatedNavigation {
						motion_type: MotionType::Delete,
						nav: NavigationMotion::WordForward,
						count: 1
					}),
			}

			insert {
				"<Esc>" => EditorCommand::Static {
					fun: |cx| { cx.editor.enter_normal_mode(); EventResult::Consumed(None) },
					doc: "normal mode"
				},
			}
}

impl EditorView {
	pub fn new() -> Self {
		let km: &'static mut KeyMaps = Box::leak(Box::new(KeyMaps::new()));
		let parser = KeyParser::new(&km.normal);

		Self { parser, km }
	}

	fn execute(
		&mut self,
		cmd: &EditorCommand,
		repeat: Option<u32>,
		cx: &mut Context,
	) -> EventResult {
		match cmd {
			EditorCommand::Static { fun, .. } => fun(cx),

			EditorCommand::Motion(motion) => {
				if let Some(r) = repeat {
					multiply_motion_count(&mut motion.clone(), r);
				}

				motion::apply(cx.editor.buffer_mut(), motion.to_owned());
				EventResult::Consumed(None)
			}

			EditorCommand::Typable { name, args } => {
				cx.editor.run_ex_command(name, args)
			}
			EditorCommand::Macro(keys) => {
				for k in keys.iter().cloned() {
					self.handle_event(&Event::Key(k), cx);
				}
				EventResult::Consumed(None)
			}
		}
	}

	pub fn render_view(
		&self,
		frame: &mut ryvex_tui::buffer::Buffer,
		_editor: &Editor,
		doc: &Document,
		area: ryvex_ui::graphics::Rect,
	) {
		let max_rows = area.height.saturating_sub(2);
		for (row_idx, line) in doc.content().lines().enumerate() {
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
			AsciiKeyCode::Space => cx.editor.insert_character(' '),
			AsciiKeyCode::Backspace | AsciiKeyCode::Del => {
				cx.editor.delete_at_cursor()
			}
			AsciiKeyCode::CarriageReturn => cx.editor.insert_character('\n'),
			_control_char if key.is_control_character() => {}
			_seperator if key.is_seperator() => {}
			_printable_character => cx.editor.insert_character(key.to_char()),
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
		event: &ryvex_target::term::event::Event,
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

				// match cx.editor.mode {
				// 	Mode::Normal => self.parser.set_keymap(&self.km.normal),
				// 	Mode::Insert => self.parser.set_keymap(&self.km.insert),
				// 	Mode::Visual => self.parser.set_keymap(&self.km.visual),
				// 	Mode::Command => return EventResult::Ignored(None),
				// }
			}
			Event::Resize(_, _) => todo!(),
		}

		EventResult::Consumed(None)
	}

	fn should_update(&self) -> bool {
		true
	}
}

fn multiply_motion_count(motion: &mut Motion, r: u32) {
	match motion {
		Motion::NavigationOnly { count, .. } |
		Motion::OperatedNavigation { count, .. } => {
			*count = count.saturating_mul(r);
		}
		_ => {}
	}
}
