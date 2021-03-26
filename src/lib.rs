
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(all(not(doc), not(windows)))]
compile_error!("These PDCurses bindings support only Windows.");

use libc::FILE;

fn cstr(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}

fn wstr(s: &[u16]) -> Vec<u16> {
    let mut out = Vec::with_capacity(s.len() + 1);
    out.extend_from_slice(s);
    out.push(0);
    out
}

fn chstr(s: &[chtype]) -> Vec<chtype> {
    let mut out = Vec::with_capacity(s.len() + 1);
    out.extend_from_slice(s);
    out.push(0);
    out
}

unsafe fn wcstr(ptr: *const u16) -> Vec<u16> {
    core::slice::from_raw_parts(ptr, libc::wcslen(ptr)).to_vec()
}

pub mod sys;
#[doc(inline)]
pub use sys::{
    wint_t,

    PDC_BUILD,
    PDC_VER_MAJOR,
    PDC_VER_MINOR,
    PDC_VERDOT,

    FALSE,
    TRUE,
    ERR,
    OK,

    chtype,
    cchar_t,
    attr_t,

    PDC_VERSION,
    PDC_VFLAG_DEBUG,
    PDC_VFLAG_WIDE,
    PDC_VFLAG_UTF8,
    PDC_VFLAG_DLL,
    PDC_VFLAG_RGB,

    mmask_t,
    MOUSE_STATUS,
    mouse_constants::*,
    MEVENT,

    MOUSE_X_POS,
    MOUSE_Y_POS,
    A_BUTTON_CHANGED,
    MOUSE_MOVED,
    MOUSE_POS_REPORT,
    BUTTON_CHANGED,
    BUTTON_STATUS,
    MOUSE_WHEEL_UP,
    MOUSE_WHEEL_DOWN,
    MOUSE_WHEEL_LEFT,
    MOUSE_WHEEL_RIGHT,

    PDC_PAIR,

    attribute_constants,
    attribute_constants::*,

    PDC_ACS,
    ACS_ULCORNER,
    ACS_LLCORNER,
    ACS_URCORNER,
    ACS_LRCORNER,
    ACS_RTEE,
    ACS_LTEE,
    ACS_BTEE,
    ACS_TTEE,
    ACS_HLINE,
    ACS_VLINE,
    ACS_PLUS,
    ACS_S1,
    ACS_S9,
    ACS_DIAMOND,
    ACS_CKBOARD,
    ACS_DEGREE,
    ACS_PLMINUS,
    ACS_BULLET,
    ACS_LARROW,
    ACS_RARROW,
    ACS_DARROW,
    ACS_UARROW,
    ACS_BOARD,
    ACS_LANTERN,
    ACS_BLOCK,
    ACS_S3,
    ACS_S7,
    ACS_LEQUAL,
    ACS_GEQUAL,
    ACS_PI,
    ACS_NEQUAL,
    ACS_STERLING,
    ACS_BSSB,
    ACS_SSBB,
    ACS_BBSS,
    ACS_SBBS,
    ACS_SBSS,
    ACS_SSSB,
    ACS_SSBS,
    ACS_BSSS,
    ACS_BSBS,
    ACS_SBSB,
    ACS_SSSS,

    WACS_ULCORNER,
    WACS_LLCORNER,
    WACS_URCORNER,
    WACS_LRCORNER,
    WACS_RTEE,
    WACS_LTEE,
    WACS_BTEE,
    WACS_TTEE,
    WACS_HLINE,
    WACS_VLINE,
    WACS_PLUS,
    WACS_S1,
    WACS_S9,
    WACS_DIAMOND,
    WACS_CKBOARD,
    WACS_DEGREE,
    WACS_PLMINUS,
    WACS_BULLET,
    WACS_LARROW,
    WACS_RARROW,
    WACS_DARROW,
    WACS_UARROW,
    WACS_BOARD,
    WACS_LANTERN,
    WACS_BLOCK,
    WACS_S3,
    WACS_S7,
    WACS_LEQUAL,
    WACS_GEQUAL,
    WACS_PI,
    WACS_NEQUAL,
    WACS_STERLING,
    WACS_BSSB,
    WACS_SSBB,
    WACS_BBSS,
    WACS_SBBS,
    WACS_SBSS,
    WACS_SSSB,
    WACS_SSBS,
    WACS_BSSS,
    WACS_BSBS,
    WACS_SBSB,
    WACS_SSSS,

    color_constants,
    color_constants::*,

    key_constants,
    key_constants::*,

    KEY_F,

    PDC_CLIP_SUCCESS,
    PDC_CLIP_ACCESS_ERROR,
    PDC_CLIP_EMPTY,
    PDC_CLIP_MEMORY_ERROR,

    PDC_KEY_MODIFIER_SHIFT,
    PDC_KEY_MODIFIER_CONTROL,
    PDC_KEY_MODIFIER_ALT,
    PDC_KEY_MODIFIER_NUMLOCK,
};

#[cfg(feature = "ncurses_compat")]
#[doc(inline)]
pub use ncurses_compat::*;
#[cfg(feature = "ncurses_compat")]
mod ncurses_compat {
    // Compiler doesn't get the hint that the items below are public and are not dead code,
    // so we hint that ourselves.
    #![allow(dead_code)]

    use super::*;

    pub const fn A_NORMAL     () -> chtype { sys::A_NORMAL }
    pub const fn A_ALTCHARSET () -> chtype { sys::A_ALTCHARSET }
    pub const fn A_RIGHT      () -> chtype { sys::A_RIGHT }
    pub const fn A_LEFT       () -> chtype { sys::A_LEFT }
    pub const fn A_ITALIC     () -> chtype { sys::A_ITALIC }
    pub const fn A_UNDERLINE  () -> chtype { sys::A_UNDERLINE }
    pub const fn A_REVERSE    () -> chtype { sys::A_REVERSE }
    pub const fn A_BLINK      () -> chtype { sys::A_BLINK }
    pub const fn A_BOLD       () -> chtype { sys::A_BOLD }
    pub const fn A_ATTRIBUTES () -> chtype { sys::A_ATTRIBUTES }
    pub const fn A_CHARTEXT   () -> chtype { sys::A_CHARTEXT }
    pub const fn A_COLOR      () -> chtype { sys::A_COLOR }
    pub const fn A_LEFTLINE   () -> chtype { sys::A_LEFTLINE }
    pub const fn A_RIGHTLINE  () -> chtype { sys::A_RIGHTLINE }
    pub const fn A_STANDOUT   () -> chtype { sys::A_STANDOUT }
    pub const fn A_DIM        () -> chtype { sys::A_DIM }
    pub const fn A_INVIS      () -> chtype { sys::A_INVIS }
    pub const fn A_PROTECT    () -> chtype { sys::A_PROTECT }
    pub const fn A_HORIZONTAL () -> chtype { sys::A_HORIZONTAL }
    pub const fn A_LOW        () -> chtype { sys::A_LOW }
    pub const fn A_TOP        () -> chtype { sys::A_TOP }
    pub const fn A_VERTICAL   () -> chtype { sys::A_VERTICAL }
    pub const fn CHR_MSK      () -> chtype { sys::CHR_MSK }
    pub const fn ATR_MSK      () -> chtype { sys::ATR_MSK }
    pub const fn ATR_NRM      () -> chtype { sys::ATR_NRM }
    pub const fn WA_NORMAL    () -> chtype { sys::WA_NORMAL }
    pub const fn WA_ALTCHARSET() -> chtype { sys::WA_ALTCHARSET }
    pub const fn WA_BLINK     () -> chtype { sys::WA_BLINK }
    pub const fn WA_BOLD      () -> chtype { sys::WA_BOLD }
    pub const fn WA_DIM       () -> chtype { sys::WA_DIM }
    pub const fn WA_INVIS     () -> chtype { sys::WA_INVIS }
    pub const fn WA_ITALIC    () -> chtype { sys::WA_ITALIC }
    pub const fn WA_LEFT      () -> chtype { sys::WA_LEFT }
    pub const fn WA_PROTECT   () -> chtype { sys::WA_PROTECT }
    pub const fn WA_REVERSE   () -> chtype { sys::WA_REVERSE }
    pub const fn WA_RIGHT     () -> chtype { sys::WA_RIGHT }
    pub const fn WA_STANDOUT  () -> chtype { sys::WA_STANDOUT }
    pub const fn WA_UNDERLINE () -> chtype { sys::WA_UNDERLINE }
    pub const fn WA_HORIZONTAL() -> chtype { sys::WA_HORIZONTAL }
    pub const fn WA_LOW       () -> chtype { sys::WA_LOW }
    pub const fn WA_TOP       () -> chtype { sys::WA_TOP }
    pub const fn WA_VERTICAL  () -> chtype { sys::WA_VERTICAL }
    pub const fn WA_ATTRIBUTES() -> chtype { sys::WA_ATTRIBUTES }
}

/// Handle to a `WINDOW` structure.
pub type WINDOW = *mut sys::WINDOW;
/// Handle to a `SCREEN` structure.
pub type SCREEN = *mut sys::SCREEN;


pub fn LINES() -> i32 {
    unsafe { sys::LINES }
}

pub fn COLS() -> i32 {
    unsafe { sys::COLS }
}

pub fn stdscr() -> WINDOW {
    unsafe { sys::stdscr }
}

pub fn curscr() -> WINDOW {
    unsafe { sys::curscr }
}

pub fn SP() -> SCREEN {
    unsafe { sys::SP }
}

pub fn Mouse_status() -> MOUSE_STATUS {
    unsafe { sys::Mouse_status }
}

pub fn COLORS() -> i32 {
    unsafe { sys::COLORS }
}

pub fn COLOR_PAIRS() -> i32 {
    unsafe { sys::COLOR_PAIRS }
}

pub fn TABSIZE() -> i32 {
    unsafe { sys::TABSIZE }
}

pub fn acs_map() -> *mut chtype {
    unsafe { sys::acs_map }
}

pub fn ttytype() -> *mut libc::c_char {
    unsafe { sys::ttytype }
}


pub fn addch(ch: chtype) -> i32 {
    unsafe { sys::addch(ch) }
}

// pub fn addchnstr(ch: &[chtype], n: usize) -> i32 {
//     addchstr(&ch[..n])
// }

pub fn addchstr(ch: &[chtype]) -> i32 {
    unsafe { sys::addchstr(chstr(ch).as_ptr()) }
}

// pub fn addnstr(s: &str, n: usize) -> i32 {
//     addstr(&s[..n])
// }

pub fn addstr(s: &str) -> i32 {
    unsafe { sys::addstr(cstr(s).as_ptr()) }
}

pub fn attroff(attrs: chtype) -> i32 {
    unsafe { sys::attroff(attrs) }
}

pub fn attron(attrs: chtype) -> i32 {
    unsafe { sys::attron(attrs) }
}

pub fn attrset(attrs: chtype) -> i32 {
    unsafe { sys::attrset(attrs) }
}

pub fn attr_get(attrs: &mut attr_t, color_pair: &mut i16) -> i32 {
    unsafe { sys::attr_get(attrs, color_pair, core::ptr::null_mut()) }
}

pub fn attr_off(attrs: attr_t) -> i32 {
    unsafe { sys::attr_off(attrs, core::ptr::null_mut()) }
}

pub fn attr_on(attrs: attr_t) -> i32 {
    unsafe { sys::attr_on(attrs, core::ptr::null_mut()) }
}

pub fn attr_set(attrs: attr_t, color_pair: i16) -> i32 {
    unsafe { sys::attr_set(attrs, color_pair, core::ptr::null_mut()) }
}

pub fn baudrate() -> i32 {
    unsafe { sys::baudrate() }
}

pub fn beep() -> i32 {
    unsafe { sys::beep() }
}

pub fn bkgd(ch: chtype) -> i32 {
    unsafe { sys::bkgd(ch) }
}

pub fn bkgdset(ch: chtype) {
    unsafe { sys::bkgdset(ch) }
}

pub fn border(
    ls: chtype,
    rs: chtype,
    ts: chtype,
    bs: chtype,
    tl: chtype,
    tr: chtype,
    bl: chtype,
    br: chtype,
) -> i32 {
    unsafe { sys::border(ls, rs, ts, bs, tl, tr, bl, br) }
}

pub fn box_(win: WINDOW, verch: chtype, horch: chtype) -> i32 {
    unsafe { sys::box_(win, verch, horch) }
}

pub fn can_change_color() -> bool {
    unsafe { sys::can_change_color() }
}

pub fn cbreak() -> i32 {
    unsafe { sys::cbreak() }
}

pub fn chgat(n: i32, attrs: attr_t, color_pair: i16) -> i32 {
    unsafe { sys::chgat(n, attrs, color_pair, core::ptr::null()) }
}

pub fn clearok(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::clearok(win, flag) }
}

pub fn clear() -> i32 {
    unsafe { sys::clear() }
}

pub fn clrtobot() -> i32 {
    unsafe { sys::clrtobot() }
}

pub fn clrtoeol() -> i32 {
    unsafe { sys::clrtoeol() }
}

pub fn color_content(color: i16, red: &mut i16, green: &mut i16, blue: &mut i16) -> i32 {
    unsafe { sys::color_content(color, red, green, blue) }
}

pub fn color_set(color_pair: i16) -> i32 {
    unsafe { sys::color_set(color_pair, core::ptr::null_mut()) }
}

pub fn COLOR_PAIR(n: i16) -> attr_t {
    unsafe { sys::COLOR_PAIR(n) }
}

pub fn copywin(
    src: WINDOW,
    dst: WINDOW,
    src_tr: i32,
    src_tc: i32,
    dst_tr: i32,
    dst_tc: i32,
    dst_br: i32,
    dst_bc: i32,
    _overlay: i32,
) -> i32 {
    unsafe { sys::copywin(src, dst, src_tr, src_tc, dst_tr, dst_tc, dst_br, dst_bc, _overlay) }
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum CURSOR_VISIBILITY {
    CURSOR_INVISIBLE = 0,
    CURSOR_VISIBLE = 1,
    CURSOR_VERY_VISIBLE = 2,
}

pub fn curs_set(visibility: CURSOR_VISIBILITY) -> i32 {
    unsafe { sys::curs_set(visibility as i32) }
}

pub fn def_prog_mode() -> i32 {
    unsafe { sys::def_prog_mode() }
}

pub fn def_shell_mode() -> i32 {
    unsafe { sys::def_shell_mode() }
}

pub fn delay_output(ms: i32) -> i32 {
    unsafe { sys::delay_output(ms) }
}

pub fn delch() -> i32 {
    unsafe { sys::delch() }
}

pub fn deleteln() -> i32 {
    unsafe { sys::deleteln() }
}

pub fn delscreen(scr: SCREEN) {
    unsafe { sys::delscreen(scr) }
}

pub fn delwin(win: WINDOW) -> i32 {
    unsafe { sys::delwin(win) }
}

pub fn derwin(win: WINDOW, nlines: i32, ncols: i32, begy: i32, begx: i32) -> WINDOW {
    unsafe { sys::derwin(win, nlines, ncols, begy, begx) }
}

pub fn doupdate() -> i32 {
    unsafe { sys::doupdate() }
}

pub fn dupwin(win: WINDOW) -> WINDOW {
    unsafe { sys::dupwin(win) }
}

pub fn echochar(ch: chtype) -> i32 {
    unsafe { sys::echochar(ch) }
}

pub fn echo() -> i32 {
    unsafe { sys::echo() }
}

pub fn endwin() -> i32 {
    unsafe { sys::endwin() }
}

pub fn erasechar() -> u8 {
    unsafe { sys::erasechar() as u8 }
}

pub fn erase() -> i32 {
    unsafe { sys::erase() }
}

pub fn filter() {
    unsafe { sys::filter() }
}

pub fn flash() -> i32 {
    unsafe { sys::flash() }
}

pub fn flushinp() -> i32 {
    unsafe { sys::flushinp() }
}

pub fn getbkgd(win: WINDOW) -> chtype {
    unsafe { sys::getbkgd(win) }
}

pub fn getch() -> i32 {
    unsafe { sys::getch() }
}

// TODO
// pub fn getnstr(str: *mut c_char, n: i32) -> i32 {
//     unsafe { sys::getnstr() }
// }
// pub fn getstr(str: *mut c_char) -> i32 {
//     unsafe { sys::getstr() }
// }

pub fn getwin(filep: *mut FILE) -> WINDOW {
    unsafe { sys::getwin(filep) }
}

pub fn halfdelay(tenths: i32) -> i32 {
    unsafe { sys::halfdelay(tenths) }
}

pub fn has_colors() -> bool {
    unsafe { sys::has_colors() }
}

pub fn has_ic() -> bool {
    unsafe { sys::has_ic() }
}

pub fn has_il() -> bool {
    unsafe { sys::has_il() }
}

pub fn hline(ch: chtype, n: i32) -> i32 {
    unsafe { sys::hline(ch, n) }
}

pub fn idcok(win: WINDOW, flag: bool) {
    unsafe { sys::idcok(win, flag) }
}

pub fn idlok(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::idlok(win, flag) }
}

pub fn immedok(win: WINDOW, flag: bool) {
    unsafe { sys::immedok(win, flag) }
}

// TODO
// pub fn inchnstr(ch: *mut chtype, n: i32) -> i32;
// pub fn inchstr(ch: *mut chtype) -> i32;

pub fn inch() -> chtype {
    unsafe { sys::inch() }
}

pub fn init_color(color: i16, red: i16, green: i16, blue: i16) -> i32 {
    unsafe { sys::init_color(color, red, green, blue) }
}

pub fn init_pair(color_pair: i16, fg: i16, bg: i16) -> i32 {
    unsafe { sys::init_pair(color_pair, fg, bg) }
}

pub fn initscr() -> WINDOW {
    unsafe { sys::initscr() }
}

// TODO
// pub fn innstr(str: *mut c_char, n: i32) -> i32;

pub fn insch(ch: chtype) -> i32 {
    unsafe { sys::insch(ch) }
}

pub fn insdelln(n: i32) -> i32 {
    unsafe { sys::insdelln(n) }
}

pub fn insertln() -> i32 {
    unsafe { sys::insertln() }
}

// pub fn insnstr(s: &str, n: usize) -> i32 {
//     insstr(&s[..n])
// }

pub fn insstr(s: &str) -> i32 {
    unsafe { sys::insstr(cstr(s).as_ptr()) }
}

// TODO
// pub fn instr(str: *mut c_char) -> i32;

pub fn intrflush(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::intrflush(win, flag) }
}

pub fn isendwin() -> bool {
    unsafe { sys::isendwin() }
}

pub fn is_linetouched(win: WINDOW, line: i32) -> bool {
    unsafe { sys::is_linetouched(win, line) }
}

pub fn is_wintouched(win: WINDOW) -> bool {
    unsafe { sys::is_wintouched(win) }
}

fn keyname_(c: i32) -> String {
    // NOTE: In PDCurses 3.9 keyname() never returns NULL.
    let s = unsafe { std::ffi::CStr::from_ptr(sys::keyname(c)) };
    s.to_str().unwrap().into()
}

#[cfg(feature = "ncurses_compat")]
#[inline(always)]
pub fn keyname(c: i32) -> Option<String> {
    Some(keyname_(c))
}

#[cfg(not(feature = "ncurses_compat"))]
#[inline(always)]
pub fn keyname(c: i32) -> String {
    keyname_(c)
}

pub fn keypad(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::keypad(win, flag) }
}

pub fn killchar() -> u8 {
    unsafe { sys::killchar() as u8 }
}

pub fn leaveok(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::leaveok(win, flag) }
}

pub fn longname() -> Option<&'static str> {
    match unsafe { sys::longname() } {
        s if s.is_null() => { None }
        s => {
            let s = unsafe { std::ffi::CStr::from_ptr(s) };
            Some(s.to_str().unwrap())
        }
    }
}

pub fn meta(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::meta(win, flag) }
}

pub fn move_(y: i32, x: i32) -> i32 {
    unsafe { sys::move_(y, x) }
}

/// An alias for [`move_`].
pub fn mv(y: i32, x: i32) -> i32 {
    unsafe { sys::move_(y, x) }
}

pub fn mvaddch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvaddch(y, x, ch) }
}

// pub fn mvaddchnstr(y: i32, x: i32, ch: &[chtype], n: usize) -> i32 {
//     mvaddchstr(y, x, &ch[..n])
// }

pub fn mvaddchstr(y: i32, x: i32, ch: &[chtype]) -> i32 {
    unsafe { sys::mvaddchstr(y, x, chstr(ch).as_ptr()) }
}

// pub fn mvaddnstr(y: i32, x: i32, s: &str, n: usize) -> i32 {
//     mvaddstr(y, x, &s[..n])
// }

pub fn mvaddstr(y: i32, x: i32, s: &str) -> i32 {
    unsafe { sys::mvaddstr(y, x, cstr(s).as_ptr()) }
}

pub fn mvchgat(y: i32, x: i32, n: i32, attrs: attr_t, color_pair: i16) -> i32 {
    unsafe { sys::mvchgat(y, x, n, attrs, color_pair, core::ptr::null()) }
}

pub fn mvcur(oldrow: i32, oldcol: i32, newrow: i32, newcol: i32) -> i32 {
    unsafe { sys::mvcur(oldrow, oldcol, newrow, newcol) }
}

pub fn mvdelch(y: i32, x: i32) -> i32 {
    unsafe { sys::mvdelch(y, x) }
}

pub fn mvderwin(win: WINDOW, pary: i32, parx: i32) -> i32 {
    unsafe { sys::mvderwin(win, pary, parx) }
}

pub fn mvgetch(y: i32, x: i32) -> i32 {
    unsafe { sys::mvgetch(y, x) }
}

// TODO
// pub fn mvgetnstr(y: i32, x: i32, str: *mut c_char, n: i32) -> i32 {
//     unsafe { sys::mvgetnstr() }
// }
// pub fn mvgetstr(y: i32, x: i32, str: *mut c_char) -> i32 {
//     unsafe { sys::mvgetstr() }
// }

pub fn mvhline(y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    unsafe { sys::mvhline(y, x, ch, n) }
}

pub fn mvinch(y: i32, x: i32) -> chtype {
    unsafe { sys::mvinch(y, x) }
}

// TODO
// pub fn mvinchnstr(y: i32, x: i32, ch: *mut chtype, n: i32) -> i32 {
//     unsafe { sys::mvinchnstr() }
// }
// pub fn mvinchstr(y: i32, x: i32, ch: *mut chtype) -> i32 {
//     unsafe { sys::mvinchstr() }
// }
// pub fn mvinnstr(y: i32, x: i32, str: *mut c_char, n: i32) -> i32 {
//     unsafe { sys::mvinnstr() }
// }

pub fn mvinsch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvinsch(y, x, ch) }
}

// pub fn mvinsnstr(y: i32, x: i32, s: &str, n: usize) -> i32 {
//     mvinsstr(y, x, &s[..n])
// }

pub fn mvinsstr(y: i32, x: i32, s: &str) -> i32 {
    unsafe { sys::mvinsstr(y, x, cstr(s).as_ptr()) }
}

// TODO
// pub fn mvinstr(y: i32, x: i32, str: *mut c_char) -> i32 {
//     unsafe { sys::mvinstr() }
// }

// TODO: C variadic
// pub fn mvprintw(y: i32, x: i32, fmt: *const c_char, ...) -> i32 {
//     unsafe { sys::mvprintw() }
// }
// pub fn mvscanw(y: i32, x: i32, fmt: *const c_char, ...) -> i32 {
//     unsafe { sys::mvscanw() }
// }

pub fn mvvline(y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    unsafe { sys::mvvline(y, x, ch, n) }
}

// pub fn mvwaddchnstr(win: WINDOW, y: i32, x: i32, ch: &[chtype], n: usize) -> i32 {
//     mvwaddchstr(win, y, x, &ch[..n])
// }

pub fn mvwaddchstr(win: WINDOW, y: i32, x: i32, ch: &[chtype]) -> i32 {
    unsafe { sys::mvwaddchstr(win, y, x, chstr(ch).as_ptr()) }
}

pub fn mvwaddch(win: WINDOW, y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvwaddch(win, y, x, ch) }
}

// pub fn mvwaddnstr(win: WINDOW, y: i32, x: i32, s: &str, n: usize) -> i32 {
//     mvwaddstr(win, y, x, &s[..n])
// }

pub fn mvwaddstr(win: WINDOW, y: i32, x: i32, s: &str) -> i32 {
    unsafe { sys::mvwaddstr(win, y, x, cstr(s).as_ptr()) }
}

pub fn mvwchgat(win: WINDOW, y: i32, x: i32, n: i32, attrs: attr_t, color_pair: i16) -> i32 {
    unsafe { sys::mvwchgat(win, y, x, n, attrs, color_pair, core::ptr::null()) }
}

pub fn mvwdelch(win: WINDOW, y: i32, x: i32) -> i32 {
    unsafe { sys::mvwdelch(win, y, x) }
}

pub fn mvwgetch(win: WINDOW, y: i32, x: i32) -> i32 {
    unsafe { sys::mvwgetch(win, y, x) }
}

// TODO
// pub fn mvwgetnstr      (win: WINDOW, y: i32, x: i32, str: *mut c_char, n: i32) -> i32;
// pub fn mvwgetstr       (win: WINDOW, y: i32, x: i32, str: *mut c_char) -> i32;

pub fn mvwhline(win: WINDOW, y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    unsafe { sys::mvwhline(win, y, x, ch, n) }
}

// TODO
// pub fn mvwinchnstr     (win: WINDOW, y: i32, x: i32, ch: *mut chtype, n: i32) -> i32;
// pub fn mvwinchstr      (win: WINDOW, y: i32, x: i32, ch: *mut chtype) -> i32;

pub fn mvwinch(win: WINDOW, y: i32, x: i32) -> chtype {
    unsafe { sys::mvwinch(win, y, x) }
}

// TODO
// pub fn mvwinnstr(win: WINDOW, y: i32, x: i32, str: *mut c_char, n: i32) -> i32;

pub fn mvwinsch(win: WINDOW, y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvwinsch(win, y, x, ch) }
}

// pub fn mvwinsnstr(win: WINDOW, y: i32, x: i32, s: &str, n: usize) -> i32 {
//     mvwinsstr(win, y, x, &s[..n])
// }

pub fn mvwinsstr(win: WINDOW, y: i32, x: i32, s: &str) -> i32 {
    unsafe { sys::mvwinsstr(win, y, x, cstr(s).as_ptr()) }
}

// TODO
// pub fn mvwinstr(win: WINDOW, y: i32, x: i32, str: *mut c_char) -> i32;

pub fn mvwin(win: WINDOW, y: i32, x: i32) -> i32 {
    unsafe { sys::mvwin(win, y, x) }
}

// TODO: C variadic
// pub fn mvwprintw(win: WINDOW, y: i32, x: i32, fmt: *const c_char, ...) -> i32;
// pub fn mvwscanw (win: WINDOW, y: i32, x: i32, fmt: *const c_char, ...) -> i32;

pub fn mvwvline(win: WINDOW, y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    unsafe { sys::mvwvline(win, y, x, ch, n) }
}

pub fn napms(ms: i32) -> i32 {
    unsafe { sys::napms(ms) }
}

pub fn newpad(nlines: i32, ncols: i32) -> WINDOW {
    unsafe { sys::newpad(nlines, ncols) }
}

pub fn newterm(termtype: Option<&str>, outfd: *mut FILE, infd: *mut FILE) -> SCREEN {
    let ty = termtype.map(cstr);
    let ty_ptr = ty.as_ref().map(|s| s.as_ptr()).unwrap_or(core::ptr::null());
    unsafe { sys::newterm(ty_ptr, outfd, infd) }
}

pub fn newwin(nlines: i32, ncols: i32, begy: i32, begx: i32) -> WINDOW {
    unsafe { sys::newwin(nlines, ncols, begy, begx) }
}

pub fn nl() -> i32 {
    unsafe { sys::nl() }
}

pub fn nocbreak() -> i32 {
    unsafe { sys::nocbreak() }
}

pub fn nodelay(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::nodelay(win, flag) }
}

pub fn noecho() -> i32 {
    unsafe { sys::noecho() }
}

pub fn nonl() -> i32 {
    unsafe { sys::nonl() }
}

pub fn noqiflush() {
    unsafe { sys::noqiflush() }
}

pub fn noraw() -> i32 {
    unsafe { sys::noraw() }
}

pub fn notimeout(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::notimeout(win, flag) }
}

pub fn overlay(src: WINDOW, dst: WINDOW) -> i32 {
    unsafe { sys::overlay(src, dst) }
}

pub fn overwrite(src: WINDOW, dst: WINDOW) -> i32 {
    unsafe { sys::overwrite(src, dst) }
}

pub fn pair_content(color_pair: i16, fg: &mut i16, bg: &mut i16) -> i32 {
    unsafe { sys::pair_content(color_pair, fg, bg) }
}

pub fn PAIR_NUMBER(n: attr_t) -> i16 {
    unsafe { sys::PAIR_NUMBER(n) }
}

pub fn pechochar(win: WINDOW, ch: chtype) -> i32 {
    unsafe { sys::pechochar(win, ch) }
}

pub fn pnoutrefresh(win: WINDOW, py: i32, px: i32, sy1: i32, sx1: i32, sy2: i32, sx2: i32) -> i32 {
    unsafe { sys::pnoutrefresh(win, py, px, sy1, sx1, sy2, sx2) }
}

pub fn prefresh(win: WINDOW, py: i32, px: i32, sy1: i32, sx1: i32, sy2: i32, sx2: i32) -> i32 {
    unsafe { sys::prefresh(win, py, px, sy1, sx1, sy2, sx2) }
}

// TODO: C variadic
// pub fn printw(fmt: *const c_char, ...) -> i32;

pub fn putwin(win: WINDOW, filep: *mut FILE) -> i32 {
    unsafe { sys::putwin(win, filep) }
}

pub fn qiflush() {
    unsafe { sys::qiflush() }
}

pub fn raw() -> i32 {
    unsafe { sys::raw() }
}

pub fn redrawwin(win: WINDOW) -> i32 {
    unsafe { sys::redrawwin(win) }
}

pub fn refresh() -> i32 {
    unsafe { sys::refresh() }
}

pub fn reset_prog_mode() -> i32 {
    unsafe { sys::reset_prog_mode() }
}

pub fn reset_shell_mode() -> i32 {
    unsafe { sys::reset_shell_mode() }
}

pub fn resetty() -> i32 {
    unsafe { sys::resetty() }
}

pub fn ripoffline(line: i32, init: Option<unsafe extern "C" fn(win: WINDOW, ncols: i32) -> i32>) -> i32 {
    unsafe { sys::ripoffline(line, init) }
}

pub fn savetty() -> i32 {
    unsafe { sys::savetty() }
}

// TODO: C variadic
// pub fn scanw(fmt: *const c_char, ...) -> i32;

pub fn scr_dump(filename: &str) -> i32 {
    unsafe { sys::scr_dump(cstr(filename).as_ptr()) }
}

pub fn scr_init(filename: &str) -> i32 {
    unsafe { sys::scr_init(cstr(filename).as_ptr()) }
}

pub fn scr_restore(filename: &str) -> i32 {
    unsafe { sys::scr_restore(cstr(filename).as_ptr()) }
}

pub fn scr_set(filename: &str) -> i32 {
    unsafe { sys::scr_set(cstr(filename).as_ptr()) }
}

pub fn scrl(n: i32) -> i32 {
    unsafe { sys::scrl(n) }
}

pub fn scroll(win: WINDOW) -> i32 {
    unsafe { sys::scroll(win) }
}

pub fn scrollok(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::scrollok(win, flag) }
}

pub fn set_term(new: SCREEN) -> SCREEN {
    unsafe { sys::set_term(new) }
}

pub fn setscrreg(top: i32, bot: i32) -> i32 {
    unsafe { sys::setscrreg(top, bot) }
}

pub fn slk_attroff(attrs: chtype) -> i32 {
    unsafe { sys::slk_attroff(attrs) }
}

pub fn slk_attr_off(attrs: attr_t) -> i32 {
    unsafe { sys::slk_attr_off(attrs, core::ptr::null_mut()) }
}

pub fn slk_attron(attrs: chtype) -> i32 {
    unsafe { sys::slk_attron(attrs) }
}

pub fn slk_attr_on(attrs: attr_t) -> i32 {
    unsafe { sys::slk_attr_on(attrs, core::ptr::null_mut()) }
}

pub fn slk_attrset(attrs: chtype) -> i32 {
    unsafe { sys::slk_attrset(attrs) }
}

pub fn slk_attr_set(attrs: attr_t, color_pair: i16) -> i32 {
    unsafe { sys::slk_attr_set(attrs, color_pair, core::ptr::null_mut()) }
}

pub fn slk_clear() -> i32 {
    unsafe { sys::slk_clear() }
}

pub fn slk_color(color_pair: i16) -> i32 {
    unsafe { sys::slk_color(color_pair) }
}

pub fn slk_init(fmt: i32) -> i32 {
    unsafe { sys::slk_init(fmt) }
}

pub fn slk_label(labnum: i32) -> String {
    let s = unsafe { std::ffi::CStr::from_ptr(sys::slk_label(labnum)) };
    s.to_str().unwrap().to_owned()
}

pub fn slk_noutrefresh() -> i32 {
    unsafe { sys::slk_noutrefresh() }
}

pub fn slk_refresh() -> i32 {
    unsafe { sys::slk_refresh() }
}

pub fn slk_restore() -> i32 {
    unsafe { sys::slk_restore() }
}

pub fn slk_set(labnum: i32, label: &str, justify: i32) -> i32 {
    unsafe { sys::slk_set(labnum, cstr(label).as_ptr(), justify) }
}

pub fn slk_touch() -> i32 {
    unsafe { sys::slk_touch() }
}

pub fn standend() -> i32 {
    unsafe { sys::standend() }
}

pub fn standout() -> i32 {
    unsafe { sys::standout() }
}

pub fn start_color() -> i32 {
    unsafe { sys::start_color() }
}

pub fn subpad(orig: WINDOW, nlines: i32, ncols: i32, begy: i32, begx: i32) -> WINDOW {
    unsafe { sys::subpad(orig, nlines, ncols, begy, begx) }
}

pub fn subwin(orig: WINDOW, nlines: i32, ncols: i32, begy: i32, begx: i32) -> WINDOW {
    unsafe { sys::subwin(orig, nlines, ncols, begy, begx) }
}

pub fn syncok(win: WINDOW, flag: bool) -> i32 {
    unsafe { sys::syncok(win, flag) }
}

pub fn termattrs() -> chtype {
    unsafe { sys::termattrs() }
}

pub fn term_attrs() -> attr_t {
    unsafe { sys::term_attrs() }
}

pub fn termname() -> &'static str {
    let s = unsafe { std::ffi::CStr::from_ptr(sys::termname()) };
    s.to_str().unwrap()
}

pub fn timeout(delay: i32) {
    unsafe { sys::timeout(delay) }
}

pub fn touchline(win: WINDOW, start: i32, count: i32) -> i32 {
    unsafe { sys::touchline(win, start, count) }
}

pub fn touchwin(win: WINDOW) -> i32 {
    unsafe { sys::touchwin(win) }
}

pub fn typeahead(fd: i32) -> i32 {
    unsafe { sys::typeahead(fd) }
}

pub fn ungetch(ch: i32) -> i32 {
    unsafe { sys::ungetch(ch) }
}

pub fn untouchwin(win: WINDOW) -> i32 {
    unsafe { sys::untouchwin(win) }
}

pub fn use_env(flag: bool) {
    unsafe { sys::use_env(flag) }
}

// NOTE: declared but never defined stubs.
//
// pub fn vidattr  (attrs: chtype) -> i32;
// pub fn vid_attr (attrs: attr_t, color_pair: i16) -> i32;
// pub fn vidputs  (attrs: chtype, putc: Option<unsafe extern "C" fn(ch: i32) -> i32>) -> i32;
// pub fn vid_puts (attrs: attr_t, color_pair: i16, putc: Option<unsafe extern "C" fn(ch: i32) -> i32>) -> i32;

 pub fn vline(ch: chtype, n: i32) -> i32 {
     unsafe { sys::vline(ch, n) }
 }

// TODO: C va_list
// pub fn vw_printw (win: WINDOW, fmt: *const c_char, varglist: va_list) -> i32;
// pub fn vwprintw  (win: WINDOW, fmt: *const c_char, varglist: va_list) -> i32;
// pub fn vw_scanw  (win: WINDOW, fmt: *const c_char, varglist: va_list) -> i32;
// pub fn vwscanw   (win: WINDOW, fmt: *const c_char, varglist: va_list) -> i32;

// pub fn waddchnstr(win: WINDOW, ch: &[chtype], n: usize) -> i32 {
//     waddchstr(win, &ch[..n])
// }

pub fn waddchstr(win: WINDOW, ch: &[chtype]) -> i32 {
    unsafe { sys::waddchstr(win, chstr(ch).as_ptr()) }
}

pub fn waddch(win: WINDOW, ch: chtype) -> i32 {
    unsafe { sys::waddch(win, ch) }
}

// pub fn waddnstr(win: WINDOW, s: &str, n: usize) -> i32 {
//     waddstr(win, &s[..n])
// }

pub fn waddstr(win: WINDOW, s: &str) -> i32 {
    unsafe { sys::waddstr(win, cstr(s).as_ptr()) }
}

pub fn wattroff(win: WINDOW, attrs: chtype) -> i32 {
    unsafe { sys::wattroff(win, attrs) }
}

pub fn wattron(win: WINDOW, attrs: chtype) -> i32 {
    unsafe { sys::wattron(win, attrs) }
}

pub fn wattrset(win: WINDOW, attrs: chtype) -> i32 {
    unsafe { sys::wattrset(win, attrs) }
}

pub fn wattr_get(win: WINDOW, attrs: &mut attr_t, color_pair: &mut i16) -> i32 {
    unsafe { sys::wattr_get(win, attrs, color_pair, core::ptr::null_mut()) }
}

pub fn wattr_off(win: WINDOW, attrs: attr_t) -> i32 {
    unsafe { sys::wattr_off(win, attrs, core::ptr::null_mut()) }
}

pub fn wattr_on(win: WINDOW, attrs: attr_t) -> i32 {
    unsafe { sys::wattr_on(win, attrs, core::ptr::null_mut()) }
}

pub fn wattr_set(win: WINDOW, attrs: attr_t, color_pair: i16) -> i32 {
    unsafe { sys::wattr_set(win, attrs, color_pair, core::ptr::null_mut()) }
}

pub fn wbkgdset(win: WINDOW, ch: chtype) {
    unsafe { sys::wbkgdset(win, ch) }
}

pub fn wbkgd(win: WINDOW, ch: chtype) -> i32 {
    unsafe { sys::wbkgd(win, ch) }
}

pub fn wborder(
    win: WINDOW,
    ls: chtype,
    rs: chtype,
    ts: chtype,
    bs: chtype,
    tl: chtype,
    tr: chtype,
    bl: chtype,
    br: chtype,
) -> i32 {
    unsafe { sys::wborder(win, ls, rs, ts, bs, tl, tr, bl, br) }
}

pub fn wchgat(win: WINDOW, n: i32, attrs: attr_t, color_pair: i16) -> i32 {
    unsafe { sys::wchgat(win, n, attrs, color_pair, core::ptr::null_mut()) }
}

pub fn wclear(win: WINDOW) -> i32 {
    unsafe { sys::wclear(win) }
}

pub fn wclrtobot(win: WINDOW) -> i32 {
    unsafe { sys::wclrtobot(win) }
}

pub fn wclrtoeol(win: WINDOW) -> i32 {
    unsafe { sys::wclrtoeol(win) }
}

pub fn wcolor_set(win: WINDOW, color: i16) -> i32 {
    unsafe { sys::wcolor_set(win, color, core::ptr::null_mut()) }
}

pub fn wcursyncup(win: WINDOW) {
    unsafe { sys::wcursyncup(win) }
}

pub fn wdelch(win: WINDOW) -> i32 {
    unsafe { sys::wdelch(win) }
}

pub fn wdeleteln(win: WINDOW) -> i32 {
    unsafe { sys::wdeleteln(win) }
}

pub fn wechochar(win: WINDOW, ch: chtype) -> i32 {
    unsafe { sys::wechochar(win, ch) }
}

pub fn werase(win: WINDOW) -> i32 {
    unsafe { sys::werase(win) }
}

pub fn wgetch(win: WINDOW) -> i32 {
    unsafe { sys::wgetch(win) }
}

// TODO
// pub fn wgetnstr(win: WINDOW, s: *mut c_char, n: i32) -> i32 {
//     unsafe { sys::wgetnstr(win, s, n) }
// }
// pub fn wgetstr(win: WINDOW, s: *mut c_char) -> i32 {
//     unsafe { sys::wgetstr(win, s) }
// }

pub fn whline(win: WINDOW, ch: chtype, n: i32) -> i32 {
    unsafe { sys::whline(win, ch, n) }
}

// TODO
// pub fn winchnstr       (win: WINDOW, ch: *mut chtype, n: i32) -> i32;
// pub fn winchstr        (win: WINDOW, ch: *mut chtype) -> i32;

pub fn winch(win: WINDOW) -> chtype {
    unsafe { sys::winch(win) }
}

// TODO
// pub fn winnstr         (win: WINDOW, str: *mut c_char, n: i32) -> i32;

pub fn winsch(win: WINDOW, ch: chtype) -> i32 {
    unsafe { sys::winsch(win, ch) }
}

pub fn winsdelln(win: WINDOW, n: i32) -> i32 {
    unsafe { sys::winsdelln(win, n) }
}

pub fn winsertln(win: WINDOW) -> i32 {
    unsafe { sys::winsertln(win) }
}

// pub fn winsnstr(win: WINDOW, s: &str, n: usize) -> i32 {
//     winsstr(win, &s[..n])
// }

pub fn winsstr(win: WINDOW, s: &str) -> i32 {
    unsafe { sys::winsstr(win, cstr(s).as_ptr()) }
}

// TODO
// pub fn winstr(win: WINDOW, str: *mut c_char) -> i32;

pub fn wmove(win: WINDOW, y: i32, x: i32) -> i32 {
    unsafe { sys::wmove(win, y, x) }
}

pub fn wnoutrefresh(win: WINDOW) -> i32 {
    unsafe { sys::wnoutrefresh(win) }
}

// TODO: C variadic
// pub fn wprintw(win: WINDOW, fmt: *const c_char, ...) -> i32;

pub fn wredrawln(win: WINDOW, beg_line: i32, num_lines: i32) -> i32 {
    unsafe { sys::wredrawln(win, beg_line, num_lines) }
}

pub fn wrefresh(win: WINDOW) -> i32 {
    unsafe { sys::wrefresh(win) }
}

// TODO: C variadic
// pub fn wscanw(win: WINDOW, fmt: *const c_char, ...) -> i32;

pub fn wscrl(win: WINDOW, n: i32) -> i32 {
    unsafe { sys::wscrl(win, n) }
}

pub fn wsetscrreg(win: WINDOW, top: i32, bot: i32) -> i32 {
    unsafe { sys::wsetscrreg(win, top, bot) }
}

pub fn wstandend(win: WINDOW) -> i32 {
    unsafe { sys::wstandend(win) }
}

pub fn wstandout(win: WINDOW) -> i32 {
    unsafe { sys::wstandout(win) }
}

pub fn wsyncdown(win: WINDOW) {
    unsafe { sys::wsyncdown(win) }
}

pub fn wsyncup(win: WINDOW) {
    unsafe { sys::wsyncup(win) }
}

pub fn wtimeout(win: WINDOW, delay: i32) {
    unsafe { sys::wtimeout(win, delay) }
}

pub fn wtouchln(win: WINDOW, y: i32, n: i32, changed: i32) -> i32 {
    unsafe { sys::wtouchln(win, y, n, changed) }
}

pub fn wvline(win: WINDOW, ch: chtype, n: i32) -> i32 {
    unsafe { sys::wvline(win, ch, n) }
}


// pub fn addnwstr(w: &[u16], n: usize) -> i32 {
//     addwstr(&w[..n])
// }

pub fn addwstr(w: &[u16]) -> i32 {
    unsafe { sys::addwstr(wstr(w).as_ptr()) }
}

// TODO
// pub fn add_wch       (wch: *const cchar_t) -> i32;
// pub fn add_wchnstr(wch: &[cchar_t], n: usize) -> i32 {
//     add_wchstr(&wch[..n])
// }
// pub fn add_wchstr(wch: &[cchar_t]) -> i32 {
//     unsafe { sys::add_wchstr(ccstr(wch).as_ptr()) }
// }
// pub fn bkgrnd(wch: *const cchar_t) -> i32 {
//     unsafe { sys::bkgrnd(wch) }
// }
// pub fn bkgrndset(wch: *const cchar_t) {
//     unsafe { sys::bkgrndset(wch) }
// }

pub fn border_set(
    ls: *const cchar_t,
    rs: *const cchar_t,
    ts: *const cchar_t,
    bs: *const cchar_t,
    tl: *const cchar_t,
    tr: *const cchar_t,
    bl: *const cchar_t,
    br: *const cchar_t,
) -> i32 {
    unsafe { sys::border_set(ls, rs, ts, bs, tl, tr, bl, br) }
}

pub fn box_set(win: WINDOW, verch: *const cchar_t, horch: *const cchar_t) -> i32 {
    unsafe { sys::box_set(win, verch, horch) }
}

// TODO
// pub fn echo_wchar    (wch: *const cchar_t) -> i32;

pub fn erasewchar(wch: &mut u16) -> i32 {
    unsafe { sys::erasewchar(wch) }
}

pub fn getbkgrnd(wch: &mut cchar_t) -> i32 {
    unsafe { sys::getbkgrnd(wch) }
}

// TODO
// pub fn getcchar(
//     wcval: &cchar_t,
//     wch: &mut u16,
//     attrs: &mut attr_t,
//     color_pair: &mut i16,
// ) -> i32 {
//     unsafe { sys::getcchar(wcval, wch, attrs, color_pair, core::ptr::null_mut()) }
// }

pub fn get_wch(wch: &mut u16) -> i32 {
    unsafe { sys::get_wch(wch) }
}

#[deprecated = "use `get_wstr`"]
pub fn getn_wstr(_: &mut [u16], _: i32) -> i32 {
    unimplemented!("use `get_wstr`")
}

pub fn get_wstr(buf: &mut [u16]) -> i32 {
    assert!(buf.len() <= i32::MAX as usize);
    unsafe { sys::getn_wstr(buf.as_mut_ptr(), buf.len() as i32) }
}

pub fn hline_set(wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::hline_set(wch, n) }
}

// TODO
// pub fn innwstr(wstr: *mut wchar_t, n: i32) -> i32;

// pub fn ins_nwstr(w: &[u16], n: usize) -> i32 {
//     ins_wstr(&w[..n])
// }

// TODO
// pub fn ins_wch(wch: *const cchar_t) -> i32 {
//     unsafe { sys::ins_wch(wch) }
// }

pub fn ins_wstr(w: &[u16]) -> i32 {
    unsafe { sys::ins_wstr(wstr(w).as_ptr()) }
}

// pub fn inwstr        (wstr: *mut wchar_t) -> i32;
// pub fn in_wch        (wch: *mut cchar_t) -> i32;
// pub fn in_wchnstr    (wch: *mut cchar_t, n: i32) -> i32;
// pub fn in_wchstr     (wch: *mut cchar_t) -> i32;

pub fn key_name(w: u16) -> &'static str {
    let s = unsafe { std::ffi::CStr::from_ptr(sys::key_name(w)) };
    s.to_str().unwrap()
}

pub fn killwchar(wch: &mut u16) -> i32 {
    unsafe { sys::killwchar(wch) }
}

// pub fn mvaddnwstr(y: i32, x: i32, w: &[u16], n: usize) -> i32 {
//     mvaddwstr(y, x, &w[..n])
// }

pub fn mvaddwstr(y: i32, x: i32, w: &[u16]) -> i32 {
    unsafe { sys::mvaddwstr(y, x, wstr(w).as_ptr()) }
}

// TODO
// pub fn mvadd_wch     (y: i32, x: i32, wch: *const cchar_t) -> i32;
// pub fn mvadd_wchnstr (y: i32, x: i32, wch: *const cchar_t, n: i32) -> i32;
// pub fn mvadd_wchstr  (y: i32, x: i32, wch: *const cchar_t) -> i32;
// pub fn mvgetn_wstr   (y: i32, x: i32, wstr: *mut wint_t, n: i32) -> i32;
// pub fn mvget_wch     (y: i32, x: i32, wch: *mut wint_t) -> i32;
// pub fn mvget_wstr    (y: i32, x: i32, wstr: *mut wint_t) -> i32;

pub fn mvhline_set(y: i32, x: i32, wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::mvhline_set(y, x, wch, n) }
}

// TODO
// pub fn mvinnwstr(y: i32, x: i32, wstr: *mut wchar_t, n: i32) -> i32;

// pub fn mvins_nwstr(y: i32, x: i32, w: &[u16], n: usize) -> i32 {
//     mvins_wstr(y, x, &w[..n])
// }

// TODO
// pub fn mvins_wch(y: i32, x: i32, wch: *const cchar_t) -> i32;

pub fn mvins_wstr(y: i32, x: i32, w: &[u16]) -> i32 {
    unsafe { sys::mvins_wstr(y, x, wstr(w).as_ptr()) }
}

// TODO
// pub fn mvinwstr      (y: i32, x: i32, wstr: *mut wchar_t) -> i32;
// pub fn mvin_wch      (y: i32, x: i32, wch: *mut cchar_t) -> i32;
// pub fn mvin_wchnstr  (y: i32, x: i32, wch: *mut cchar_t, n: i32) -> i32;
// pub fn mvin_wchstr   (y: i32, x: i32, wch: *mut cchar_t) -> i32;

pub fn mvvline_set(y: i32, x: i32, wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::mvvline_set(y, x, wch, n) }
}

// pub fn mvwaddnwstr(win: WINDOW, y: i32, x: i32, w: &[u16], n: usize) -> i32 {
//     mvwaddwstr(win, y, x, &w[..n])
// }

pub fn mvwaddwstr(win: WINDOW, y: i32, x: i32, w: &[u16]) -> i32 {
    unsafe { sys::mvwaddwstr(win, y, x, wstr(w).as_ptr()) }
}

// TODO
// pub fn mvwadd_wch    (win: WINDOW, y: i32, x: i32, wch: *const cchar_t) -> i32;
// pub fn mvwadd_wchnstr(win: WINDOW, y: i32, x: i32, wch: *const cchar_t, n: i32) -> i32;
// pub fn mvwadd_wchstr (win: WINDOW, y: i32, x: i32, wch: *const cchar_t) -> i32;
// pub fn mvwgetn_wstr  (win: WINDOW, y: i32, x: i32, wstr: *mut wint_t, n: i32) -> i32;
// pub fn mvwget_wch    (win: WINDOW, y: i32, x: i32, wch: *mut wint_t) -> i32;
// pub fn mvwget_wstr   (win: WINDOW, y: i32, x: i32, wstr: *mut wint_t) -> i32;

pub fn mvwhline_set(win: WINDOW, y: i32, x: i32, wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::mvwhline_set(win, y, x, wch, n) }
}

// TODO
// pub fn mvwinnwstr(win: WINDOW, y: i32, x: i32, w: *mut wchar_t, n: i32) -> i32;

// pub fn mvwins_nwstr(win: WINDOW, y: i32, x: i32, w: &[u16], n: usize) -> i32 {
//     mvwins_wstr(win, y, x, &w[..n])
// }

// TODO
// pub fn mvwins_wch(win: WINDOW, y: i32, x: i32, wch: *const cchar_t) -> i32;

pub fn mvwins_wstr(win: WINDOW, y: i32, x: i32, w: &[u16]) -> i32 {
    unsafe { sys::mvwins_wstr(win, y, x, wstr(w).as_ptr()) }
}

// TODO
// pub fn mvwin_wch     (win: WINDOW, y: i32, x: i32, wch: *mut cchar_t) -> i32;
// pub fn mvwin_wchnstr (win: WINDOW, y: i32, x: i32, wch: *mut cchar_t, n: i32) -> i32;
// pub fn mvwin_wchstr  (win: WINDOW, y: i32, x: i32, wch: *mut cchar_t) -> i32;
// pub fn mvwinwstr     (win: WINDOW, y: i32, x: i32, wstr: *mut wchar_t) -> i32;

pub fn mvwvline_set(win: WINDOW, y: i32, x: i32, wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::mvwvline_set(win, y, x, wch, n) }
}

// TODO
// pub fn pecho_wchar(win: WINDOW, wch: *const cchar_t) -> i32;
// pub fn setcchar(
//     wch: &mut cchar_t,
//     wch: &u16,
//     attrs: attr_t,
//     color_pair: i16,
// ) -> i32 {
//     unsafe { sys::setcchar(wch, wch, attrs, color_pair, core::ptr::null_mut()) }
// }

pub fn slk_wset(labnum: i32, label: &[u16], justify: i32) -> i32 {
    unsafe { sys::slk_wset(labnum, wstr(label).as_ptr(), justify) }
}

pub fn unget_wch(wch: u16) -> i32 {
    unsafe { sys::unget_wch(wch) }
}

pub fn vline_set(wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::vline_set(wch, n) }
}

// pub fn waddnwstr(win: WINDOW, w: &[u16], n: usize) -> i32 {
//     waddwstr(win, &w[..n])
// }

pub fn waddwstr(win: WINDOW, w: &[u16]) -> i32 {
    unsafe { sys::waddwstr(win, wstr(w).as_ptr()) }
}

// TODO
// pub fn wadd_wch      (win: WINDOW, wch: *const cchar_t) -> i32;
// pub fn wadd_wchnstr  (win: WINDOW, wch: *const cchar_t, n: i32) -> i32;
// pub fn wadd_wchstr   (win: WINDOW, wch: *const cchar_t) -> i32;

pub fn wbkgrnd(win: WINDOW, wch: *const cchar_t) -> i32 {
    unsafe { sys::wbkgrnd(win, wch) }
}

pub fn wbkgrndset(win: WINDOW, wch: *const cchar_t) {
    unsafe { sys::wbkgrndset(win, wch) }
}

pub fn wborder_set(
    win: WINDOW,
    ls: *const cchar_t,
    rs: *const cchar_t,
    ts: *const cchar_t,
    bs: *const cchar_t,
    tl: *const cchar_t,
    tr: *const cchar_t,
    bl: *const cchar_t,
    br: *const cchar_t,
) -> i32 {
    unsafe { sys::wborder_set(win, ls, rs, ts, bs, tl, tr, bl, br) }
}

// TODO
// pub fn wecho_wchar   (win: WINDOW, wch: *const cchar_t) -> i32;
// pub fn wgetbkgrnd    (win: WINDOW, wch: *mut cchar_t) -> i32;

pub fn wget_wch(win: WINDOW, wch: &mut u16) -> i32 {
    unsafe { sys::wget_wch(win, wch) }
}

pub fn wget_wstr(win: WINDOW, buf: &mut [u16]) -> i32 {
    assert!(buf.len() <= i32::MAX as usize);
    unsafe { sys::wgetn_wstr(win, buf.as_mut_ptr(), buf.len() as i32) }
}

#[deprecated = "use `wget_wstr`"]
pub fn wgetn_wstr(_: WINDOW, _: &mut [u16], _: i32) -> i32 {
    unimplemented!("use `wget_wstr`")
}

pub fn whline_set(win: WINDOW, wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::whline_set(win, wch, n) }
}

// TODO
// pub fn winnwstr(win: WINDOW, wstr: *mut wchar_t, n: i32) -> i32;

// pub fn wins_nwstr(win: WINDOW, w: &[u16], n: usize) -> i32 {
//     wins_wstr(win, &w[..n])
// }

// TODO
// pub fn wins_wch(win: WINDOW, wch: *const cchar_t) -> i32;

pub fn wins_wstr(win: WINDOW, w: &[u16]) -> i32 {
    unsafe { sys::wins_wstr(win, wstr(w).as_ptr()) }
}

// TODO
// pub fn winwstr       (win: WINDOW, wstr: *mut wchar_t) -> i32;
// pub fn win_wch       (win: WINDOW, wch: *mut cchar_t) -> i32;
// pub fn win_wchnstr   (win: WINDOW, wch: *mut cchar_t, n: i32) -> i32;
// pub fn win_wchstr    (win: WINDOW, wch: *mut cchar_t) -> i32;
// pub fn wunctrl       (wc: *mut cchar_t) -> *mut wchar_t;

pub fn wvline_set(win: WINDOW, wch: *const cchar_t, n: i32) -> i32 {
    unsafe { sys::wvline_set(win, wch, n) }
}


pub fn getattrs(win: WINDOW) -> chtype {
    unsafe { sys::getattrs(win) }
}

pub fn getbegx(win: WINDOW) -> i32 {
    unsafe { sys::getbegx(win) }
}

pub fn getbegy(win: WINDOW) -> i32 {
    unsafe { sys::getbegy(win) }
}

pub fn getmaxx(win: WINDOW) -> i32 {
    unsafe { sys::getmaxx(win) }
}

pub fn getmaxy(win: WINDOW) -> i32 {
    unsafe { sys::getmaxy(win) }
}

pub fn getparx(win: WINDOW) -> i32 {
    unsafe { sys::getparx(win) }
}

pub fn getpary(win: WINDOW) -> i32 {
    unsafe { sys::getpary(win) }
}

pub fn getcurx(win: WINDOW) -> i32 {
    unsafe { sys::getcurx(win) }
}

pub fn getcury(win: WINDOW) -> i32 {
    unsafe { sys::getcury(win) }
}

pub fn traceoff() {
    unsafe { sys::traceoff() }
}

pub fn traceon() {
    unsafe { sys::traceon() }
}

pub fn unctrl(c: chtype) -> &'static str {
    let s = unsafe { std::ffi::CStr::from_ptr(sys::unctrl(c)) };
    s.to_str().unwrap()
}

pub fn crmode() -> i32 {
    unsafe { sys::crmode() }
}

pub fn nocrmode() -> i32 {
    unsafe { sys::nocrmode() }
}

pub fn draino(ms: i32) -> i32 {
    unsafe { sys::draino(ms) }
}

pub fn resetterm() -> i32 {
    unsafe { sys::resetterm() }
}

pub fn fixterm() -> i32 {
    unsafe { sys::fixterm() }
}

pub fn saveterm() -> i32 {
    unsafe { sys::saveterm() }
}

pub fn setsyx(y: i32, x: i32) {
    unsafe { sys::setsyx(y, x) }
}


pub fn mouse_set(mbe: mmask_t) -> i32 {
    unsafe { sys::mouse_set(mbe) }
}

pub fn mouse_on(mbe: mmask_t) -> i32 {
    unsafe { sys::mouse_on(mbe) }
}

pub fn mouse_off(mbe: mmask_t) -> i32 {
    unsafe { sys::mouse_off(mbe) }
}

pub fn request_mouse_pos() -> i32 {
    unsafe { sys::request_mouse_pos() }
}

pub fn wmouse_position(win: WINDOW, y: &mut  i32, x: &mut  i32) {
    unsafe { sys::wmouse_position(win, y, x) }
}

pub fn getmouse() -> mmask_t {
    unsafe { sys::getmouse() }
}


pub fn assume_default_colors(fg: i32, bg: i32) -> i32 {
    unsafe { sys::assume_default_colors(fg, bg) }
}

pub fn curses_version() -> &'static str {
    let s = unsafe { std::ffi::CStr::from_ptr(sys::curses_version()) };
    s.to_str().unwrap()
}

pub fn has_key(key: i32) -> bool {
    unsafe { sys::has_key(key) }
}

pub fn is_keypad(win: WINDOW) -> bool {
    unsafe { sys::is_keypad(win) }
}

pub fn is_leaveok(win: WINDOW) -> bool {
    unsafe { sys::is_leaveok(win) }
}

pub fn is_pad(pad: WINDOW) -> bool {
    unsafe { sys::is_pad(pad) }
}

pub fn set_tabsize(tabsize: i32) -> i32 {
    unsafe { sys::set_tabsize(tabsize) }
}

pub fn use_default_colors() -> i32 {
    unsafe { sys::use_default_colors() }
}

pub fn wresize(win: WINDOW, nlines: i32, ncols: i32) -> i32 {
    unsafe { sys::wresize(win, nlines, ncols) }
}

pub fn has_mouse() -> bool {
    unsafe { sys::has_mouse() }
}

pub fn mouseinterval(wait: i32) -> i32 {
    unsafe { sys::mouseinterval(wait) }
}

pub fn mousemask(mast: mmask_t, oldmask: &mut mmask_t) -> mmask_t {
    unsafe { sys::mousemask(mast, oldmask) }
}

pub fn mouse_trafo(y: &mut i32, x: &mut i32, to_screen: bool) -> bool {
    unsafe { sys::mouse_trafo(y, x, to_screen) }
}

pub fn nc_getmouse(event: &mut MEVENT) -> i32 {
    unsafe { sys::nc_getmouse(event) }
}

pub fn ungetmouse(event: &mut MEVENT) -> i32 {
    unsafe { sys::ungetmouse(event) }
}

pub fn wenclose(win: WINDOW, y: i32, x: i32) -> bool {
    unsafe { sys::wenclose(win, y, x) }
}

pub fn wmouse_trafo(win: WINDOW, y: &mut i32, x: &mut i32, to_screen: bool) -> bool {
    unsafe { sys::wmouse_trafo(win, y, x, to_screen) }
}


pub fn addrawch(ch: chtype) -> i32 {
    unsafe { sys::addrawch(ch) }
}

pub fn insrawch(ch: chtype) -> i32 {
    unsafe { sys::insrawch(ch) }
}

pub fn is_termresized() -> bool {
    unsafe { sys::is_termresized() }
}

pub fn mvaddrawch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvaddrawch(y, x, ch) }
}

pub fn mvdeleteln(y: i32, x: i32) -> i32 {
    unsafe { sys::mvdeleteln(y, x) }
}

pub fn mvinsertln(y: i32, x: i32) -> i32 {
    unsafe { sys::mvinsertln(y, x) }
}

pub fn mvinsrawch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvinsrawch(y, x, ch) }
}

pub fn mvwaddrawch(win: WINDOW, y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvwaddrawch(win, y, x, ch) }
}

pub fn mvwdeleteln(win: WINDOW, y: i32, x: i32) -> i32 {
    unsafe { sys::mvwdeleteln(win, y, x) }
}

pub fn mvwinsertln(win: WINDOW, y: i32, x: i32) -> i32 {
    unsafe { sys::mvwinsertln(win, y, x) }
}

pub fn mvwinsrawch(win: WINDOW, y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { sys::mvwinsrawch(win, y, x, ch) }
}

pub fn raw_output(flag: bool) -> i32 {
    unsafe { sys::raw_output(flag) }
}

pub fn resize_term(nlines: i32, ncols: i32) -> i32 {
    unsafe { sys::resize_term(nlines, ncols) }
}

pub fn resize_window(win: WINDOW, nlines: i32, ncols: i32) -> WINDOW {
    unsafe { sys::resize_window(win, nlines, ncols) }
}

pub fn waddrawch(win: WINDOW, ch: chtype) -> i32 {
    unsafe { sys::waddrawch(win, ch) }
}

pub fn winsrawch(win: WINDOW, ch: chtype) -> i32 {
    unsafe { sys::winsrawch(win, ch) }
}

pub fn wordchar() -> u8 {
    unsafe { sys::wordchar() as u8 }
}

pub fn slk_wlabel(labnum: i32) -> Vec<u16> {
    unsafe { wcstr(sys::slk_wlabel(labnum)) }
}

pub fn PDC_get_version(ver: &mut PDC_VERSION) {
    unsafe { sys::PDC_get_version(ver) }
}

pub fn PDC_ungetch(ch: i32) -> i32 {
    unsafe { sys::PDC_ungetch(ch) }
}

pub fn PDC_set_blink(blinkon: bool) -> i32 {
    unsafe { sys::PDC_set_blink(blinkon) }
}

pub fn PDC_set_bold(boldon: bool) -> i32 {
    unsafe { sys::PDC_set_bold(boldon) }
}

pub fn PDC_set_line_color(color: i16) -> i32 {
    unsafe { sys::PDC_set_line_color(color) }
}

pub fn PDC_set_title(title: &str) {
    unsafe { sys::PDC_set_title(cstr(title).as_ptr()) }
}

pub fn PDC_clearclipboard() -> i32 {
    unsafe { sys::PDC_clearclipboard() }
}

pub fn PDC_getclipboard(contents: &mut Vec<u8>) -> i32 {
    let mut ptr: *mut i8 = core::ptr::null_mut();
    let mut len: i32 = 0;

    if sys::ERR == unsafe {
        sys::PDC_getclipboard(&mut ptr, &mut len)
    } {
        return sys::ERR;
    }

    contents.clear();
    contents.reserve(len as usize);
    contents.extend_from_slice(unsafe {
        core::slice::from_raw_parts(ptr as *const u8, len as usize)
    });

    unsafe {
        sys::PDC_freeclipboard(ptr);
    }

    sys::OK
}

pub fn PDC_setclipboard(contents: &[u8]) -> i32 {
    unsafe { sys::PDC_setclipboard(contents.as_ptr() as *const i8, contents.len() as i32) }
}

pub fn PDC_get_key_modifiers() -> u32 {
    unsafe { sys::PDC_get_key_modifiers() }
}

pub fn PDC_return_key_modifiers(flag: bool) -> i32 {
    unsafe { sys::PDC_return_key_modifiers(flag) }
}


pub fn touchoverlap(win1: WINDOW, win2: WINDOW) -> i32 {
    unsafe { sys::touchoverlap(win1, win2) }
}

pub fn underend() -> i32 {
    unsafe { sys::underend() }
}

pub fn underscore() -> i32 {
    unsafe { sys::underscore() }
}

pub fn wunderend(win: WINDOW) -> i32 {
    unsafe { sys::wunderend(win) }
}

pub fn wunderscore(win: WINDOW) -> i32 {
    unsafe { sys::wunderscore(win) }
}


pub fn getbegyx(win: WINDOW, y: &mut i32, x: &mut i32) {
    unsafe { sys::getbegyx(win, y, x) }
}

pub fn getmaxyx(win: WINDOW, y: &mut i32, x: &mut i32) {
    unsafe { sys::getmaxyx(win, y, x) }
}

pub fn getparyx(win: WINDOW, y: &mut i32, x: &mut i32) {
    unsafe { sys::getparyx(win, y, x) }
}

pub fn getyx(win: WINDOW, y: &mut i32, x: &mut i32) {
    unsafe { sys::getyx(win, y, x) }
}

pub fn getsyx(y: &mut i32, x: &mut i32) {
    unsafe { sys::getsyx(y, x) }
}


pub mod panel {
    use super::sys;
    use super::WINDOW;
    use core::ffi::c_void;

    pub use sys::panel::{
        PANELOBS,
        PANEL,
    };

    pub fn bottom_panel(pan: *mut PANEL) -> i32 {
        unsafe { sys::panel::bottom_panel(pan) }
    }

    pub fn del_panel(pan: *mut PANEL) -> i32 {
        unsafe { sys::panel::del_panel(pan) }
    }

    pub fn hide_panel(pan: *mut PANEL) -> i32 {
        unsafe { sys::panel::hide_panel(pan) }
    }

    pub fn move_panel(pan: *mut PANEL, starty: i32, startx: i32) -> i32 {
        unsafe { sys::panel::move_panel(pan, starty, startx) }
    }

    pub fn new_panel(win: WINDOW) -> *mut PANEL {
        unsafe { sys::panel::new_panel(win) }
    }

    pub fn panel_above(pan: *const PANEL) -> *mut PANEL {
        unsafe { sys::panel::panel_above(pan) }
    }

    pub fn panel_below(pan: *const PANEL) -> *mut PANEL {
        unsafe { sys::panel::panel_below(pan) }
    }

    pub fn panel_hidden(pan: *const PANEL) -> i32 {
        unsafe { sys::panel::panel_hidden(pan) }
    }

    pub fn panel_userptr(pan: *const PANEL) -> *const c_void {
        unsafe { sys::panel::panel_userptr(pan) }
    }

    pub fn panel_window(pan: *const PANEL) -> WINDOW {
        unsafe { sys::panel::panel_window(pan) }
    }

    pub fn replace_panel(pan: *mut PANEL, win: WINDOW) -> i32 {
        unsafe { sys::panel::replace_panel(pan, win) }
    }

    pub fn set_panel_userptr(pan: *mut PANEL, user: *const c_void) -> i32 {
        unsafe { sys::panel::set_panel_userptr(pan, user) }
    }

    pub fn show_panel(pan: *mut PANEL) -> i32 {
        unsafe { sys::panel::show_panel(pan) }
    }

    pub fn top_panel(pan: *mut PANEL) -> i32 {
        unsafe { sys::panel::top_panel(pan) }
    }

    pub fn update_panels() {
        unsafe { sys::panel::update_panels() }
    }
}
