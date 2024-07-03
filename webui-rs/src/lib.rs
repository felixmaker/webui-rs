use std::ffi::{CStr, CString};

use webui_sys::*;

#[repr(transparent)]
pub struct Window {
    inner: usize,
}

impl Window {
    /// Create a new WebUI window object.
    pub fn new() -> Self {
        let new_window = unsafe { webui_new_window() };
        Self { inner: new_window }
    }

    /// Create a new WebUI window object using a specified window id.
    ///
    /// Note: The window id should be > 0, and < WEBUI_MAX_IDS
    pub fn new_with_id(window_id: usize) -> Self {
        let new_window = unsafe { webui_new_window_id(window_id) };
        Self { inner: new_window }
    }

    /// Get a free window number.
    pub fn get_free_id() -> usize {
        let free_id = unsafe { webui_get_new_window_id() };
        free_id
    }

    /// Get the recommended web browser ID to use. If you are already using one, this function will return the same ID.
    pub fn get_best_browser(&self) -> Browser {
        let browser = unsafe { webui_get_best_browser(self.inner) };
        Browser::from_id(browser as i32)
    }

    /// Show a window using embedded HTML, or a file. If the window is already open, it will be refreshed.
    pub fn show(&self, content: &str) -> bool {
        let content = CString::new(content).unwrap();
        unsafe { webui_show(self.inner, content.as_ptr()) }
    }

    /// Show a window using a specific web browser.
    pub fn show_browser(&self, content: &str, browser: Browser) {
        let content = CString::new(content).unwrap();
        unsafe { webui_show_browser(self.inner, content.as_ptr(), browser.to_id() as usize) };
    }

    /// Show a WebView window using embedded HTML, or a file. If the window is already open, it will be refreshed.
    pub fn show_webview(&self, content: &str) {
        let content = CString::new(content).unwrap();
        unsafe { webui_show_wv(self.inner, content.as_ptr()) };
    }

    /// Set the window in Kiosk mode (Full screen).
    pub fn set_kiosk(&self, status: bool) {
        unsafe { webui_set_kiosk(self.inner, status) }
    }

    /// Close the window.
    pub fn close(&self) {
        unsafe { webui_close(self.inner) }
    }

    /// Close the window and free all related memory resources.
    pub fn destroy(&self) {
        unsafe { webui_destroy(self.inner) }
    }

    /// Set the web-server root folder path.
    pub fn set_root_folder(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe { webui_set_root_folder(self.inner, path.as_ptr()) }
    }

    /// Check if the window is still running.
    pub fn is_shown(&self) -> bool {
        unsafe { webui_is_shown(self.inner) }
    }

    /// Set the default embedded HTML favicon.
    pub fn set_icon(&self, icon: &str, icon_type: &str) {
        let icon = CString::new(icon).unwrap();
        let icon_type = CString::new(icon_type).unwrap();
        unsafe { webui_set_icon(self.inner, icon.as_ptr(), icon_type.as_ptr()) }
    }

    /// Set a window in hidden mode.
    pub fn set_hide(&self, status: bool) {
        unsafe { webui_set_hide(self.inner, status) }
    }

    /// Set the window size.
    pub fn set_size(&self, width: u32, height: u32) {
        unsafe { webui_set_size(self.inner, width, height) }
    }

    /// Set the window position.
    pub fn set_position(&self, x: u32, y: u32) {
        unsafe { webui_set_position(self.inner, x, y) }
    }

    /// Set the web browser profile to use. An empty name and path means the default user profile.
    pub fn set_profile(&self, name: &str, path: &str) {
        let name = CString::new(name).unwrap();
        let path = CString::new(path).unwrap();
        unsafe { webui_set_profile(self.inner, name.as_ptr(), path.as_ptr()) }
    }

    /// Set the web browser proxy server to use.
    ///
    /// Note: Need to be called before show().
    pub fn set_proxy(&self, proxy_server: &str) {
        let proxy_server = CString::new(proxy_server).unwrap();
        unsafe { webui_set_proxy(self.inner, proxy_server.as_ptr()) }
    }

    /// Get current URL of a running window.
    ///
    /// Note: By default WebUI allow access to the URL of a window only from localhost.
    pub fn get_url(&self) -> String {
        let url = unsafe { webui_get_url(self.inner) };
        let url = unsafe { CStr::from_ptr(url) };
        url.to_string_lossy().to_string()
    }

    /// Allow a specific window address (URL) to be accessible from any public network.
    /// By default WebUI allow access to the URL of a window only from localhost.
    pub fn set_public(&self, status: bool) {
        unsafe { webui_set_public(self.inner, status) }
    }

    /// Navigate to a specific URL.
    pub fn navigate(&self, url: &str) {
        let url = CString::new(url).unwrap();
        unsafe { webui_navigate(self.inner, url.as_ptr()) }
    }

    /// Delete a specific window web-browser local folder profile.
    ///
    /// Note: It's recommended to be called when program exit, and after all windows are closed.
    ///
    /// This can break functionality of other running windows if using the same web-browser profile.
    pub fn delete_profile(&self) {
        unsafe { webui_delete_profile(self.inner) }
    }

    /// Get the ID of the parent process (The web browser may re-create another new process).
    pub fn get_parent_process_id(&self) -> usize {
        unsafe { webui_get_parent_process_id(self.inner) }
    }

    /// Get the ID of the last child process (The web browser may re-create other child process).
    pub fn get_child_process_id(&self) -> usize {
        unsafe { webui_get_child_process_id(self.inner) }
    }

    /// Set a custom web-server network port to be used by WebUI.
    /// This can be useful to determine the HTTP link of `webui.js`in case
    /// you are trying to use WebUI with an external web-server like NGNIX.
    pub fn set_port(&self, port: usize) -> bool {
        unsafe { webui_set_port(self.inner, port) }
    }

    /// Control if UI events comming from this window should be processed one a time in a single blocking thread True,
    /// or process every event in a new non-blocking thread False. This update single window.
    /// You can use set_config() to update all windows.
    ///
    /// Note: If this is set to True, the API script() won't return any response until this current event is finished.
    pub fn set_event_blocking(&self, status: bool) {
        unsafe { webui_set_event_blocking(self.inner, status) }
    }

    /// Run JavaScript without waiting for the response.
    pub fn run(&self, script: &str) {
        let script = CString::new(script).unwrap();
        unsafe { webui_run(self.inner, script.as_ptr()) }
    }

    // /// Bind an HTML element event with a function. Empty element means all events.
    // pub fn bind(&self, element_id: &str, event: &Event) {

    // }
}

pub enum Browser {
    NoBrowser,
    AnyBrowser,
    Chrome,
    Firefox,
    Edge,
    Safari,
    Chromium,
    Opera,
    Brave,
    Vivaldi,
    Epic,
    Yandex,
    ChromiumBased,
}

impl Browser {
    #[allow(non_upper_case_globals)]
    fn from_id(id: webui_browser) -> Self {
        match id {
            NoBrowser => Self::NoBrowser,
            AnyBrowser => Self::AnyBrowser,
            Chrome => Self::Chrome,
            Firefox => Self::Firefox,
            Edge => Self::Edge,
            Safari => Self::Safari,
            Chromium => Self::Chromium,
            Opera => Self::Opera,
            Brave => Self::Brave,
            Vivaldi => Self::Vivaldi,
            Epic => Self::Epic,
            Yandex => Self::Yandex,
            ChromiumBased => Self::ChromiumBased,
            _ => unimplemented!(),
        }
    }

    fn to_id(&self) -> webui_browser {
        match self {
            Self::NoBrowser => NoBrowser,
            Self::AnyBrowser => AnyBrowser,
            Self::Chrome => Chrome,
            Self::Firefox => Firefox,
            Self::Edge => Edge,
            Self::Safari => Safari,
            Self::Chromium => Chromium,
            Self::Opera => Opera,
            Self::Brave => Brave,
            Self::Vivaldi => Vivaldi,
            Self::Epic => Epic,
            Self::Yandex => Yandex,
            Self::ChromiumBased => ChromiumBased,
        }
    }
}
