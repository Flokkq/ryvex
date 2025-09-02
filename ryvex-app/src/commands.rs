use crate::compositor::{
	Context,
	EventResult,
};
use crate::keymap::EditorCommand::{
	self,
	*,
};
use ryvex_core::motion::NavigationMotion;
use ryvex_core::{
	error_chain,
	motion::{
		Motion::*,
		MotionType::*,
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

pub fn delete_range(range: ryvex_core::motion::Range) -> EditorCommand {
	Motion(OperatedRange {
		motion_type: Delete,
		range,
		count: 1,
	})
}
