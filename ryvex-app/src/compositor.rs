use std::any::Any;

use crate::editor::editor::Editor;

use ryvex_term::event::Event;
use ryvex_tui::buffer::Buffer;
use ryvex_ui::graphics::Rect;

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
}

impl Compositor {
	pub fn new(area: Rect) -> Self {
		Self {
			layers: Vec::new(),
			area,
			full_redraw: false,
		}
	}

	pub fn size(&self) -> Rect {
		self.area
	}

	pub fn render(&mut self, area: Rect, frame: &mut Buffer, cx: &mut Context) {
		for layer in &mut self.layers {
			layer.render(area, frame, cx);
		}
	}

	pub fn push(&mut self, mut layer: Box<dyn Component>) {
		let size = self.size();
		layer.required_size((size.width, size.height));
		self.layers.push(layer);
	}

	pub fn handle_event(&mut self, event: &Event, cx: &mut Context) -> bool {
		let mut consumed = false;
		let mut callbacks = Vec::new();

		for layer in self.layers.iter_mut().rev() {
			match layer.handle_event(event, cx) {
				EventResult::Consumed(Some(callback)) => {
					callbacks.push(callback);
					consumed = true;
					break;
				}
				EventResult::Consumed(None) => {
					consumed = true;
					break;
				}
				EventResult::Ignored(Some(callback)) => {
					callbacks.push(callback);
				}
				EventResult::Ignored(None) => {}
			}
		}

		for callback in callbacks {
			callback(self, cx)
		}

		consumed
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
	fn handle_event(
		&mut self,
		_event: &Event,
		_cx: &mut Context,
	) -> EventResult {
		EventResult::Ignored(None)
	}

	/// Should redraw? Useful for saving redraw cycles if we know component
	/// didn't change.
	fn should_update(&self) -> bool {
		true
	}

	/// Render the component onto the provided surface.
	fn render(&mut self, area: Rect, frame: &mut Buffer, cx: &mut Context);

	fn type_name(&self) -> &'static str {
		std::any::type_name::<Self>()
	}

	fn id(&self) -> Option<&'static str> {
		None
	}

	fn required_size(&mut self, _viewport: (u16, u16)) -> Option<(u16, u16)> {
		None
	}
}
