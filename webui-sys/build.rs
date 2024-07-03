fn main() {
    cc::Build::new()
        .define("NO_SSL", "")
        .define("NDEBUG", "")
        .define("NO_CACHING", "")
        .define("NO_CGI", "")
        .define("USE_WEBSOCKET", "")
        .include("webui/src/civetweb")
        .include("webui/src/webview")
        .include("webui/include")
        .file("webui/src/civetweb/civetweb.c")
        .file("webui/src/webui.c")
        .compile("webui");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        println!("cargo::rustc-link-lib=user32");
        println!("cargo::rustc-link-lib=shell32");
        println!("cargo::rustc-link-lib=ole32");
    }
}
