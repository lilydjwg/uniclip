Unify the clipboard operations for X11 and Wayland
====

This program calls `xclip` if `$XDG_SESSION_TYPE` is `x11` and `wl-copy` / `wl-paste` if it's `wayland`.

Use `uniclip --help` to see command line options. It's mostly based on `xsel`.
