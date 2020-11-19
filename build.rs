
const PDCLIB_FILES: &[&str] = &[
    "PDCurses/pdcurses/addch.c",
    "PDCurses/pdcurses/addchstr.c",
    "PDCurses/pdcurses/addstr.c",
    "PDCurses/pdcurses/attr.c",
    "PDCurses/pdcurses/beep.c",
    "PDCurses/pdcurses/bkgd.c",
    "PDCurses/pdcurses/border.c",
    "PDCurses/pdcurses/clear.c",
    "PDCurses/pdcurses/color.c",
    "PDCurses/pdcurses/debug.c",
    "PDCurses/pdcurses/delch.c",
    "PDCurses/pdcurses/deleteln.c",
    "PDCurses/pdcurses/getch.c",
    "PDCurses/pdcurses/getstr.c",
    "PDCurses/pdcurses/getyx.c",
    "PDCurses/pdcurses/inch.c",
    "PDCurses/pdcurses/inchstr.c",
    "PDCurses/pdcurses/initscr.c",
    "PDCurses/pdcurses/inopts.c",
    "PDCurses/pdcurses/insch.c",
    "PDCurses/pdcurses/insstr.c",
    "PDCurses/pdcurses/instr.c",
    "PDCurses/pdcurses/kernel.c",
    "PDCurses/pdcurses/keyname.c",
    "PDCurses/pdcurses/mouse.c",
    "PDCurses/pdcurses/move.c",
    "PDCurses/pdcurses/outopts.c",
    "PDCurses/pdcurses/overlay.c",
    "PDCurses/pdcurses/pad.c",
    "PDCurses/pdcurses/panel.c",
    "PDCurses/pdcurses/printw.c",
    "PDCurses/pdcurses/refresh.c",
    "PDCurses/pdcurses/scanw.c",
    "PDCurses/pdcurses/scroll.c",
    "PDCurses/pdcurses/scr_dump.c",
    "PDCurses/pdcurses/slk.c",
    "PDCurses/pdcurses/termattr.c",
    "PDCurses/pdcurses/touch.c",
    "PDCurses/pdcurses/util.c",
    "PDCurses/pdcurses/window.c",
];

const WINCON_FILES: &[&str] = &[
    "PDCurses/wincon/pdcclip.c",
    "PDCurses/wincon/pdcdisp.c",
    "PDCurses/wincon/pdcgetsc.c",
    "PDCurses/wincon/pdckbd.c",
    "PDCurses/wincon/pdcscrn.c",
    "PDCurses/wincon/pdcsetsc.c",
    "PDCurses/wincon/pdcutil.c",
];

fn main() {
    println!("cargo:rerun-if-changed=PDCurses");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(not(doc))] {
        build();
    }
}

fn build() {
    let mut build = cc::Build::new();

    if std::env::var_os("CARGO_FEATURE_DEBUG").is_some() {
        build.define("PDCDEBUG", None);
    }

    if std::env::var_os("CARGO_FEATURE_FORCE_UTF8").is_some() {
        build.define("PDC_FORCE_UTF8", None);
    }

    build
        .warnings(false)
        .extra_warnings(false)
        .define("PDC_RGB", None)
        .define("PDC_WIDE", None)
        .include("PDCurses")
        .files(PDCLIB_FILES)
        .files(WINCON_FILES)
        .compile("pdcurses");

    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=advapi32");
}
