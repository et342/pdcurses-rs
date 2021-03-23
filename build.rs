
fn main() {
    println!("cargo:rerun-if-changed=PDCurses");
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(all(not(doc), windows)) {
        build();
    }
}

fn build() {
    if cfg!(feature = "unbundled") {
        println!("cargo:rerun-if-env-changed=LIBRARY_PATH");
        println!("cargo:rustc-link-lib=static=pdcurses");

        if let Some(paths) = std::env::var_os("LIBRARY_PATH") {
            for path in std::env::split_paths(&paths) {
                println!("cargo:rustc-link-search=native={}", path.display());
            }
        }
    } else {
        let mut build = cc::Build::new();

        if cfg!(feature = "debug") {
            build.define("PDCDEBUG", None);
        }

        if cfg!(feature = "force_utf8") {
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
    }
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=advapi32");
}

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
