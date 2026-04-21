mod context;
mod event;
mod handler;
mod window;

use std::ffi::CString;

pub(crate) use context::*;
pub use event::*;
pub use window::*;

use webui_sys::*;

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

#[repr(i32)] // Replace u32 with the actual type of webui_config
pub(crate) enum Config {
    ShowWaitConnection = 0,
    UiEventBlocking = 1,
    FolderMonitor = 2,
    MultiClient = 3,
    UseCookies = 4,
    AsynchronousResponse = 5,
}
