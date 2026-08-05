use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};

use save_engine::Save;

#[derive(Default)]
pub struct AppState {
    pub save: Mutex<Option<Save>>,
    pub path: Mutex<Option<PathBuf>>,
}

/// Locks a Mutex, recovering the inner value instead of panicking if a previous holder panicked
/// while holding the lock. A poisoned AppState lock would otherwise break every command for the
/// rest of the process; the guarded value itself is still perfectly usable after a poison.
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
