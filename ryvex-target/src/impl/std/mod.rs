pub mod env;
pub mod fs;
pub mod process;

use env::StdEnv;
use fs::StdFileHandle;
use fs::StdFileSystem;
use process::StdShell;

use crate::std::path::Path;
use crate::target::TargetPathScheme;

pub type TargetEnvironment = StdEnv<TargetPathScheme>;
pub type TargetFileHandle = StdFileHandle<TargetPathScheme>;
pub type TargetFileSystem = StdFileSystem<TargetPathScheme>;
pub type TargetShell = StdShell<TargetPathScheme>;
pub type TargetPath = Path<TargetPathScheme>;

#[derive(Debug, Clone, Default)]
pub struct TargetContext {
	pub env:   TargetEnvironment,
	pub fs:    TargetFileSystem,
	pub shell: TargetShell,
}
