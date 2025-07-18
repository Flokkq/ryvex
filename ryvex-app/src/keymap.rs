use ryvex_core::motion::Motion;
use ryvex_target::key::AsciiKeyCode;

use crate::compositor::{
	Context,
	EventResult,
};

pub type KeyEvent = AsciiKeyCode;

#[derive(Clone)]
pub enum EditorCommand {
	Typable {
		name: String,
		args: String,
	},
	Motion(Motion),
	Static {
		fun: fn(&mut Context) -> EventResult,
		doc: &'static str,
	},
	Macro(Vec<KeyEvent>),
}

#[derive(Default)]
pub struct KeyNode {
	pub command: Option<EditorCommand>,
	pub next:    Vec<(AsciiKeyCode, Box<KeyNode>)>,
}

impl KeyNode {
	fn child_mut(&mut self, k: AsciiKeyCode) -> &mut KeyNode {
		if let Some(idx) = self.next.iter().position(|(key, _)| *key == k) {
			return self.next[idx].1.as_mut();
		}

		self.next.push((k, Box::new(KeyNode::default())));
		self.next.last_mut().unwrap().1.as_mut()
	}

	fn child(&self, k: AsciiKeyCode) -> Option<&KeyNode> {
		self.next
			.iter()
			.find(|(key, _)| *key == k)
			.map(|(_, node)| &**node)
	}

	pub fn bind(&mut self, seq: &[AsciiKeyCode], cmd: EditorCommand) {
		let mut node = self;
		for &k in seq {
			node = node.child_mut(k);
		}

		node.command = Some(cmd);
	}

	pub fn bind_str(&mut self, seq: &str, cmd: EditorCommand) {
		let keys: Vec<AsciiKeyCode> =
			seq.bytes().map(AsciiKeyCode::from).collect();
		self.bind(&keys, cmd);
	}
}

pub struct KeyParser<'a> {
	keymap: &'a KeyNode,
	cursor: &'a KeyNode,
	count:  Option<u32>,
}

impl<'a> KeyParser<'a> {
	pub fn new(root: &'a KeyNode) -> Self {
		Self {
			keymap: root,
			cursor: root,
			count:  None,
		}
	}

	pub fn set_keymap(&mut self, root: &'a KeyNode) {
		self.keymap = root;
		self.reset();
	}

	pub fn feed(&mut self, key: AsciiKeyCode) -> ParseResult<'a> {
		if core::ptr::eq(self.cursor, self.keymap) && key.is_digit() {
			let d = key.to_char().to_digit(10).unwrap();
			self.count = Some(self.count.unwrap_or(0) * 10 + d);

			return ParseResult::Incomplete;
		}

		match self.cursor.child(key) {
			Some(node) => {
				self.cursor = node;

				match &self.cursor.command {
					None => ParseResult::Incomplete,
					Some(cmd) => {
						let repeat = self.count.take();
						self.reset();
						ParseResult::Command(cmd, repeat)
					}
				}
			}
			None => {
				self.reset();
				ParseResult::Error
			}
		}
	}

	fn reset(&mut self) {
		self.cursor = self.keymap;
		self.count = None;
	}
}

pub enum ParseResult<'a> {
	Incomplete,
	Command(&'a EditorCommand, Option<u32>),
	Error,
}
