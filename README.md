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

In the spirit of [ncurses-rs], this is a very thin wrapper around PDCurses, so all the relevant documentation can be found in the `docs` subdirectory of the PDCurses [repo][PDCurses].

Although this means that this package is not documented, html-based reference documentation can still be built locally by running the following command either in the root of a local checkout of this repo, or in the root of a package that depends on this one:

```
cargo doc -p pdcurses --no-deps --open
```

Normally, documentation will be built under the `target/doc/pdcurses` directory and can be accessed at `target/doc/pdcurses/index.html`. <br>
Flag `--open` will tell cargo to attempt to open generated documentation in a default web-browser.


[ncurses-rs]: https://github.com/jeaye/ncurses-rs
[PDCurses]: https://github.com/wmcbrine/PDCurses
