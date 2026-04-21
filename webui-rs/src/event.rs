use std::ffi::{c_char, CStr, CString};

use webui_sys::*;

use crate::{CONTEXT, EventType, Window};

/// The event type.
#[repr(transparent)]
pub struct Event {
    pub(crate) inner: *mut webui_event_t,
}

impl Event {
    /// Get the event.
    pub(crate) fn get_event(&self) -> &webui_event_t {
        unsafe { &*self.inner }
    }

    /// Get the inner pointer.
    pub(crate) fn as_ptr(&self) -> *mut webui_event_t {
        self.inner
    }

    /// Get the window.
    pub fn get_window(&self) -> Option<Window> {
        let window = self.get_event().window;
        CONTEXT
            .lock()
            .unwrap()
            .get(&window)
            .and_then(|x| x.upgrade())
            .map(|inner| Window { inner })
    }

    pub(crate) fn get_bind_id(&self) -> usize {
        self.get_event().bind_id
    }

    /// Get the event type.
    pub fn get_type(&self) -> EventType {
        unsafe { std::mem::transmute(self.get_event().event_type) }
    }

    /// Show a window for a specific single client. Useful in multi-client mode to send different content to different connected
    /// clients.
    pub fn show_client(&self, content: &str) -> bool {
        let content = CString::new(content).unwrap();
        unsafe { webui_show_client(self.as_ptr(), content.as_ptr()) }
    }

    /// Close the connection for a specific single client only, without closing the window for other connected clients.
    pub fn close_client(&self) {
        unsafe { webui_close_client(self.as_ptr()) }
    }

    /// Navigate a specific single client to a URL, without affecting other connected clients.
    pub fn navigate_client(&self, url: &str) {
        let url = CString::new(url).unwrap();
        unsafe { webui_navigate_client(self.as_ptr(), url.as_ptr()) }
    }

    /// Run JavaScript for a specific single client without waiting for a response.
    pub fn run_client(&self, script: &str) {
        let script = CString::new(script).unwrap();
        unsafe {
            webui_run_client(self.as_ptr(), script.as_ptr());
        }
    }

    /// Run JavaScript for a specific single client and get the response back.
    pub fn script_client(&self, script: &str, timeout: usize, capacity: usize) -> Option<String> {
        let mut buffer: Vec<c_char> = Vec::with_capacity(capacity);
        let script = CString::new(script).unwrap();
        unsafe {
            webui_script_client(
                self.as_ptr(),
                script.as_ptr(),
                timeout,
                buffer.as_mut_ptr(),
                capacity,
            )
        }
        .then(|| {
            unsafe { CStr::from_ptr(buffer.as_ptr()) }
                .to_string_lossy()
                .to_string()
        })
    }

    /// Get the number of arguments passed to the callback from JavaScript.
    pub fn get_count(&self) -> usize {
        unsafe { webui_get_count(self.as_ptr()) }
    }

    /// Get the first argument as a 64-bit integer.
    pub fn get_int(&self) -> i64 {
        unsafe { webui_get_int(self.as_ptr()) }
    }

    /// Get an argument as a 64-bit integer at a specific index.
    pub fn get_int_at(&self, index: usize) -> i64 {
        unsafe { webui_get_int_at(self.as_ptr(), index) }
    }

    /// Get the first argument as a double-precision float.
    pub fn get_float(&self) -> f64 {
        unsafe { webui_get_float(self.as_ptr()) }
    }

    /// Get an argument as a double-precision float at a specific index.
    pub fn get_float_at(&self, index: usize) -> f64 {
        unsafe { webui_get_float_at(self.as_ptr(), index) }
    }

    /// Get the first argument as a string slice.
    pub fn get_string(&self) -> String {
        unsafe {
            let ptr = webui_get_string(self.as_ptr());
            CStr::from_ptr(ptr).to_string_lossy().to_string()
        }
    }

    /// Get an argument as a string slice at a specific index.
    pub fn get_string_at(&self, index: usize) -> String {
        unsafe {
            let ptr = webui_get_string_at(self.as_ptr(), index);
            CStr::from_ptr(ptr).to_string_lossy().to_string()
        }
    }

    /// Get the first argument as a boolean.
    pub fn get_bool(&self) -> bool {
        unsafe { webui_get_bool(self.as_ptr()) }
    }

    /// Get an argument as a boolean at a specific index.
    pub fn get_bool_at(&self, index: usize) -> bool {
        unsafe { webui_get_bool_at(self.as_ptr(), index) }
    }

    /// Get the size in bytes of the first argument. Useful for raw binary data.
    pub fn get_size(&self) -> usize {
        unsafe { webui_get_size(self.as_ptr()) }
    }

    /// Get the size in bytes of an argument at a specific index.
    pub fn get_size_at(&self, index: usize) -> usize {
        unsafe { webui_get_size_at(self.as_ptr(), index) }
    }

    /// Return a 64-bit integer as the response to a JavaScript await call.
    pub fn return_int(&self, value: i64) {
        unsafe { webui_return_int(self.as_ptr(), value) }
    }

    /// Return a double-precision float as the response to a JavaScript await call.
    pub fn return_float(&self, value: f64) {
        unsafe { webui_return_float(self.as_ptr(), value) }
    }

    /// Return a string as the response to a JavaScript await call.
    pub fn return_string<S: AsRef<str>>(&self, value: S) {
        let c_str = std::ffi::CString::new(value.as_ref()).unwrap_or_default();
        unsafe { webui_return_string(self.as_ptr(), c_str.as_ptr()) }
    }

    /// Return a boolean as the response to a JavaScript await call.
    pub fn return_bool(&self, value: bool) {
        unsafe { webui_return_bool(self.as_ptr(), value) }
    }

    /// Send raw binary data to a JavaScript function for a specific single client only.
    pub fn send_raw<T>(&self, function: &str, data: T)
    where
        T: Into<Vec<u8>>,
    {
        let function = CString::new(function).unwrap();
        let data = data.into().into_boxed_slice();
        unsafe {
            webui_send_raw_client(
                self.as_ptr(),
                function.as_ptr(),
                data.as_ptr() as _,
                data.len(),
            )
        }
    }
}
