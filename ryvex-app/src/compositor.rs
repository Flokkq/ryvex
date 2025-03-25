use std::any::Any;

use crate::{
	editor::editor::Editor,
	error::Result,
};

use ryvex_term::event::Event;
use ryvex_tui::buffer::Buffer;
use ryvex_ui::rect::Rect;

pub struct Compositor {
	layers: Vec<Box<dyn Component>>,
	area:   Rect,

	pub(crate) full_redraw: bool,
}

pub type Callback = Box<dyn FnOnce(&mut Compositor, &mut Context)>;

pub enum EventResult {
	Ignored(Option<Callback>),
	Consumed(Option<Callback>),
}

pub struct Context<'a> {
	pub editor: &'a mut Editor,
	pub scroll: Option<usize>,
}

impl Compositor {
	pub fn new(area: Rect) -> Self {
		Self {
			layers: Vec::new(),
			area,
			full_redraw: false,
		}
	}

	pub fn render() -> Result<()> {
		Ok(())
	}
}

pub trait AnyComponent {
	fn as_any(&self) -> &dyn Any;

	fn as_any_mut(&mut self) -> &mut dyn Any;

	fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Component> AnyComponent for T {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}

	fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
		self
	}
}

impl dyn AnyComponent {
	pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
		self.as_any().downcast_ref()
	}

	pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
		self.as_any_mut().downcast_mut()
	}

	pub fn downcast<T: Any>(
		self: Box<Self>,
	) -> std::result::Result<Box<T>, Box<Self>> {
		// Do the check here + unwrap, so the error
		// value is `Self` and not `dyn Any`.
		if self.as_any().is::<T>() {
			Ok(self.as_boxed_any().downcast().unwrap())
		} else {
			Err(self)
		}
	}

	pub fn is<T: Any>(&self) -> bool {
		self.as_any().is::<T>()
	}
}

pub trait Component: Any + AnyComponent {
	/// Process input events, return true if handled.
	fn handle_event(&mut self, _event: &Event) -> EventResult {
		EventResult::Ignored(None)
	}

	/// Should redraw? Useful for saving redraw cycles if we know component
	/// didn't change.
	fn should_update(&self) -> bool {
		true
	}

	/// Render the component onto the provided surface.
	fn render(&mut self, area: Rect, frame: &mut Buffer);

	fn type_name(&self) -> &'static str {
		std::any::type_name::<Self>()
	}

	fn id(&self) -> Option<&'static str> {
		None
	}
}
