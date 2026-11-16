mod ui_context;
mod ui_window;
mod window_handle;

use std::{cell::RefCell, rc::Rc};

pub use ui_context::UiApplication;

type DestroyQueue = Rc<RefCell<Vec<usize>>>;
type CallbackSlot = Rc<RefCell<Option<mlua::RegistryKey>>>;