use webui::*;

fn main() -> Result<(), WebUIError> {
    let window = Window::new();
    window.bind("say_hello", |_| println!("Hello, world!"));
    window.show(r#"<html><head><title>Hello, world!</title><script src="webui.js"></script></head><body><button onclick="say_hello()">Hello</button></body></html>"#)?;
    wait();
    Ok(())
}
