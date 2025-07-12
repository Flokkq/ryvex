use alloc::{
	string::String,
	vec::Vec,
};

pub trait Environment {
	fn var(&self, key: &str) -> Option<String>;

	fn set_var(&self, key: &str, val: &str);

	fn args(&self) -> Vec<String>;
}
