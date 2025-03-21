use crate::{
	args::Args,
	error::Result,
};

pub struct Application {
}

impl Application {
	pub fn build(args: Args) -> Result<Self> {
	}

	pub fn run_until_stopped(&self) -> Result<i32> {
		Ok(0)
	}
}
