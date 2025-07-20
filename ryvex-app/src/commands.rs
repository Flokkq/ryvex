use crate::keymap::EditorCommand::{
	self,
	*,
};
use crate::{
	compositor::{
		Context,
		EventResult,
	},
	define_keymaps,
};
use ryvex_core::motion::NavigationMotion;
use ryvex_core::{
	error_chain,
	motion::{
		Motion::*,
		MotionType::*,
		NavigationMotion::*,
	},
};

pub fn to_insert_mode(cx: &mut Context) -> EventResult {
	cx.editor.enter_insert_mode();
	EventResult::Consumed(None)
}

pub fn to_command_mode(cx: &mut Context) -> EventResult {
	cx.editor.enter_command_mode();
	EventResult::Consumed(None)
}

pub fn quit_editor(cx: &mut Context) -> EventResult {
	cx.editor.quit();
	EventResult::Consumed(None)
}

pub fn write_active_document(cx: &mut Context) -> EventResult {
	cx.editor.write_active_document(&cx.target_cx.fs);
	EventResult::Consumed(None)
}

pub fn cmd(
	f: fn(&mut Context) -> EventResult,
	doc: &'static str,
) -> EditorCommand {
	Static { fun: f, doc }
}

pub fn to_normal_mode(cx: &mut Context) -> EventResult {
	cx.editor.enter_normal_mode();
	EventResult::Consumed(None)
}

pub fn submit_command(cx: &mut Context) -> EventResult {
	let _ = cx
		.editor
		.submit_command(cx.target_cx)
		.map_err(|e| error_chain!(&e, "failed executing command"));

	cx.editor.enter_normal_mode();
	EventResult::Consumed(None)
}

pub fn nav(nav: NavigationMotion) -> EditorCommand {
	Motion(NavigationOnly { nav, count: 1 })
}

pub fn delete(nav: NavigationMotion) -> EditorCommand {
	Motion(OperatedNavigation {
		motion_type: Delete,
		nav,
		count: 1,
	})
}

define_keymaps! {
	normal {
		"i" => cmd(to_insert_mode,"enter insert mode"),
		":" => cmd(to_command_mode,"enter command mode"),

		"q" => cmd(quit_editor,"quit editor"),
		"w" => cmd(write_active_document,"save active document"),

		"h" => nav(CharBackward),
		"j" => nav(LineForward),
		"k" => nav(LineBackward),
		"l" => nav(CharForward),
		"dw" => delete(WordForward),
	}
	insert {
		"<C-[>"=> cmd(to_normal_mode,"normal mode")
	}
	command {
		"<C-[>"=> cmd(to_normal_mode,"normal mode"),
		"<C-M>"=> cmd(submit_command, "submit command"),
	}
}
