use std::{
    collections::HashMap,
    ffi::{c_char, c_int, CStr, CString},
    os::raw::c_void,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{Arc, Mutex},
};

use webui_sys::*;

use crate::{get_window, handler::*, Event, CONTEXT};

pub(crate) struct WindowInner {
    id: usize,
    handler: WindowHandler,
}

#[derive(Default)]
pub(crate) struct WindowHandler {
    on_close: Mutex<Option<Arc<dyn CloseHandler>>>,
    on_event: Mutex<HashMap<usize, Arc<dyn EventHandler>>>,
    on_file: Mutex<Option<Arc<dyn FileHandlerWindow>>>,
}

#[repr(transparent)]
pub struct Window {
    pub(crate) inner: Arc<WindowInner>,
}

impl Window {
    /// Create a new window object.
    pub fn new() -> Self {
        let id = unsafe { webui_new_window() };
        let window = Arc::new(WindowInner {
            id,
            handler: Default::default(),
        });
        let window_weak = Arc::downgrade(&window);
        CONTEXT.lock().unwrap().insert(id, window_weak);
        Self { inner: window }
    }

    /// Get the id of inner window.
    pub(crate) fn id(&self) -> usize {
        self.inner.id
    }

    /// Show a window using embedded HTML, a URL, a local file, or a local folder. If the window is already open, it will
    /// be refreshed. This refreshes all clients in multi-client mode.
    ///
    /// # Remarks
    /// To use only a specific browser please use show_browser()
    /// To use only WebView please use show_webview()
    pub fn show(&self, content: &str) -> bool {
        let content = CString::new(content).unwrap();
        unsafe { webui_show(self.id(), content.as_ptr()) }
    }

    /// Show a window using a specific web browser.
    ///
    /// # Remarks
    /// It's recommended to use ChromiumBased browser.
    /// On macOS, the browser's icon may still appear in the Dock after exit. We recommend using show_webview on macOS to
    /// avoid this.
    pub fn show_browser(&self, content: &str, browser: Browser) {
        let content = CString::new(content).unwrap();
        unsafe { webui_show_browser(self.id(), content.as_ptr(), browser as _) };
    }

    /// Show a WebView window using embedded HTML, a URL, or a local file. If the window is already open, it will be refreshed.
    ///
    /// # Remarks
    /// WebUI's primary focus is using web browsers as GUI, but if you need to use WebView instead of a web browser, then you
    /// can use this API, which was added to WebUI starting from v2.5.
    ///
    /// - Windows Dependencies: WebView2, and WebView2Loader.dll.
    /// - Linux Dependencies: WebKit GTK v3.
    /// - macOS Dependencies: WebKit (WKWebView).
    pub fn show_webview(&self, content: &str) {
        let content = CString::new(content).unwrap();
        unsafe { webui_show_wv(self.id(), content.as_ptr()) };
    }

    /// Start only the local web server without opening a browser window, and return the URL. Useful for headless web app
    /// scenarios.
    pub fn start_server(&self, content: &str) -> String {
        let content = CString::new(content).unwrap();
        let url = unsafe { webui_start_server(self.id(), content.as_ptr()) };
        unsafe { CStr::from_ptr(url).to_string_lossy().to_string() }
    }

    /// Close the window. The window object will still exist and can be shown again later.
    pub fn close(&self) {
        unsafe { webui_close(self.id()) }
    }

    /// Check if the window is still running.
    pub fn is_shown(&self) -> bool {
        unsafe { webui_is_shown(self.id()) }
    }

    /// Bring the window to the front and give it keyboard focus.
    pub fn focus(&self) {
        unsafe { webui_focus(self.id()) }
    }

    /// Minimize the WebView window.
    pub fn minimize(&self) {
        unsafe { webui_minimize(self.id()) }
    }

    /// Maximize a WebView window.
    pub fn maximize(&self) {
        unsafe { webui_maximize(self.id()) }
    }

    /// Set the window in Kiosk mode (Full screen).
    pub fn set_kiosk(&self, status: bool) {
        unsafe { webui_set_kiosk(self.id(), status) }
    }

    /// Set the window size.
    pub fn set_size(&self, width: u32, height: u32) {
        unsafe { webui_set_size(self.id(), width, height) }
    }

    /// Set the minimum allowed window size. The user will not be able to resize the window smaller than this.
    ///
    /// # Remarks
    /// Currently works on Windows only.
    pub fn set_minimum_size(&self, width: u32, height: u32) {
        unsafe { webui_set_minimum_size(self.id(), width, height) }
    }

    /// Set the window position.
    pub fn set_position(&self, x: u32, y: u32) {
        unsafe { webui_set_position(self.id(), x, y) }
    }

    /// Center the window on the screen.
    ///
    /// # Remarks
    /// Call this before show() for best results. Works better with WebView.
    pub fn set_center(&self) {
        unsafe { webui_set_center(self.id()) }
    }

    /// Start the window in hidden mode. The window will be running but not visible.
    ///
    /// # Remarks
    /// Should be called before show().
    pub fn set_hide(&self, hide: bool) {
        unsafe { webui_set_hide(self.id(), hide) }
    }

    /// Remove the window frame and title bar (borderless/frameless mode).
    ///
    /// # Remarks
    /// Works with WebView windows only.
    pub fn set_frameless(&self, frameless: bool) {
        unsafe { webui_set_frameless(self.id(), frameless) }
    }

    /// Enable or disable window background transparency.
    ///
    /// # Remarks
    /// Works with WebView windows only. The web content must also use a transparent/semi-transparent background for this
    /// to be visible.
    pub fn set_transparent(&self, transparent: bool) {
        unsafe { webui_set_transparent(self.id(), transparent) }
    }

    /// Control whether the user can resize the window.
    ///
    /// # Remarks
    /// Works with WebView windows only.
    pub fn set_resizable(&self, resizable: bool) {
        unsafe { webui_set_resizable(self.id(), resizable) }
    }

    /// Set the default embedded HTML favicon. The icon is served automatically by WebUI's built-in server.
    pub fn set_icon(&self, icon: &str, icon_type: &str) {
        let icon = CString::new(icon).unwrap();
        let icon_type = CString::new(icon_type).unwrap();
        unsafe { webui_set_icon(self.id(), icon.as_ptr(), icon_type.as_ptr()) }
    }

    /// Mark the window as supporting high-contrast mode. Use this together with CSS to build a better high-contrast theme.
    pub fn set_high_contrast(&self, high_contrast: bool) {
        unsafe { webui_set_high_contrast(self.id(), high_contrast) }
    }

    /// Check if the operating system is currently using a high-contrast theme.
    pub fn is_high_contrast(&self) -> bool {
        unsafe { webui_is_high_contrast() }
    }

    /// Add custom command-line parameters to the browser launch command. This can be used, for example, to enable remote
    /// debugging.
    pub fn set_custom_parameters(&self, params: &str) {
        let params = CString::new(params).unwrap();
        unsafe {
            webui_set_custom_parameters(self.id(), params.as_ptr() as _);
        }
    }

    /// Set a callback to intercept the close event of a WebView window. Return false from the handler to prevent the window
    /// from closing; return true to allow it.
    pub fn set_close_handler_webview<F>(&self, callback: F)
    where
        F: CloseHandler,
    {
        extern "C" fn shim(id: usize) -> bool {
            let Some(window) = get_window(id) else {
                return true;
            };
            let callback = window.inner.handler.on_close.lock().unwrap().clone();
            callback.map_or(true, |callback| {
                std::panic::catch_unwind(AssertUnwindSafe(|| callback(&window))).unwrap_or(true)
            })
        }
        *self.inner.handler.on_close.lock().unwrap() = Some(Arc::new(callback));
        unsafe { webui_set_close_handler_wv(self.id(), Some(shim)) }
    }

    /// Navigate all connected clients of a window to a specific URL.
    pub fn navigate(&self, url: &str) {
        let url = CString::new(url).unwrap();
        unsafe { webui_navigate(self.id(), url.as_ptr()) }
    }

    /// Get the current URL of a running window's web server.
    ///
    /// # Remarks
    /// By default, WebUI only allows access to this URL from localhost.
    pub fn get_url(&self) -> String {
        let url = unsafe { webui_get_url(self.id()) };
        let url = unsafe { CStr::from_ptr(url) };
        url.to_string_lossy().to_string()
    }

    /// Allow the window's web server URL to be accessible from external devices on the network. By default, WebUI
    /// only allows access from localhost.
    pub fn set_public(&self, status: bool) {
        unsafe { webui_set_public(self.id(), status) }
    }

    /// Bind an HTML element click event or a JavaScript function call to a C callback. Use an empty element name to catch all events
    /// from the window.
    pub fn bind<F>(&self, function: &str, callback: F)
    where
        F: EventHandler,
    {
        let function = CString::new(function).unwrap();
        extern "C" fn shim(event: *mut webui_event_t) {
            let event = Event { inner: event };
            let window = event.get_window();
            let bind_id = event.get_bind_id();

            if let Some(window) = window {
                let callback = window
                    .inner
                    .handler
                    .on_event
                    .lock()
                    .unwrap()
                    .get(&bind_id)
                    .cloned();
                if let Some(callback) = callback {
                    let _ = catch_unwind(AssertUnwindSafe(|| callback(&event)));
                }
            }
        }
        let id = unsafe { webui_bind(self.id(), function.as_ptr(), Some(shim)) };
        self.inner
            .handler
            .on_event
            .lock()
            .unwrap()
            .insert(id, Arc::new(callback));
    }

    /// Control whether UI events from this window are processed one at a time in a single blocking thread (true), or each in a new
    /// non-blocking thread (false). Applies to this window only. Use set_config(ui_event_blocking, ...) to apply to all windows.
    ///
    /// # Remarks
    /// When set to true, the script() API will not return a response until the current event callback has finished.
    pub fn set_event_blocking(&self, event_blocking: bool) {
        unsafe { webui_set_event_blocking(self.id(), event_blocking) };
    }

    /// Set the maximum time in seconds to wait for the browser window to connect after calling show(). A value of 0 means wait forever.
    pub fn set_timeout(&self, second: usize) {
        unsafe { webui_set_timeout(second) };
    }

    /// Run JavaScript and get the response back. Works in single client mode. Make sure your buffer is large enough to hold the response.
    pub fn script(&self, script: &str, timeout: usize, capacity: usize) -> Option<String> {
        let mut buffer: Vec<c_char> = Vec::with_capacity(capacity);
        let script = CString::new(script).unwrap();
        unsafe {
            webui_script(
                self.id(),
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

    /// Choose between Deno, Bun, and Nodejs as the runtime for .js and .ts files served by the web server.
    pub fn set_runtime(&self, runtime: Runtime) {
        unsafe { webui_set_runtime(self.id(), runtime as _) }
    }

    /// Send raw binary data to a JavaScript function in the UI. Sends to all connected clients.
    pub fn send_raw<T>(&self, function: &str, data: T)
    where
        T: Into<Vec<u8>>,
    {
        let function = CString::new(function).unwrap();
        let data = data.into().into_boxed_slice();
        unsafe { webui_send_raw(self.id(), function.as_ptr(), data.as_ptr() as _, data.len()) }
    }

    /// Set a custom handler to serve files. The handler receives the requested filename and must return a complete HTTP response
    /// (headers + body). Replaces any handler set with set_file_handler_window.
    pub fn set_file_handler<F>(&self, callback: F)
    where
        F: FileHandlerWindow,
    {
        extern "C" fn shim(
            window: usize,
            filename: *const c_char,
            length: *mut c_int,
        ) -> *const c_void {
            let Some(window) = get_window(window) else {
                return std::ptr::null();
            };
            let filename = unsafe { CStr::from_ptr(filename).to_string_lossy().to_string() };
            let callback = window
                .inner
                .handler
                .on_file
                .lock()
                .unwrap()
                .as_ref()
                .cloned();
            let response = callback.and_then(|callback| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    callback(&window, &filename)
                }))
                .ok()
            });
            if let Some(Some(data)) = response {
                let len = data.len();
                unsafe {
                    let ptr = webui_malloc(len);
                    if !ptr.is_null() {
                        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut u8, len);
                        *length = len as c_int;
                        return ptr;
                    }
                }
            }
            std::ptr::null()
        }

        *self.inner.handler.on_file.lock().unwrap() = Some(Arc::new(callback));

        unsafe {
            webui_set_file_handler_window(self.id(), Some(shim));
        }
    }

    /// Set the web-server root folder path for a specific window.
    pub fn set_root_folder(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe { webui_set_root_folder(self.id(), path.as_ptr()) }
    }

    /// Get the recommended web browser ID to use for this window. If a browser is already in use, returns that browser's ID.
    pub fn get_best_browser(&self) -> Browser {
        let browser = unsafe { webui_get_best_browser(self.id()) };
        Browser::from_id(browser as i32)
    }

    /// Set the web browser profile to use. An empty name and path uses the default user profile.
    pub fn set_profile(&self, name: &str, path: &str) {
        let name = CString::new(name).unwrap();
        let path = CString::new(path).unwrap();
        unsafe { webui_set_profile(self.id(), name.as_ptr(), path.as_ptr()) }
    }

    /// Set the web browser proxy server.
    ///
    /// # Remarks
    /// Must be called before show().
    pub fn set_proxy(&self, proxy_server: &str) {
        let proxy_server = CString::new(proxy_server).unwrap();
        unsafe { webui_set_proxy(self.id(), proxy_server.as_ptr()) }
    }

    /// Delete a specific window web-browser local folder profile.
    ///
    /// # Remarks
    /// Recommended to call after all windows are closed, before clean().
    /// This can break functionality of other windows using the same browser profile.
    pub fn delete_profile(&self) {
        unsafe { webui_delete_profile(self.id()) }
    }

    /// Get the network port used by the running window's web server. Useful for constructing the webui.js URL when
    /// integrating with an external server.
    pub fn get_port(&self) -> usize {
        unsafe { webui_get_port(self.id()) }
    }

    /// Set a specific network port for the window's web server. Useful when integrating with an external web server like NGINX.
    pub fn set_port(&self, port: usize) -> bool {
        unsafe { webui_set_port(self.id(), port) }
    }

    /// Get the process ID of the backend application (the parent process). The web browser may create additional child processes.
    pub fn get_parent_process_id(&self) -> usize {
        unsafe { webui_get_parent_process_id(self.id()) }
    }

    /// Get the process ID of the browser window child process. In WebView mode, returns the parent PID because backend and WebView 
    /// run in the same process.
    pub fn get_child_process_id(&self) -> usize {
        unsafe { webui_get_child_process_id(self.id()) }
    }

    /// Get the native window handle. On Windows returns HWND (works with both WebView and web browser). On Linux returns GtkWindow* 
    /// (WebView only).
    pub fn get_window_handler(&self) -> *mut c_void {
        unsafe { webui_get_hwnd(self.id()) }
    }

    /// Run JavaScript without waiting for the response.
    pub fn run(&self, script: &str) {
        let script = CString::new(script).unwrap();
        unsafe { webui_run(self.id(), script.as_ptr()) }
    }
}

impl Drop for WindowInner {
    fn drop(&mut self) {
        let mut context = CONTEXT.lock().unwrap();
        unsafe { webui_destroy(self.id) };
        context.remove(&self.id);
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Runtime {
    /// No runtime
    None = 0,
    /// Deno runtime
    Deno = 1,
    /// Node.js runtime
    NodeJS = 2,
    /// Bun runtime
    Bun = 3,
}

#[repr(i32)]
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
}
