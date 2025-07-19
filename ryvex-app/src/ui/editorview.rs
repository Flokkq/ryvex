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
		ParseResult,
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
				":"  => EditorCommand::Static {
					fun: |cx| { cx.editor.enter_command_mode(); EventResult::Consumed(None) },
					doc: "insert mode"
				},
				"q"  => EditorCommand::Static {
					fun: |cx| { cx.editor.quit(); EventResult::Consumed(None) },
					doc: "quit editor"
				},
				"w"  => EditorCommand::Static {
					fun: |cx| { cx.editor.write_active_document(&cx.target_cx.fs); EventResult::Consumed(None) },
					doc: "quit editor"
				},
				"h"  => EditorCommand::Motion(
					Motion::NavigationOnly { nav: NavigationMotion::CharBackward, count: 1 }),
				"j"  => EditorCommand::Motion(
					Motion::NavigationOnly { nav: NavigationMotion::LineForward, count: 1 }),
				"k"  => EditorCommand::Motion(
					Motion::NavigationOnly { nav: NavigationMotion::LineBackward, count: 1 }),
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
				"<C-[>" => EditorCommand::Static {
					fun: |cx| { cx.editor.enter_normal_mode(); EventResult::Consumed(None) },
					doc: "normal mode"
				},
			}

			command {
				"<C-[>" => EditorCommand::Static {
					fun: |cx| { cx.editor.enter_normal_mode(); cx.editor.log_info("Entering command mode"); EventResult::Consumed(None) },
					doc: "normal mode"
				},
				"<C-M>" => EditorCommand::Static {
					fun: |cx| { let _ = cx.editor.submit_command(cx.target_cx); cx.editor.log_info("Exectuing command"); EventResult::Consumed(None) },
					doc: "submit command"
				},
			}
}

impl EditorView {
	pub fn new() -> Self {
		let km: &'static mut KeyMaps = Box::leak(Box::new(KeyMaps::new()));
		let parser = KeyParser::new(&km.normal);
		Self { parser, km }
	}

	fn switch_keymap(&mut self, mode: Mode) {
		match mode {
			Mode::Normal => self.parser.set_keymap(&self.km.normal),
			Mode::Insert => self.parser.set_keymap(&self.km.insert),
			Mode::Visual => self.parser.set_keymap(&self.km.normal),
			Mode::Command => self.parser.set_keymap(&self.km.command),
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

	fn execute(
		&mut self,
		cmd: &EditorCommand,
		repeat: Option<u32>,
		cx: &mut Context,
	) -> EventResult {
		match cmd {
			EditorCommand::Static { fun, .. } => fun(cx),

			EditorCommand::Motion(motion) => {
				let effective = if let Some(r) = repeat {
					scaled_motion(motion, r)
				} else {
					motion.clone()
				};
				motion::apply(cx.editor.buffer_mut(), effective);
				EventResult::Consumed(None)
			}

			EditorCommand::Typable { name, args } => {
				cx.editor.run_ex_command(name, args)
			}

			EditorCommand::Macro(keys) => {
				for k in keys.iter().cloned() {
					let evt = Event::Key(k);
					self.handle_event(&evt, cx);
					if cx.editor.should_close() {
						break;
					}
				}
				EventResult::Consumed(None)
			}
		}
	}

	fn insert_default(&self, key: AsciiKeyCode, cx: &mut Context) {
		match key {
			AsciiKeyCode::Backspace | AsciiKeyCode::Del => {
				cx.editor.delete_at_cursor()
			}
			AsciiKeyCode::CarriageReturn | AsciiKeyCode::LineFeed => {
				cx.editor.insert_character('\n')
			}
			k if k.is_control_character() => {}
			k if k.is_seperator() => {
				if k == AsciiKeyCode::Space {
					cx.editor.insert_character(' ');
				}
			}
			printable => cx.editor.insert_character(printable.to_char()),
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
				let mode_before = cx.editor.mode;
				self.switch_keymap(mode_before);

				if mode_before == Mode::Insert {
					let parse_res = self.parser.feed(*key);

					match parse_res {
						ParseResult::Incomplete => {
							return EventResult::Consumed(None);
						}
						ParseResult::Command(cmd, repeat) => {
							let res = self.execute(cmd, repeat, cx);
							self.switch_keymap(cx.editor.mode);
							return res;
						}
						ParseResult::Error => {
							self.parser.set_keymap(match cx.editor.mode {
								Mode::Insert => &self.km.insert,
								_ => &self.km.normal,
							});
							self.insert_default(*key, cx);
							return EventResult::Consumed(None);
						}
					}
				}

				match self.parser.feed(*key) {
					ParseResult::Incomplete => {
						return EventResult::Consumed(None)
					}
					ParseResult::Command(cmd, repeat) => {
						let res = self.execute(cmd, repeat, cx);
						self.switch_keymap(cx.editor.mode);
						return res;
					}
					ParseResult::Error => {
						cx.editor.log_warn(format!("Unknown mapping: {}", key));
						return EventResult::Consumed(None);
					}
				}
			}
			Event::Resize(_, _) => { /* TODO */ }
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

fn scaled_motion(m: &Motion, mult: u32) -> Motion {
	if mult <= 1 {
		return m.clone();
	}
	match m {
		Motion::NavigationOnly { nav, count } => Motion::NavigationOnly {
			nav:   *nav,
			count: count.saturating_mul(mult),
		},
		Motion::OperatedNavigation {
			motion_type,
			nav,
			count,
		} => Motion::OperatedNavigation {
			motion_type: *motion_type,
			nav:         *nav,
			count:       count.saturating_mul(mult),
		},
		Motion::OperatedRange {
			motion_type,
			range,
			count,
		} => Motion::OperatedRange {
			motion_type: *motion_type,
			range:       range.clone(),
			count:       count.saturating_mul(mult),
		},
	}
}
