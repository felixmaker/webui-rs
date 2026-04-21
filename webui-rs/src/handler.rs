use crate::{Event, Window};

macro_rules! define_handler {
    (
        $(
            $(#[$attr:meta])*
            $name:ident: ($($arg:ty),*) $(-> $ret:ty)?
        );* $(;)?
    ) => {
        $(
            $(#[$attr])*
            pub trait $name: Fn($($arg),*) $(-> $ret)? + Send + Sync + 'static {}

            impl<T> $name for T where T: Fn($($arg),*) $(-> $ret)? + Send + Sync + 'static {}
        )*
    };
}

define_handler! {
    /// Intercept the close event of a WebView window. Return false from the handler to prevent the window 
    /// from closing; return true to allow it.
    CloseHandler: (&Window) -> bool;

    /// Bind an HTML element click event or a JavaScript function call to a C callback.
    EventHandler: (&Event);

    /// The handler receives the requested filename and must return a complete HTTP response 
    /// (headers + body). Replaces any handler set with set_file_handler_window.
    FileHandlerWindow: (&Window, &str) -> Option<String>;
}
