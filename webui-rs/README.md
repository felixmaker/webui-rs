# webui-rs

**webui-rs** provides unofficial, lightweight Rust bindings for [WebUI](https://github.com), allowing you to use any web browser or WebView as a GUI for your Rust applications.

> [!NOTE]  
> This is an unofficial implementation. If you are looking for the official crate, please visit [rust-webui](https://github.com/webui-dev/rust-webui).

## Quick Start

```bash
cargo add webui-rs
```

## Example

```rust
use webui::*;

fn main() -> Result<(), WebUIError> {
    let window = Window::new();

    // Bind a Rust closure to a JavaScript function
    window.bind("say_hello", |_| println!("Hello, world!"));

    // Show the window with embedded HTML
    window.show(r#"
        <html>
            <head>
                <title>Hello, world!</title>
                <script src="webui.js"></script>
            </head>
            <body>
                <button onclick="say_hello()">Click Me!</button>
            </body>
        </html>
    "#)?;

    // Wait until all windows are closed
    wait();
    Ok(())
}
```

## Why this crate?

While official bindings exist, **webui-rs** focuses on:

- **Idiomatic Rust**: A more natural API for Rust developers.
- **Simplicity**: Minimal abstraction layers over the C library.

## License

This project is licensed under the MIT License.
