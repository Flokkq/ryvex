use crate::commands::{
	cmd,
	delete,
	delete_range,
	nav,
	quit_editor,
	submit_command,
	to_command_mode,
	to_insert_mode,
	to_normal_mode,
	write_active_document,
};
use crate::define_keymaps;
use ryvex_core::motion::NavigationMotion::{
	self,
	*,
};
use ryvex_core::motion::Range;

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

		"d" => {
			"i" => {
				"w" => delete_range(Range::Word),
				_   => delete_range(Range::Line)
			},
			"w" => delete(NavigationMotion::WordForward),
			"b" => delete(NavigationMotion::WordBackward),
		}
	}

	insert {
		"<C-[>"=> cmd(to_normal_mode,"normal mode")
	}
	command {
		"<C-[>"=> cmd(to_normal_mode,"normal mode"),
		"<C-M>"=> cmd(submit_command, "submit command"),
	}
}
