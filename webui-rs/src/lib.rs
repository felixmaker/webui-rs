#![warn(missing_docs)]

//! Rust bindings to WebUI.

mod context;
mod error;
mod event;
mod handler;
mod types;
mod window;

pub(crate) use context::*;
pub use error::*;
pub use event::*;
pub use types::*;
pub use window::*;

use std::{
    ffi::{c_char, c_void, CStr, CString},
    panic::{catch_unwind, AssertUnwindSafe},
};

use webui_sys::*;

use crate::handler::LoggerHandler;

/// Close all open windows. This will make wait() return (Break).
pub fn exit() {
    unsafe { webui_exit() }
}

/// Block the current thread and wait until all opened windows get closed.
pub fn wait() {
    unsafe { webui_wait() }
}

/// Non-blocking alternative to wait(). Returns true if at least one window is still open, false when all windows are
/// closed. Call this in a loop from the main thread to interleave your own main-thread work.
///
/// # Safety
/// In WebView mode, you must call this from the main thread.
pub fn wait_async() -> bool {
    unsafe { webui_wait_async() }
}

/// Open a URL in the operating system's default web browser (not a WebUI window).
pub fn open_url(url: &str) {
    let url = CString::new(url).unwrap();
    unsafe { webui_open_url(url.as_ptr()) }
}

/// Control WebUI global behaviour. It's recommended to call this at the beginning, before show().
pub(crate) fn set_config(option: Config, status: bool) {
    unsafe {
        webui_set_config(option as _, status);
    }
}

/// Wait for browser to connect before show() returns.
/// Default: true
pub fn set_show_wait_connection(status: bool) {
    set_config(Config::ShowWaitConnection, status);
}

/// Process all UI events in a single blocking thread (true)
/// or each in a new thread (false). Applies to all windows.
/// Default: false
pub fn set_ui_event_blocking(status: bool) {
    set_config(Config::UiEventBlocking, status);
}

/// Auto-refresh the window when any file in the root folder changes.
/// Default: false
pub fn set_folder_monitor(status: bool) {
    set_config(Config::FolderMonitor, status);
}

/// Allow multiple browser clients to connect to the same window.
/// Useful for web apps. See documentation for details.
/// Default: false
pub fn set_multi_client(status: bool) {
    set_config(Config::MultiClient, status);
}

/// Use WebUI auth cookies to identify clients and block unauthorized
/// URL access. Keep true to restrict access to one client at a time.
/// Default: true
pub fn set_use_cookies(status: bool) {
    set_config(Config::UseCookies, status);
}

/// Set to true if your backend uses async operations and sets
/// responses via webui_return_x() after the callback returns.
/// Default: false
pub fn set_asynchronous_response(status: bool) {
    set_config(Config::AsynchronousResponse, status);
}

/// Set the web-server root folder path for all windows.
pub fn set_default_root_folder(path: &str) -> Result<(), WebUIError> {
    let path = CString::new(path).unwrap();
    let result = unsafe { webui_set_default_root_folder(path.as_ptr()) };
    WebUIError::from_bool(result)
}

/// Get the HTTP MIME type string for a given file extension.
pub fn get_mime_type(file: &str) -> String {
    let file = CString::new(file).unwrap();
    unsafe {
        let mine_type = webui_get_mime_type(file.as_ptr());
        CStr::from_ptr(mine_type).to_string_lossy().to_string()
    }
}

/// Check whether a specific web browser is installed on the system.
pub fn browser_exist(browser: Browser) -> bool {
    unsafe { webui_browser_exist(browser as _) }
}

/// Set a custom folder path where WebUI should look for the browser executable.
pub fn set_browser_folder(path: &str) {
    let path = CString::new(path).unwrap();
    unsafe { webui_set_browser_folder(path.as_ptr()) }
}

/// Delete all local web-browser profile folders created by WebUI.
pub fn delete_all_profiles() {
    unsafe { webui_delete_all_profiles() }
}

/// Find and return an available (unused) network port.
pub fn get_port() -> usize {
    unsafe { webui_get_free_port() }
}

/// Encode a string to Base64.
pub fn encode(text: &str) -> String {
    let text = CString::new(text).unwrap();
    let encoded = unsafe { webui_encode(text.as_ptr()) };
    let result = unsafe { CStr::from_ptr(encoded as _).to_string_lossy().to_string() };
    unsafe { webui_free(encoded as _) };
    result
}

/// Decode a Base64-encoded string.
pub fn decode(text: &str) -> String {
    let text = CString::new(text).unwrap();
    let decoded = unsafe { webui_decode(text.as_ptr()) };
    let result = unsafe { CStr::from_ptr(decoded as _).to_string_lossy().to_string() };
    unsafe { webui_free(decoded as _) };
    result
}

/// Set the SSL/TLS certificate and private key (both in PEM format). If called with empty strings, WebUI generates
/// a self-signed certificate.
///
/// # Remarks
/// This works only with the TLS build of WebUI (webui-2-secure).
pub fn set_tls_certificate(certificate_pem: &str, private_key_pem: &str) -> Result<(), WebUIError> {
    let certificate_pem = CString::new(certificate_pem).unwrap();
    let private_key_pem = CString::new(private_key_pem).unwrap();
    let result = unsafe { webui_set_tls_certificate(certificate_pem.as_ptr(), private_key_pem.as_ptr()) };
    WebUIError::from_bool(result)
}

/// Set a custom logging function to receive WebUI's internal log messages. Useful for debugging or integrating with your
/// own logging system.
pub fn set_logger<F>(callback: F)
where
    F: LoggerHandler,
{
    extern "C" fn shim(level: usize, log: *const c_char, user_data: *mut c_void) {
        let level: LoggerLevel = unsafe { std::mem::transmute(level) };
        let log = unsafe { CStr::from_ptr(log).to_string_lossy().to_string() };
        let callback = unsafe { &*(user_data as *mut Box<dyn LoggerHandler>) };
        let _ = catch_unwind(AssertUnwindSafe(|| callback(level, &log)));
    }
    let user_data: Box<dyn LoggerHandler> = Box::new(callback);
    let user_data = Box::into_raw(Box::new(user_data));
    unsafe {
        webui_set_logger(Some(shim), user_data as _);
    }
}

/// Free all memory resources used by WebUI. Should be called only once at the very end of your application,
/// after wait() returns.
pub unsafe fn clean() {
    webui_clean();
}

/// Get the error code from the most recent WebUI operation that failed.
pub(crate) fn get_last_error_number() -> usize {
    unsafe { webui_get_last_error_number() }
}

/// Get the human-readable error message from the most recent WebUI operation that failed.
pub(crate) fn get_last_error_message() -> String {
    unsafe {
        let message = webui_get_last_error_message();
        CStr::from_ptr(message).to_string_lossy().to_string()
    }
}
