/// The Runtime type.
#[repr(i32)]
#[allow(missing_docs)]
pub enum Runtime {
    None,
    Deno,
    NodeJS,
    Bun,
}

/// The Browser type.
#[repr(usize)]
#[allow(missing_docs)]
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
    Webview,
}

#[repr(i32)]
pub(crate) enum Config {
    ShowWaitConnection,
    UiEventBlocking,
    FolderMonitor,
    MultiClient,
    UseCookies,
    AsynchronousResponse,
}

/// The logger level.
#[repr(usize)]
#[allow(missing_docs)]
pub enum LoggerLevel {
    Debug,
    Info,
    Error,
}

/// The event type.
#[repr(usize)]
pub enum EventType {
    /// Window disconnection event.
    Disconnected = 0,
    /// Window connection event.
    Connected,
    /// Mouse click event.
    MouseClick,
    /// Window navigation event.
    Navigation,
    /// Custom function call event.
    Callback,
}
