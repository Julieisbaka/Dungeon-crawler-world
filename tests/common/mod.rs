use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Creates a uniquely-named temporary directory and returns its path.
/// The name embeds both the current process ID and a per-process counter so
/// that paths are globally unique even when multiple integration-test binaries
/// run in parallel and each resets the counter to 1.
/// Any pre-existing directory at that path is removed first.
pub fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "{}_{}_{}", prefix, std::process::id(), id
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}
