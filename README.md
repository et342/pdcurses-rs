# pdcurses
Windows Console [PDCurses] bindings for Rust.

# Usage

Cargo.toml

```toml
[dependencies]
pdcurses = { git = "https://github.com/et342/pdcurses-rs" }
```

or with a local checkout:

```toml
[dependencies]
pdcurses = { path = "path/to/pdcurses-rs" }
```

or if used for portability in conjunction with [ncurses-rs]:

```toml
[target.'cfg(unix)'.dependencies]
ncurses = { version = "5.99.0", features = ["wide"] }
[target.'cfg(windows)'.dependencies]
pdcurses = { git = "https://github.com/et342/pdcurses-rs", features = ["ncurses_compat"] }
```

main.rs

```rust
// #[cfg(unix)]
// use ncurses::*;
// #[cfg(windows)]
use pdcurses::*;

fn main() {
    initscr();

    let mut h = 0;
    let mut w = 0;
    getmaxyx(stdscr(), &mut h, &mut w);

    let text = "Some text in the middle of the screen (press any key to exit)";
    let text_len = text.len() as i32;

    mv(h/2, w/2 - text_len/2);
    addstr(text);
    refresh();

    getch();
    endwin();
}
```

# Documentation

This is a thin undocumented wrapper around PDCurses. All the relevant documentation of PDCurses functionality can be found in the `docs` subdirectory of the PDCurses [repo][PDCurses].

[PDCurses]: https://github.com/wmcbrine/PDCurses
