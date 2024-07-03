@echo off

bindgen -o webui-sys\src\webui.rs webui-sys\webui\include\webui.h ^
    --allowlist-function webui.* --allowlist-type webui.* ^
    --no-layout-tests --no-prepend-enum-name ^
    -- --target=i686-pc-windows-msvc
