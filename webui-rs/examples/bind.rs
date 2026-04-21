use webui_rs::*;

fn main() {
    let window = Window::new();

    window.bind("say_hello", |event| {
        println!("Say Hello");
        if let Some(window) = event.window() {
            println!("URL: {}", window.get_url());
            window.bind("say_hello", |_| {
                println!("Say Hello, No longer")
            });
        }
    });

    window.show_browser(r#"
    <html>
    <head>
    <title>Hello, world!</title>
    <script src="webui.js"></script>     
    </head>
    <body>
    <input id="input1"></input>
    <input id="input2" disabled></input>
    <button onclick="say_hello()">Hello</button>
    <a href="https://www.whatismybrowser.com/detect/what-http-headers-is-my-browser-sending/" target="_blank">Whatismybrowser</a>
    <script>
    
    </script>
    </body>
    </html>        
    "#, Browser::Edge);

    wait();
}
