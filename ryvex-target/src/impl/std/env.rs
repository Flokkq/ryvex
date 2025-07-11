use std::marker::PhantomData;

use crate::std::{
	env::Environment,
	path::PathScheme,
};

#[derive(Debug, Clone, Default)]
pub struct StdEnv<S: PathScheme> {
	_scheme: PhantomData<S>,
}

impl<S: PathScheme> StdEnv<S> {
	pub fn new() -> Self {
		StdEnv {
			_scheme: PhantomData,
		}
	}
}

impl<S: PathScheme> Environment for StdEnv<S> {
	fn var(&self, key: &str) -> Option<String> {
		std::env::var(key).ok()
	}

	fn args(&self) -> Vec<String> {
		std::env::args().collect()
	}

	fn set_var(&self, key: &str, val: &str) {
		std::env::set_var(key, val)
	}
}
