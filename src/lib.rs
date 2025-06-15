use std::sync::{Arc, Mutex, RwLock};

use nih_plug::editor::Editor;

pub mod editor;

pub fn create_wry_editor<T>(
    user_state: T,
    url: String,
) -> Option<Box<dyn Editor>>
where
    T: 'static + Send + Sync,
{
    Some(Box::new(editor::WryEditor {
        user_state: Arc::new(RwLock::new(user_state)),
        url: url.clone(),
        webview_spawned: Arc::new(Mutex::new(Vec::new())),
    }))
}
