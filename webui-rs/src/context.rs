use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, Weak},
};

use crate::{Window, WindowInner};

pub(crate) static CONTEXT: LazyLock<Mutex<HashMap<usize, Weak<WindowInner>>>> =
    LazyLock::new(|| Default::default());

pub(crate) fn get_window(id: usize) -> Option<Window> {
    CONTEXT
        .lock()
        .unwrap()
        .get(&id)
        .and_then(|x| x.upgrade())
        .map(|inner| Window { inner })
}
