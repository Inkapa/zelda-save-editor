use std::path::PathBuf;
use std::sync::Mutex;

use save_engine::Save;

#[derive(Default)]
pub struct AppState {
    pub save: Mutex<Option<Save>>,
    pub path: Mutex<Option<PathBuf>>,
}
