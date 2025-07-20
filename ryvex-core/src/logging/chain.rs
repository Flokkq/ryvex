use ryvex_target::std::error::Error;

pub const MAX_ERROR_FRAMES: usize = 32;

#[derive(Debug)]
pub struct ErrorFrame {
	pub msg: String,
}

#[derive(Debug)]
pub struct ErrorChain {
	pub frames:    Vec<ErrorFrame>,
	pub truncated: bool,
}

impl ErrorChain {
	pub fn capture(err: &(dyn Error + 'static)) -> ErrorChain {
		let mut frames = Vec::new();
		let mut truncated = false;

		let mut current: Option<&(dyn Error + 'static)> = Some(err);
		let mut i = 0;

		while let Some(e) = current {
			if i == MAX_ERROR_FRAMES {
				truncated = true;
				break;
			}

			frames.push(ErrorFrame { msg: e.to_string() });
			current = e.source();
			i += 1;
		}

		ErrorChain { frames, truncated }
	}
}

pub enum TraceData {
	None,
	ErrorChain(ErrorChain),
}
