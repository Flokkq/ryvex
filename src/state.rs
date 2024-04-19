use std::sync::MutexGuard;
use std::sync::{Mutex, OnceLock};

use crate::startup::OpenFile;

pub struct AppState {
    pub file: Option<OpenFile>,
}

pub struct GlobalState {
    state: Mutex<AppState>,
}

impl GlobalState {
    pub fn new() -> Self {
        let initial_state = AppState { file: None };
        GlobalState {
            state: Mutex::new(initial_state),
        }
    }

    pub fn get_state(
        &self,
    ) -> Result<
        MutexGuard<'_, AppState>,
        std::sync::PoisonError<MutexGuard<'_, AppState>>,
    > {
        self.state.lock()
    }
}

static GLOBAL_STATE: OnceLock<GlobalState> = OnceLock::new();

pub fn get_global_state() -> &'static GlobalState {
    GLOBAL_STATE.get_or_init(GlobalState::new)
}

pub fn set_open_file(open_file: OpenFile) {
    let global_state = get_global_state();

    if let Ok(mut state) = global_state.get_state() {
        state.file = Some(open_file);
    } else {
        panic!("Global state mutex is poisoned");
    }
}
