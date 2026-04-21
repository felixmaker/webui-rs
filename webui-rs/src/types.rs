#[repr(i32)]
pub enum Runtime {
    None,
    Deno,
    NodeJS,
    Bun,
}

#[repr(usize)]
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

#[repr(usize)]
pub enum LoggerLevel {
    Debug,
    Info,
    Error,
}

#[repr(usize)]
pub enum EventType {
    Disconnected,
    Connected,
    MouseClick,
    Navigation,
    Callback,
}
