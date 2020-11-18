//! Low-level bindings to PDCurses.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/*----------------------------------------------------------------------*/

use libc::{ FILE, wchar_t, c_char, c_uchar, c_int, c_short, c_long, c_ulong, c_ushort, c_void };

/* Windows-specific types figured out by bindgen 0.54.0 */

#[doc(hidden)]
mod details {
    pub type va_list = *mut libc::c_char;
}

#[doc(hidden)]
use details::*;

pub type wint_t = c_ushort;

/*
    Comments (after this one) in this file are original PDCurses comments.
*/

/*----------------------------------------------------------------------*/

pub const PDCURSES     : bool  = true;  /* PDCurses-only features are available */
pub const PDC_BUILD    : c_int = 3905;  /* API build version */
pub const PDC_VER_MAJOR: c_int = 3;     /* major version number */
pub const PDC_VER_MINOR: c_int = 9;     /* minor version number */
pub const PDC_VERDOT   : &str  = "3.9"; /* version string */

pub const CHTYPE_LONG  : bool  = true;  /* chtype >= 32 bits */

/*----------------------------------------------------------------------
 *
 *  Constants and Types
 *
 */

pub const FALSE: c_int = 0;
pub const TRUE : c_int = 1;
pub const ERR  : c_int = -1;
pub const OK   : c_int = 0;

pub type chtype = c_ulong;

pub type cchar_t = chtype;
pub type  attr_t = chtype;

/*----------------------------------------------------------------------
 *
 *  Version Info
 *
 */

/* Use this structure with PDC_get_version() for run-time info about the
   way the library was built, in case it doesn't match the header. */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PDC_VERSION {
    pub flags: c_short,  /* flags OR'd together (see below) */
    pub build: c_short,  /* PDC_BUILD at compile time */
    pub major: c_uchar,  /* PDC_VER_MAJOR */
    pub minor: c_uchar,  /* PDC_VER_MINOR */
    pub csize: c_uchar,  /* sizeof chtype */
    pub bsize: c_uchar,  /* sizeof bool */
}

pub const PDC_VFLAG_DEBUG: c_short = 0x01;  /* set if built with -DPDCDEBUG */
pub const PDC_VFLAG_WIDE : c_short = 0x02;  /* -DPDC_WIDE */
pub const PDC_VFLAG_UTF8 : c_short = 0x04;  /* -DPDC_FORCE_UTF8 */
pub const PDC_VFLAG_DLL  : c_short = 0x08;  /* -DPDC_DLL_BUILD */
pub const PDC_VFLAG_RGB  : c_short = 0x10;  /* -DPDC_RGB */

/*----------------------------------------------------------------------
 *
 *  Mouse Interface
 *
 */

pub type mmask_t = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MOUSE_STATUS {
    pub x: c_int,                   /* absolute column, 0 based, measured in characters */
    pub y: c_int,                   /* absolute row, 0 based, measured in characters */
    pub button: [c_short; 3usize],  /* state of each button */
    pub changes: c_int,             /* flags indicating what has changed with the mouse */
}

pub use mouse_constants::*;
pub mod mouse_constants
{
    use super::*;

    pub const BUTTON_RELEASED      : c_short = 0x0000;
    pub const BUTTON_PRESSED       : c_short = 0x0001;
    pub const BUTTON_CLICKED       : c_short = 0x0002;
    pub const BUTTON_DOUBLE_CLICKED: c_short = 0x0003;
    pub const BUTTON_TRIPLE_CLICKED: c_short = 0x0004;
    pub const BUTTON_MOVED         : c_short = 0x0005;  /* PDCurses */
    pub const WHEEL_SCROLLED       : c_short = 0x0006;  /* PDCurses */
    pub const BUTTON_ACTION_MASK   : c_short = 0x0007;  /* PDCurses */

    pub const PDC_BUTTON_SHIFT     : c_short = 0x0008;  /* PDCurses */
    pub const PDC_BUTTON_CONTROL   : c_short = 0x0010;  /* PDCurses */
    pub const PDC_BUTTON_ALT       : c_short = 0x0020;  /* PDCurses */
    pub const BUTTON_MODIFIER_MASK : c_short = 0x0038;  /* PDCurses */

    /*
    * Bits associated with the MOUSE_STATUS .changes field:
    *   3         2         1         0
    * 210987654321098765432109876543210
    *                                 1 <- button 1 has changed
    *                                10 <- button 2 has changed
    *                               100 <- button 3 has changed
    *                              1000 <- mouse has moved
    *                             10000 <- mouse position report
    *                            100000 <- mouse wheel up
    *                           1000000 <- mouse wheel down
    *                          10000000 <- mouse wheel left
    *                         100000000 <- mouse wheel right
    */

    pub const PDC_MOUSE_MOVED      : c_int = 0x0008;
    pub const PDC_MOUSE_POSITION   : c_int = 0x0010;
    pub const PDC_MOUSE_WHEEL_UP   : c_int = 0x0020;
    pub const PDC_MOUSE_WHEEL_DOWN : c_int = 0x0040;
    pub const PDC_MOUSE_WHEEL_LEFT : c_int = 0x0080;
    pub const PDC_MOUSE_WHEEL_RIGHT: c_int = 0x0100;

    /* mouse bit-masks */

    pub const BUTTON1_RELEASED      : mmask_t = 0x00000001;
    pub const BUTTON1_PRESSED       : mmask_t = 0x00000002;
    pub const BUTTON1_CLICKED       : mmask_t = 0x00000004;
    pub const BUTTON1_DOUBLE_CLICKED: mmask_t = 0x00000008;
    pub const BUTTON1_TRIPLE_CLICKED: mmask_t = 0x00000010;
    pub const BUTTON1_MOVED         : mmask_t = 0x00000010; /* PDCurses */

    pub const BUTTON2_RELEASED      : mmask_t = 0x00000020;
    pub const BUTTON2_PRESSED       : mmask_t = 0x00000040;
    pub const BUTTON2_CLICKED       : mmask_t = 0x00000080;
    pub const BUTTON2_DOUBLE_CLICKED: mmask_t = 0x00000100;
    pub const BUTTON2_TRIPLE_CLICKED: mmask_t = 0x00000200;
    pub const BUTTON2_MOVED         : mmask_t = 0x00000200; /* PDCurses */

    pub const BUTTON3_RELEASED      : mmask_t = 0x00000400;
    pub const BUTTON3_PRESSED       : mmask_t = 0x00000800;
    pub const BUTTON3_CLICKED       : mmask_t = 0x00001000;
    pub const BUTTON3_DOUBLE_CLICKED: mmask_t = 0x00002000;
    pub const BUTTON3_TRIPLE_CLICKED: mmask_t = 0x00004000;
    pub const BUTTON3_MOVED         : mmask_t = 0x00004000; /* PDCurses */

    /* For the ncurses-compatible functions only, BUTTON4_PRESSED and
    BUTTON5_PRESSED are returned for mouse scroll wheel up and down;
    otherwise PDCurses doesn't support buttons 4 and 5 */

    pub const BUTTON4_RELEASED      : mmask_t = 0x00008000;
    pub const BUTTON4_PRESSED       : mmask_t = 0x00010000;
    pub const BUTTON4_CLICKED       : mmask_t = 0x00020000;
    pub const BUTTON4_DOUBLE_CLICKED: mmask_t = 0x00040000;
    pub const BUTTON4_TRIPLE_CLICKED: mmask_t = 0x00080000;

    pub const BUTTON5_RELEASED      : mmask_t = 0x00100000;
    pub const BUTTON5_PRESSED       : mmask_t = 0x00200000;
    pub const BUTTON5_CLICKED       : mmask_t = 0x00400000;
    pub const BUTTON5_DOUBLE_CLICKED: mmask_t = 0x00800000;
    pub const BUTTON5_TRIPLE_CLICKED: mmask_t = 0x01000000;

    pub const MOUSE_WHEEL_SCROLL    : mmask_t = 0x02000000; /* PDCurses */
    pub const BUTTON_MODIFIER_SHIFT : mmask_t = 0x04000000; /* PDCurses */
    pub const BUTTON_MODIFIER_CONTRO: mmask_t = 0x08000000; /* PDCurses */
    pub const BUTTON_MODIFIER_ALT   : mmask_t = 0x10000000; /* PDCurses */

    pub const ALL_MOUSE_EVENTS      : mmask_t = 0x1fffffff;
    pub const REPORT_MOUSE_POSITION : mmask_t = 0x20000000;

    /* ncurses mouse interface */

    pub const BUTTON_SHIFT  : c_short = PDC_BUTTON_SHIFT;
    pub const BUTTON_CONTROL: c_short = PDC_BUTTON_CONTROL;
    pub const BUTTON_ALT    : c_short = PDC_BUTTON_ALT;
}

pub unsafe fn MOUSE_X_POS() -> c_int { Mouse_status.x }
pub unsafe fn MOUSE_Y_POS() -> c_int { Mouse_status.y }

pub unsafe fn A_BUTTON_CHANGED ()         -> c_int   {      Mouse_status.changes & 7 }
pub unsafe fn MOUSE_MOVED      ()         -> bool    { 0 != Mouse_status.changes & PDC_MOUSE_MOVED }
pub unsafe fn MOUSE_POS_REPORT ()         -> bool    { 0 != Mouse_status.changes & PDC_MOUSE_POSITION }
pub unsafe fn BUTTON_CHANGED   (x: c_int) -> bool    { 0 != Mouse_status.changes & (1 << (x - 1)) }
pub unsafe fn BUTTON_STATUS    (x: c_int) -> c_short {      Mouse_status.button[(x as usize) - 1] }
pub unsafe fn MOUSE_WHEEL_UP   ()         -> bool    { 0 != Mouse_status.changes & PDC_MOUSE_WHEEL_UP }
pub unsafe fn MOUSE_WHEEL_DOWN ()         -> bool    { 0 != Mouse_status.changes & PDC_MOUSE_WHEEL_DOWN }
pub unsafe fn MOUSE_WHEEL_LEFT ()         -> bool    { 0 != Mouse_status.changes & PDC_MOUSE_WHEEL_LEFT }
pub unsafe fn MOUSE_WHEEL_RIGHT()         -> bool    { 0 != Mouse_status.changes & PDC_MOUSE_WHEEL_RIGHT }

/* ncurses mouse interface */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MEVENT {
    pub id: c_short,  /* unused, always 0 */
    pub x: c_int,     /* same as MOUSE_STATUS */
    pub y: c_int,     /* same as MOUSE_STATUS */
    pub z: c_int,     /* unused */
    pub bstate: mmask_t,
}

/*----------------------------------------------------------------------
 *
 *  Window and Screen Structures
 *
 */

/* definition of a window */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WINDOW {
    pub _cury: c_int,          /* current pseudo-cursor */
    pub _curx: c_int,
    pub _maxy: c_int,          /* max window coordinates */
    pub _maxx: c_int,
    pub _begy: c_int,          /* origin on screen */
    pub _begx: c_int,
    pub _flags: c_int,         /* window properties */
    pub _attrs: chtype,        /* standard attributes and colors */
    pub _bkgd: chtype,         /* background, normally blank */
    pub _clear: bool,          /* causes clear at next refresh */
    pub _leaveit: bool,        /* leaves cursor where it is */
    pub _scroll: bool,         /* allows window scrolling */
    pub _nodelay: bool,        /* input character wait flag */
    pub _immed: bool,          /* immediate update flag */
    pub _sync: bool,           /* synchronise window ancestors */
    pub _use_keypad: bool,     /* flags keypad key mode active */
    pub _y: *mut *mut chtype,  /* pointer to line pointer array */
    pub _firstch: *mut c_int,  /* first changed character in line */
    pub _lastch: *mut c_int,   /* last changed character in line */
    pub _tmarg: c_int,         /* top of scrolling region */
    pub _bmarg: c_int,         /* bottom of scrolling region */
    pub _delayms: c_int,       /* milliseconds of delay for getch() */
    pub _parx: c_int,          /* coords relative to parent (0,0) */
    pub _pary: c_int,          /* coords relative to parent (0,0) */
    pub _parent: *mut WINDOW,  /* subwin's pointer to parent win */
}

/* Color pair structure */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PDC_PAIR {
    pub f: c_short,  /* foreground color */
    pub b: c_short,  /* background color */
    pub set: bool,   /* pair has been set */
}

/* Avoid using the SCREEN struct directly -- use the corresponding
   functions if possible. This struct may eventually be made private. */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCREEN {
    pub alive: bool,                 /* if initscr() called, and not endwin() */
    pub autocr: bool,                /* if cr -> lf */
    pub cbreak: bool,                /* if terminal unbuffered */
    pub echo: bool,                  /* if terminal echo */
    pub raw_inp: bool,               /* raw input mode (v. cooked input) */
    pub raw_out: bool,               /* raw output mode (7 v. 8 bits) */
    pub audible: bool,               /* FALSE if the bell is visual */
    pub mono: bool,                  /* TRUE if current screen is mono */
    pub resized: bool,               /* TRUE if TERM has been resized */
    pub orig_attr: bool,             /* TRUE if we have the original colors */
    pub orig_fore: c_short,          /* original screen foreground color */
    pub orig_back: c_short,          /* original screen foreground color */
    pub cursrow: c_int,              /* position of physical cursor */
    pub curscol: c_int,              /* position of physical cursor */
    pub visibility: c_int,           /* visibility of cursor */
    pub orig_cursor: c_int,          /* original cursor size */
    pub lines: c_int,                /* new value for LINES */
    pub cols: c_int,                 /* new value for COLS */
    pub _trap_mbe: mmask_t,          /* trap these mouse button events */
    pub mouse_wait: c_int,           /* time to wait (in ms) for a
                                        button release after a press, in
                                        order to count it as a click */
    pub slklines: c_int,             /* lines in use by slk_init() */
    pub slk_winptr: *mut WINDOW,     /* window for slk */
    pub linesrippedoff: c_int,       /* lines ripped off via ripoffline() */
    pub linesrippedoffontop: c_int,  /* lines ripped off on
                                        top via ripoffline() */
    pub delaytenths: c_int,          /* 1/10ths second to wait block
                                        getch() for */
    pub _preserve: bool,             /* TRUE if screen background
                                        to be preserved */
    pub _restore: c_int,             /* specifies if screen background
                                        to be restored, and how */
    pub key_modifiers: c_ulong,      /* key modifiers (SHIFT, CONTROL, etc.)
                                        on last key press */
    pub return_key_modifiers: bool,  /* TRUE if modifier keys are
                                        returned as "real" keys */
    pub key_code: bool,              /* TRUE if last key is a special key;
                                         used internally by get_wch() */
    pub mouse_status: MOUSE_STATUS,  /* last returned mouse status */
    pub line_color: c_short,         /* color of line attributes - default -1 */
    pub termattrs: attr_t,           /* attribute capabilities */
    pub lastscr: *mut WINDOW,        /* the last screen image */
    pub dbfp: *mut FILE,             /* debug trace file pointer */
    pub color_started: bool,         /* TRUE after start_color() */
    pub dirty: bool,                 /* redraw on napms() after init_color() */
    pub sel_start: c_int,            /* start of selection (y * COLS + x) */
    pub sel_end: c_int,              /* end of selection */
    pub c_buffer: *mut c_int,        /* character buffer */
    pub c_pindex: c_int,             /* putter index */
    pub c_gindex: c_int,             /* getter index */
    pub c_ungch: *mut c_int,         /* array of ungotten chars */
    pub c_ungind: c_int,             /* ungetch() push index */
    pub c_ungmax: c_int,             /* allocated size of ungetch() buffer */
    pub atrtab: *mut PDC_PAIR,       /* table of color pairs */
}

/*----------------------------------------------------------------------
 *
 *  External Variables
 *
 */

extern "C" {
    pub static mut LINES       : c_int;        /* terminal height */
    pub static mut COLS        : c_int;        /* terminal width */
    pub static mut stdscr      : *mut WINDOW;  /* the default screen window */
    pub static mut curscr      : *mut WINDOW;  /* the current screen image */
    pub static mut SP          : *mut SCREEN;  /* curses variables */
    pub static mut Mouse_status: MOUSE_STATUS;
    pub static mut COLORS      : c_int;
    pub static mut COLOR_PAIRS : c_int;
    pub static mut TABSIZE     : c_int;
    pub static     acs_map     : *mut chtype;  /* alternate character set map */
    pub static     ttytype     : *mut c_char;  /* terminal name/description */
}

/*man-start**************************************************************

Text Attributes
===============

PDCurses uses a 32-bit integer for its chtype:

    +--------------------------------------------------------------------+
    |31|30|29|28|27|26|25|24|23|22|21|20|19|18|17|16|15|14|13|..| 2| 1| 0|
    +--------------------------------------------------------------------+
          color pair        |     modifiers         |   character eg 'a'

There are 256 color pairs (8 bits), 8 bits for modifiers, and 16 bits
for character data. The modifiers are bold, underline, right-line,
left-line, italic, reverse and blink, plus the alternate character set
indicator.

**man-end****************************************************************/

pub use attribute_constants::*;
pub mod attribute_constants
{
    use super::*;

    /*** Video attribute macros ***/

    pub const A_NORMAL       : chtype = 0;

    pub const A_ALTCHARSET   : chtype = 0x00010000;
    pub const A_RIGHT        : chtype = 0x00020000;
    pub const A_LEFT         : chtype = 0x00040000;
    pub const A_ITALIC       : chtype = 0x00080000;
    pub const A_UNDERLINE    : chtype = 0x00100000;
    pub const A_REVERSE      : chtype = 0x00200000;
    pub const A_BLINK        : chtype = 0x00400000;
    pub const A_BOLD         : chtype = 0x00800000;

    pub const A_ATTRIBUTES   : chtype = 0xffff0000;
    pub const A_CHARTEXT     : chtype = 0x0000ffff;
    pub const A_COLOR        : chtype = 0xff000000;

    pub const PDC_COLOR_SHIFT: chtype = 24;

    pub const A_LEFTLINE     : chtype = A_LEFT;
    pub const A_RIGHTLINE    : chtype = A_RIGHT;
    pub const A_STANDOUT     : chtype = A_REVERSE | A_BOLD; /* X/Open */

    pub const A_DIM          : chtype = A_NORMAL;
    pub const A_INVIS        : chtype = A_NORMAL;
    pub const A_PROTECT      : chtype = A_NORMAL;

    pub const A_HORIZONTAL   : chtype = A_NORMAL;
    pub const A_LOW          : chtype = A_NORMAL;
    pub const A_TOP          : chtype = A_NORMAL;
    pub const A_VERTICAL     : chtype = A_NORMAL;

    pub const CHR_MSK        : chtype = A_CHARTEXT;    /* Obsolete */
    pub const ATR_MSK        : chtype = A_ATTRIBUTES;  /* Obsolete */
    pub const ATR_NRM        : chtype = A_NORMAL;      /* Obsolete */

    /* For use with attr_t -- X/Open says, "these shall be distinct", so
    this is a non-conforming implementation. */

    pub const WA_NORMAL      : chtype = A_NORMAL;

    pub const WA_ALTCHARSET  : chtype = A_ALTCHARSET;
    pub const WA_BLINK       : chtype = A_BLINK;
    pub const WA_BOLD        : chtype = A_BOLD;
    pub const WA_DIM         : chtype = A_DIM;
    pub const WA_INVIS       : chtype = A_INVIS;
    pub const WA_ITALIC      : chtype = A_ITALIC;
    pub const WA_LEFT        : chtype = A_LEFT;
    pub const WA_PROTECT     : chtype = A_PROTECT;
    pub const WA_REVERSE     : chtype = A_REVERSE;
    pub const WA_RIGHT       : chtype = A_RIGHT;
    pub const WA_STANDOUT    : chtype = A_STANDOUT;
    pub const WA_UNDERLINE   : chtype = A_UNDERLINE;

    pub const WA_HORIZONTAL  : chtype = A_HORIZONTAL;
    pub const WA_LOW         : chtype = A_LOW;
    pub const WA_TOP         : chtype = A_TOP;
    pub const WA_VERTICAL    : chtype = A_VERTICAL;

    pub const WA_ATTRIBUTES  : chtype = A_ATTRIBUTES;
}

/*** Alternate character set macros ***/

pub const fn PDC_ACS(w: chtype) -> chtype { w | A_ALTCHARSET }

/* VT100-compatible symbols -- box chars */

pub const fn ACS_ULCORNER() -> chtype { PDC_ACS(b'l' as chtype) }
pub const fn ACS_LLCORNER() -> chtype { PDC_ACS(b'm' as chtype) }
pub const fn ACS_URCORNER() -> chtype { PDC_ACS(b'k' as chtype) }
pub const fn ACS_LRCORNER() -> chtype { PDC_ACS(b'j' as chtype) }
pub const fn ACS_RTEE    () -> chtype { PDC_ACS(b'u' as chtype) }
pub const fn ACS_LTEE    () -> chtype { PDC_ACS(b't' as chtype) }
pub const fn ACS_BTEE    () -> chtype { PDC_ACS(b'v' as chtype) }
pub const fn ACS_TTEE    () -> chtype { PDC_ACS(b'w' as chtype) }
pub const fn ACS_HLINE   () -> chtype { PDC_ACS(b'q' as chtype) }
pub const fn ACS_VLINE   () -> chtype { PDC_ACS(b'x' as chtype) }
pub const fn ACS_PLUS    () -> chtype { PDC_ACS(b'n' as chtype) }

/* VT100-compatible symbols -- other */

pub const fn ACS_S1      () -> chtype { PDC_ACS(b'o' as chtype) }
pub const fn ACS_S9      () -> chtype { PDC_ACS(b's' as chtype) }
pub const fn ACS_DIAMOND () -> chtype { PDC_ACS(b'`' as chtype) }
pub const fn ACS_CKBOARD () -> chtype { PDC_ACS(b'a' as chtype) }
pub const fn ACS_DEGREE  () -> chtype { PDC_ACS(b'f' as chtype) }
pub const fn ACS_PLMINUS () -> chtype { PDC_ACS(b'g' as chtype) }
pub const fn ACS_BULLET  () -> chtype { PDC_ACS(b'~' as chtype) }

/* Teletype 5410v1 symbols -- these are defined in SysV curses, but
   are not well-supported by most terminals. Stick to VT100 characters
   for optimum portability. */

pub const fn ACS_LARROW  () -> chtype { PDC_ACS(b',' as chtype) }
pub const fn ACS_RARROW  () -> chtype { PDC_ACS(b'+' as chtype) }
pub const fn ACS_DARROW  () -> chtype { PDC_ACS(b'.' as chtype) }
pub const fn ACS_UARROW  () -> chtype { PDC_ACS(b'-' as chtype) }
pub const fn ACS_BOARD   () -> chtype { PDC_ACS(b'h' as chtype) }
pub const fn ACS_LANTERN () -> chtype { PDC_ACS(b'i' as chtype) }
pub const fn ACS_BLOCK   () -> chtype { PDC_ACS(b'0' as chtype) }

/* That goes double for these -- undocumented SysV symbols. Don't use
   them. */

pub const fn ACS_S3      () -> chtype { PDC_ACS(b'p' as chtype) }
pub const fn ACS_S7      () -> chtype { PDC_ACS(b'r' as chtype) }
pub const fn ACS_LEQUAL  () -> chtype { PDC_ACS(b'y' as chtype) }
pub const fn ACS_GEQUAL  () -> chtype { PDC_ACS(b'z' as chtype) }
pub const fn ACS_PI      () -> chtype { PDC_ACS(b'{' as chtype) }
pub const fn ACS_NEQUAL  () -> chtype { PDC_ACS(b'|' as chtype) }
pub const fn ACS_STERLING() -> chtype { PDC_ACS(b'}' as chtype) }

/* Box char aliases */

pub const fn ACS_BSSB    () -> chtype { ACS_ULCORNER() }
pub const fn ACS_SSBB    () -> chtype { ACS_LLCORNER() }
pub const fn ACS_BBSS    () -> chtype { ACS_URCORNER() }
pub const fn ACS_SBBS    () -> chtype { ACS_LRCORNER() }
pub const fn ACS_SBSS    () -> chtype { ACS_RTEE    () }
pub const fn ACS_SSSB    () -> chtype { ACS_LTEE    () }
pub const fn ACS_SSBS    () -> chtype { ACS_BTEE    () }
pub const fn ACS_BSSS    () -> chtype { ACS_TTEE    () }
pub const fn ACS_BSBS    () -> chtype { ACS_HLINE   () }
pub const fn ACS_SBSB    () -> chtype { ACS_VLINE   () }
pub const fn ACS_SSSS    () -> chtype { ACS_PLUS    () }

/* cchar_t aliases */

pub fn WACS_ULCORNER() -> *mut cchar_t { unsafe { acs_map.add(b'l' as usize) } }
pub fn WACS_LLCORNER() -> *mut cchar_t { unsafe { acs_map.add(b'm' as usize) } }
pub fn WACS_URCORNER() -> *mut cchar_t { unsafe { acs_map.add(b'k' as usize) } }
pub fn WACS_LRCORNER() -> *mut cchar_t { unsafe { acs_map.add(b'j' as usize) } }
pub fn WACS_RTEE    () -> *mut cchar_t { unsafe { acs_map.add(b'u' as usize) } }
pub fn WACS_LTEE    () -> *mut cchar_t { unsafe { acs_map.add(b't' as usize) } }
pub fn WACS_BTEE    () -> *mut cchar_t { unsafe { acs_map.add(b'v' as usize) } }
pub fn WACS_TTEE    () -> *mut cchar_t { unsafe { acs_map.add(b'w' as usize) } }
pub fn WACS_HLINE   () -> *mut cchar_t { unsafe { acs_map.add(b'q' as usize) } }
pub fn WACS_VLINE   () -> *mut cchar_t { unsafe { acs_map.add(b'x' as usize) } }
pub fn WACS_PLUS    () -> *mut cchar_t { unsafe { acs_map.add(b'n' as usize) } }

pub fn WACS_S1      () -> *mut cchar_t { unsafe { acs_map.add(b'o' as usize) } }
pub fn WACS_S9      () -> *mut cchar_t { unsafe { acs_map.add(b's' as usize) } }
pub fn WACS_DIAMOND () -> *mut cchar_t { unsafe { acs_map.add(b'`' as usize) } }
pub fn WACS_CKBOARD () -> *mut cchar_t { unsafe { acs_map.add(b'a' as usize) } }
pub fn WACS_DEGREE  () -> *mut cchar_t { unsafe { acs_map.add(b'f' as usize) } }
pub fn WACS_PLMINUS () -> *mut cchar_t { unsafe { acs_map.add(b'g' as usize) } }
pub fn WACS_BULLET  () -> *mut cchar_t { unsafe { acs_map.add(b'~' as usize) } }

pub fn WACS_LARROW  () -> *mut cchar_t { unsafe { acs_map.add(b',' as usize) } }
pub fn WACS_RARROW  () -> *mut cchar_t { unsafe { acs_map.add(b'+' as usize) } }
pub fn WACS_DARROW  () -> *mut cchar_t { unsafe { acs_map.add(b'.' as usize) } }
pub fn WACS_UARROW  () -> *mut cchar_t { unsafe { acs_map.add(b'-' as usize) } }
pub fn WACS_BOARD   () -> *mut cchar_t { unsafe { acs_map.add(b'h' as usize) } }
pub fn WACS_LANTERN () -> *mut cchar_t { unsafe { acs_map.add(b'i' as usize) } }
pub fn WACS_BLOCK   () -> *mut cchar_t { unsafe { acs_map.add(b'0' as usize) } }

pub fn WACS_S3      () -> *mut cchar_t { unsafe { acs_map.add(b'p' as usize) } }
pub fn WACS_S7      () -> *mut cchar_t { unsafe { acs_map.add(b'r' as usize) } }
pub fn WACS_LEQUAL  () -> *mut cchar_t { unsafe { acs_map.add(b'y' as usize) } }
pub fn WACS_GEQUAL  () -> *mut cchar_t { unsafe { acs_map.add(b'z' as usize) } }
pub fn WACS_PI      () -> *mut cchar_t { unsafe { acs_map.add(b'{' as usize) } }
pub fn WACS_NEQUAL  () -> *mut cchar_t { unsafe { acs_map.add(b'|' as usize) } }
pub fn WACS_STERLING() -> *mut cchar_t { unsafe { acs_map.add(b'}' as usize) } }

pub fn WACS_BSSB    () -> *mut cchar_t { WACS_ULCORNER() }
pub fn WACS_SSBB    () -> *mut cchar_t { WACS_LLCORNER() }
pub fn WACS_BBSS    () -> *mut cchar_t { WACS_URCORNER() }
pub fn WACS_SBBS    () -> *mut cchar_t { WACS_LRCORNER() }
pub fn WACS_SBSS    () -> *mut cchar_t { WACS_RTEE    () }
pub fn WACS_SSSB    () -> *mut cchar_t { WACS_LTEE    () }
pub fn WACS_SSBS    () -> *mut cchar_t { WACS_BTEE    () }
pub fn WACS_BSSS    () -> *mut cchar_t { WACS_TTEE    () }
pub fn WACS_BSBS    () -> *mut cchar_t { WACS_HLINE   () }
pub fn WACS_SBSB    () -> *mut cchar_t { WACS_VLINE   () }
pub fn WACS_SSSS    () -> *mut cchar_t { WACS_PLUS    () }

pub use color_constants::*;
pub mod color_constants
{
    use super::*;

    /*** Color macros ***/

    pub const COLOR_BLACK  : c_short = 0;
    pub const COLOR_RED    : c_short = 1;
    pub const COLOR_GREEN  : c_short = 2;
    pub const COLOR_BLUE   : c_short = 4;
    pub const COLOR_CYAN   : c_short = COLOR_BLUE | COLOR_GREEN; /* 6 */
    pub const COLOR_MAGENTA: c_short = COLOR_RED  | COLOR_BLUE;  /* 5 */
    pub const COLOR_YELLOW : c_short = COLOR_RED  | COLOR_GREEN; /* 3 */
    pub const COLOR_WHITE  : c_short = 7;
}


/*----------------------------------------------------------------------
 *
 *  Function and Keypad Key Definitions
 *  Many are just for compatibility
 *
 */

pub use key_constants::*;
pub mod key_constants
{
    use super::*;

    pub const KEY_CODE_YES : c_int = 0x100;  /* If get_wch() gives a key code */

    pub const KEY_BREAK    : c_int = 0x101;  /* Not on PC KBD */
    pub const KEY_DOWN     : c_int = 0x102;  /* Down arrow key */
    pub const KEY_UP       : c_int = 0x103;  /* Up arrow key */
    pub const KEY_LEFT     : c_int = 0x104;  /* Left arrow key */
    pub const KEY_RIGHT    : c_int = 0x105;  /* Right arrow key */
    pub const KEY_HOME     : c_int = 0x106;  /* home key */
    pub const KEY_BACKSPACE: c_int = 0x107;  /* not on pc */
    pub const KEY_F0       : c_int = 0x108;  /* function keys; 64 reserved */

    /* Fn keys constants below are not part of PDCurses and are provided by this crate for convenience */

    pub const KEY_F1       : c_int = 0x109;
    pub const KEY_F2       : c_int = 0x10a;
    pub const KEY_F3       : c_int = 0x10b;
    pub const KEY_F4       : c_int = 0x10c;
    pub const KEY_F5       : c_int = 0x10d;
    pub const KEY_F6       : c_int = 0x10e;
    pub const KEY_F7       : c_int = 0x10f;
    pub const KEY_F8       : c_int = 0x110;
    pub const KEY_F9       : c_int = 0x111;
    pub const KEY_F10      : c_int = 0x112;
    pub const KEY_F11      : c_int = 0x113;
    pub const KEY_F12      : c_int = 0x114;
    pub const KEY_F13      : c_int = 0x115;
    pub const KEY_F14      : c_int = 0x116;
    pub const KEY_F15      : c_int = 0x117;
    pub const KEY_F16      : c_int = 0x118;
    pub const KEY_F17      : c_int = 0x119;
    pub const KEY_F18      : c_int = 0x11a;
    pub const KEY_F19      : c_int = 0x11b;
    pub const KEY_F20      : c_int = 0x11c;
    pub const KEY_F21      : c_int = 0x11d;
    pub const KEY_F22      : c_int = 0x11e;
    pub const KEY_F23      : c_int = 0x11f;
    pub const KEY_F24      : c_int = 0x120;
    pub const KEY_F25      : c_int = 0x121;
    pub const KEY_F26      : c_int = 0x122;
    pub const KEY_F27      : c_int = 0x123;
    pub const KEY_F28      : c_int = 0x124;
    pub const KEY_F29      : c_int = 0x125;
    pub const KEY_F30      : c_int = 0x126;
    pub const KEY_F31      : c_int = 0x127;
    pub const KEY_F32      : c_int = 0x128;
    pub const KEY_F33      : c_int = 0x129;
    pub const KEY_F34      : c_int = 0x12a;
    pub const KEY_F35      : c_int = 0x12b;
    pub const KEY_F36      : c_int = 0x12c;
    pub const KEY_F37      : c_int = 0x12d;
    pub const KEY_F38      : c_int = 0x12e;
    pub const KEY_F39      : c_int = 0x12f;
    pub const KEY_F40      : c_int = 0x130;
    pub const KEY_F41      : c_int = 0x131;
    pub const KEY_F42      : c_int = 0x132;
    pub const KEY_F43      : c_int = 0x133;
    pub const KEY_F44      : c_int = 0x134;
    pub const KEY_F45      : c_int = 0x135;
    pub const KEY_F46      : c_int = 0x136;
    pub const KEY_F47      : c_int = 0x137;
    pub const KEY_F48      : c_int = 0x138;
    pub const KEY_F49      : c_int = 0x139;
    pub const KEY_F50      : c_int = 0x13a;
    pub const KEY_F51      : c_int = 0x13b;
    pub const KEY_F52      : c_int = 0x13c;
    pub const KEY_F53      : c_int = 0x13d;
    pub const KEY_F54      : c_int = 0x13e;
    pub const KEY_F55      : c_int = 0x13f;
    pub const KEY_F56      : c_int = 0x140;
    pub const KEY_F57      : c_int = 0x141;
    pub const KEY_F58      : c_int = 0x142;
    pub const KEY_F59      : c_int = 0x143;
    pub const KEY_F60      : c_int = 0x144;
    pub const KEY_F61      : c_int = 0x145;
    pub const KEY_F62      : c_int = 0x146;
    pub const KEY_F63      : c_int = 0x147;

    pub const KEY_DL       : c_int = 0x148;  /* delete line */
    pub const KEY_IL       : c_int = 0x149;  /* insert line */
    pub const KEY_DC       : c_int = 0x14a;  /* delete character */
    pub const KEY_IC       : c_int = 0x14b;  /* insert char or enter ins mode */
    pub const KEY_EIC      : c_int = 0x14c;  /* exit insert char mode */
    pub const KEY_CLEAR    : c_int = 0x14d;  /* clear screen */
    pub const KEY_EOS      : c_int = 0x14e;  /* clear to end of screen */
    pub const KEY_EOL      : c_int = 0x14f;  /* clear to end of line */
    pub const KEY_SF       : c_int = 0x150;  /* scroll 1 line forward */
    pub const KEY_SR       : c_int = 0x151;  /* scroll 1 line back (reverse) */
    pub const KEY_NPAGE    : c_int = 0x152;  /* next page */
    pub const KEY_PPAGE    : c_int = 0x153;  /* previous page */
    pub const KEY_STAB     : c_int = 0x154;  /* set tab */
    pub const KEY_CTAB     : c_int = 0x155;  /* clear tab */
    pub const KEY_CATAB    : c_int = 0x156;  /* clear all tabs */
    pub const KEY_ENTER    : c_int = 0x157;  /* enter or send (unreliable) */
    pub const KEY_SRESET   : c_int = 0x158;  /* soft/reset (partial/unreliable) */
    pub const KEY_RESET    : c_int = 0x159;  /* reset/hard reset (unreliable) */
    pub const KEY_PRINT    : c_int = 0x15a;  /* print/copy */
    pub const KEY_LL       : c_int = 0x15b;  /* home down/bottom (lower left) */
    pub const KEY_ABORT    : c_int = 0x15c;  /* abort/terminate key (any) */
    pub const KEY_SHELP    : c_int = 0x15d;  /* short help */
    pub const KEY_LHELP    : c_int = 0x15e;  /* long help */
    pub const KEY_BTAB     : c_int = 0x15f;  /* Back tab key */
    pub const KEY_BEG      : c_int = 0x160;  /* beg(inning) key */
    pub const KEY_CANCEL   : c_int = 0x161;  /* cancel key */
    pub const KEY_CLOSE    : c_int = 0x162;  /* close key */
    pub const KEY_COMMAND  : c_int = 0x163;  /* cmd (command) key */
    pub const KEY_COPY     : c_int = 0x164;  /* copy key */
    pub const KEY_CREATE   : c_int = 0x165;  /* create key */
    pub const KEY_END      : c_int = 0x166;  /* end key */
    pub const KEY_EXIT     : c_int = 0x167;  /* exit key */
    pub const KEY_FIND     : c_int = 0x168;  /* find key */
    pub const KEY_HELP     : c_int = 0x169;  /* help key */
    pub const KEY_MARK     : c_int = 0x16a;  /* mark key */
    pub const KEY_MESSAGE  : c_int = 0x16b;  /* message key */
    pub const KEY_MOVE     : c_int = 0x16c;  /* move key */
    pub const KEY_NEXT     : c_int = 0x16d;  /* next object key */
    pub const KEY_OPEN     : c_int = 0x16e;  /* open key */
    pub const KEY_OPTIONS  : c_int = 0x16f;  /* options key */
    pub const KEY_PREVIOUS : c_int = 0x170;  /* previous object key */
    pub const KEY_REDO     : c_int = 0x171;  /* redo key */
    pub const KEY_REFERENCE: c_int = 0x172;  /* ref(erence) key */
    pub const KEY_REFRESH  : c_int = 0x173;  /* refresh key */
    pub const KEY_REPLACE  : c_int = 0x174;  /* replace key */
    pub const KEY_RESTART  : c_int = 0x175;  /* restart key */
    pub const KEY_RESUME   : c_int = 0x176;  /* resume key */
    pub const KEY_SAVE     : c_int = 0x177;  /* save key */
    pub const KEY_SBEG     : c_int = 0x178;  /* shifted beginning key */
    pub const KEY_SCANCEL  : c_int = 0x179;  /* shifted cancel key */
    pub const KEY_SCOMMAND : c_int = 0x17a;  /* shifted command key */
    pub const KEY_SCOPY    : c_int = 0x17b;  /* shifted copy key */
    pub const KEY_SCREATE  : c_int = 0x17c;  /* shifted create key */
    pub const KEY_SDC      : c_int = 0x17d;  /* shifted delete char key */
    pub const KEY_SDL      : c_int = 0x17e;  /* shifted delete line key */
    pub const KEY_SELECT   : c_int = 0x17f;  /* select key */
    pub const KEY_SEND     : c_int = 0x180;  /* shifted end key */
    pub const KEY_SEOL     : c_int = 0x181;  /* shifted clear line key */
    pub const KEY_SEXIT    : c_int = 0x182;  /* shifted exit key */
    pub const KEY_SFIND    : c_int = 0x183;  /* shifted find key */
    pub const KEY_SHOME    : c_int = 0x184;  /* shifted home key */
    pub const KEY_SIC      : c_int = 0x185;  /* shifted input key */

    pub const KEY_SLEFT    : c_int = 0x187;  /* shifted left arrow key */
    pub const KEY_SMESSAGE : c_int = 0x188;  /* shifted message key */
    pub const KEY_SMOVE    : c_int = 0x189;  /* shifted move key */
    pub const KEY_SNEXT    : c_int = 0x18a;  /* shifted next key */
    pub const KEY_SOPTIONS : c_int = 0x18b;  /* shifted options key */
    pub const KEY_SPREVIOUS: c_int = 0x18c;  /* shifted prev key */
    pub const KEY_SPRINT   : c_int = 0x18d;  /* shifted print key */
    pub const KEY_SREDO    : c_int = 0x18e;  /* shifted redo key */
    pub const KEY_SREPLACE : c_int = 0x18f;  /* shifted replace key */
    pub const KEY_SRIGHT   : c_int = 0x190;  /* shifted right arrow */
    pub const KEY_SRSUME   : c_int = 0x191;  /* shifted resume key */
    pub const KEY_SSAVE    : c_int = 0x192;  /* shifted save key */
    pub const KEY_SSUSPEND : c_int = 0x193;  /* shifted suspend key */
    pub const KEY_SUNDO    : c_int = 0x194;  /* shifted undo key */
    pub const KEY_SUSPEND  : c_int = 0x195;  /* suspend key */
    pub const KEY_UNDO     : c_int = 0x196;  /* undo key */

    /* PDCurses-specific key definitions -- PC only */

    pub const ALT_0        : c_int = 0x197;
    pub const ALT_1        : c_int = 0x198;
    pub const ALT_2        : c_int = 0x199;
    pub const ALT_3        : c_int = 0x19a;
    pub const ALT_4        : c_int = 0x19b;
    pub const ALT_5        : c_int = 0x19c;
    pub const ALT_6        : c_int = 0x19d;
    pub const ALT_7        : c_int = 0x19e;
    pub const ALT_8        : c_int = 0x19f;
    pub const ALT_9        : c_int = 0x1a0;
    pub const ALT_A        : c_int = 0x1a1;
    pub const ALT_B        : c_int = 0x1a2;
    pub const ALT_C        : c_int = 0x1a3;
    pub const ALT_D        : c_int = 0x1a4;
    pub const ALT_E        : c_int = 0x1a5;
    pub const ALT_F        : c_int = 0x1a6;
    pub const ALT_G        : c_int = 0x1a7;
    pub const ALT_H        : c_int = 0x1a8;
    pub const ALT_I        : c_int = 0x1a9;
    pub const ALT_J        : c_int = 0x1aa;
    pub const ALT_K        : c_int = 0x1ab;
    pub const ALT_L        : c_int = 0x1ac;
    pub const ALT_M        : c_int = 0x1ad;
    pub const ALT_N        : c_int = 0x1ae;
    pub const ALT_O        : c_int = 0x1af;
    pub const ALT_P        : c_int = 0x1b0;
    pub const ALT_Q        : c_int = 0x1b1;
    pub const ALT_R        : c_int = 0x1b2;
    pub const ALT_S        : c_int = 0x1b3;
    pub const ALT_T        : c_int = 0x1b4;
    pub const ALT_U        : c_int = 0x1b5;
    pub const ALT_V        : c_int = 0x1b6;
    pub const ALT_W        : c_int = 0x1b7;
    pub const ALT_X        : c_int = 0x1b8;
    pub const ALT_Y        : c_int = 0x1b9;
    pub const ALT_Z        : c_int = 0x1ba;

    pub const CTL_LEFT     : c_int = 0x1bb;  /* Control-Left-Arrow */
    pub const CTL_RIGHT    : c_int = 0x1bc;
    pub const CTL_PGUP     : c_int = 0x1bd;
    pub const CTL_PGDN     : c_int = 0x1be;
    pub const CTL_HOME     : c_int = 0x1bf;
    pub const CTL_END      : c_int = 0x1c0;

    pub const KEY_A1       : c_int = 0x1c1;  /* upper left on Virtual keypad */
    pub const KEY_A2       : c_int = 0x1c2;  /* upper middle on Virt. keypad */
    pub const KEY_A3       : c_int = 0x1c3;  /* upper right on Vir. keypad */
    pub const KEY_B1       : c_int = 0x1c4;  /* middle left on Virt. keypad */
    pub const KEY_B2       : c_int = 0x1c5;  /* center on Virt. keypad */
    pub const KEY_B3       : c_int = 0x1c6;  /* middle right on Vir. keypad */
    pub const KEY_C1       : c_int = 0x1c7;  /* lower left on Virt. keypad */
    pub const KEY_C2       : c_int = 0x1c8;  /* lower middle on Virt. keypad */
    pub const KEY_C3       : c_int = 0x1c9;  /* lower right on Vir. keypad */

    pub const PADSLASH     : c_int = 0x1ca;  /* slash on keypad */
    pub const PADENTER     : c_int = 0x1cb;  /* enter on keypad */
    pub const CTL_PADENTER : c_int = 0x1cc;  /* ctl-enter on keypad */
    pub const ALT_PADENTER : c_int = 0x1cd;  /* alt-enter on keypad */
    pub const PADSTOP      : c_int = 0x1ce;  /* stop on keypad */
    pub const PADSTAR      : c_int = 0x1cf;  /* star on keypad */
    pub const PADMINUS     : c_int = 0x1d0;  /* minus on keypad */
    pub const PADPLUS      : c_int = 0x1d1;  /* plus on keypad */
    pub const CTL_PADSTOP  : c_int = 0x1d2;  /* ctl-stop on keypad */
    pub const CTL_PADCENTER: c_int = 0x1d3;  /* ctl-enter on keypad */
    pub const CTL_PADPLUS  : c_int = 0x1d4;  /* ctl-plus on keypad */
    pub const CTL_PADMINUS : c_int = 0x1d5;  /* ctl-minus on keypad */
    pub const CTL_PADSLASH : c_int = 0x1d6;  /* ctl-slash on keypad */
    pub const CTL_PADSTAR  : c_int = 0x1d7;  /* ctl-star on keypad */
    pub const ALT_PADPLUS  : c_int = 0x1d8;  /* alt-plus on keypad */
    pub const ALT_PADMINUS : c_int = 0x1d9;  /* alt-minus on keypad */
    pub const ALT_PADSLASH : c_int = 0x1da;  /* alt-slash on keypad */
    pub const ALT_PADSTAR  : c_int = 0x1db;  /* alt-star on keypad */
    pub const ALT_PADSTOP  : c_int = 0x1dc;  /* alt-stop on keypad */
    pub const CTL_INS      : c_int = 0x1dd;  /* ctl-insert */
    pub const ALT_DEL      : c_int = 0x1de;  /* alt-delete */
    pub const ALT_INS      : c_int = 0x1df;  /* alt-insert */
    pub const CTL_UP       : c_int = 0x1e0;  /* ctl-up arrow */
    pub const CTL_DOWN     : c_int = 0x1e1;  /* ctl-down arrow */
    pub const CTL_TAB      : c_int = 0x1e2;  /* ctl-tab */
    pub const ALT_TAB      : c_int = 0x1e3;
    pub const ALT_MINUS    : c_int = 0x1e4;
    pub const ALT_EQUAL    : c_int = 0x1e5;
    pub const ALT_HOME     : c_int = 0x1e6;
    pub const ALT_PGUP     : c_int = 0x1e7;
    pub const ALT_PGDN     : c_int = 0x1e8;
    pub const ALT_END      : c_int = 0x1e9;
    pub const ALT_UP       : c_int = 0x1ea;  /* alt-up arrow */
    pub const ALT_DOWN     : c_int = 0x1eb;  /* alt-down arrow */
    pub const ALT_RIGHT    : c_int = 0x1ec;  /* alt-right arrow */
    pub const ALT_LEFT     : c_int = 0x1ed;  /* alt-left arrow */
    pub const ALT_ENTER    : c_int = 0x1ee;  /* alt-enter */
    pub const ALT_ESC      : c_int = 0x1ef;  /* alt-escape */
    pub const ALT_BQUOTE   : c_int = 0x1f0;  /* alt-back quote */
    pub const ALT_LBRACKET : c_int = 0x1f1;  /* alt-left bracket */
    pub const ALT_RBRACKET : c_int = 0x1f2;  /* alt-right bracket */
    pub const ALT_SEMICOLON: c_int = 0x1f3;  /* alt-semi-colon */
    pub const ALT_FQUOTE   : c_int = 0x1f4;  /* alt-forward quote */
    pub const ALT_COMMA    : c_int = 0x1f5;  /* alt-comma */
    pub const ALT_STOP     : c_int = 0x1f6;  /* alt-stop */
    pub const ALT_FSLASH   : c_int = 0x1f7;  /* alt-forward slash */
    pub const ALT_BKSP     : c_int = 0x1f8;  /* alt-backspace */
    pub const CTL_BKSP     : c_int = 0x1f9;  /* ctl-backspace */
    pub const PAD0         : c_int = 0x1fa;  /* keypad 0 */

    pub const CTL_PAD0     : c_int = 0x1fb;  /* ctl-keypad 0 */
    pub const CTL_PAD1     : c_int = 0x1fc;
    pub const CTL_PAD2     : c_int = 0x1fd;
    pub const CTL_PAD3     : c_int = 0x1fe;
    pub const CTL_PAD4     : c_int = 0x1ff;
    pub const CTL_PAD5     : c_int = 0x200;
    pub const CTL_PAD6     : c_int = 0x201;
    pub const CTL_PAD7     : c_int = 0x202;
    pub const CTL_PAD8     : c_int = 0x203;
    pub const CTL_PAD9     : c_int = 0x204;

    pub const ALT_PAD0     : c_int = 0x205;  /* alt-keypad 0 */
    pub const ALT_PAD1     : c_int = 0x206;
    pub const ALT_PAD2     : c_int = 0x207;
    pub const ALT_PAD3     : c_int = 0x208;
    pub const ALT_PAD4     : c_int = 0x209;
    pub const ALT_PAD5     : c_int = 0x20a;
    pub const ALT_PAD6     : c_int = 0x20b;
    pub const ALT_PAD7     : c_int = 0x20c;
    pub const ALT_PAD8     : c_int = 0x20d;
    pub const ALT_PAD9     : c_int = 0x20e;

    pub const CTL_DEL      : c_int = 0x20f;  /* clt-delete */
    pub const ALT_BSLASH   : c_int = 0x210;  /* alt-back slash */
    pub const CTL_ENTER    : c_int = 0x211;  /* ctl-enter */

    pub const SHF_PADENTER : c_int = 0x212;  /* shift-enter on keypad */
    pub const SHF_PADSLASH : c_int = 0x213;  /* shift-slash on keypad */
    pub const SHF_PADSTAR  : c_int = 0x214;  /* shift-star  on keypad */
    pub const SHF_PADPLUS  : c_int = 0x215;  /* shift-plus  on keypad */
    pub const SHF_PADMINUS : c_int = 0x216;  /* shift-minus on keypad */
    pub const SHF_UP       : c_int = 0x217;  /* shift-up on keypad */
    pub const SHF_DOWN     : c_int = 0x218;  /* shift-down on keypad */
    pub const SHF_IC       : c_int = 0x219;  /* shift-insert on keypad */
    pub const SHF_DC       : c_int = 0x21a;  /* shift-delete on keypad */

    pub const KEY_MOUSE    : c_int = 0x21b;  /* "mouse" key */
    pub const KEY_SHIFT_L  : c_int = 0x21c;  /* Left-shift */
    pub const KEY_SHIFT_R  : c_int = 0x21d;  /* Right-shift */
    pub const KEY_CONTROL_L: c_int = 0x21e;  /* Left-control */
    pub const KEY_CONTROL_R: c_int = 0x21f;  /* Right-control */
    pub const KEY_ALT_L    : c_int = 0x220;  /* Left-alt */
    pub const KEY_ALT_R    : c_int = 0x221;  /* Right-alt */
    pub const KEY_RESIZE   : c_int = 0x222;  /* Window resize */
    pub const KEY_SUP      : c_int = 0x223;  /* Shifted up arrow */
    pub const KEY_SDOWN    : c_int = 0x224;  /* Shifted down arrow */

    pub const KEY_MIN      : c_int = KEY_BREAK;  /* Minimum curses key value */
    pub const KEY_MAX      : c_int = KEY_SDOWN;  /* Maximum curses key */
}

pub const fn KEY_F(n: c_int) -> c_int { KEY_F0 + n }

/*----------------------------------------------------------------------
 *
 *  Functions
 *
 */

/* Standard */

extern "C" {
    pub fn addch           (ch: chtype) -> c_int;
    pub fn addchnstr       (ch: *const chtype, n: c_int) -> c_int;
    pub fn addchstr        (ch: *const chtype) -> c_int;
    pub fn addnstr         (str: *const c_char, n: c_int) -> c_int;
    pub fn addstr          (str: *const c_char) -> c_int;
    pub fn attroff         (attrs: chtype) -> c_int;
    pub fn attron          (attrs: chtype) -> c_int;
    pub fn attrset         (attrs: chtype) -> c_int;
    pub fn attr_get        (attrs: *mut attr_t, color_pair: *mut c_short, opts: *mut c_void) -> c_int;
    pub fn attr_off        (attrs: attr_t, opts: *mut c_void) -> c_int;
    pub fn attr_on         (attrs: attr_t, opts: *mut c_void) -> c_int;
    pub fn attr_set        (attrs: attr_t, color_pair: c_short, opts: *mut c_void) -> c_int;
    pub fn baudrate        () -> c_int;
    pub fn beep            () -> c_int;
    pub fn bkgd            (ch: chtype) -> c_int;
    pub fn bkgdset         (ch: chtype);
    pub fn border          (ls: chtype, rs: chtype, ts: chtype, bs: chtype, tl: chtype, tr: chtype, bl: chtype, br: chtype) -> c_int;
    #[link_name = "box"]
    pub fn box_            (win: *mut WINDOW, verch: chtype, horch: chtype) -> c_int;
    pub fn can_change_color() -> bool;
    pub fn cbreak          () -> c_int;
    pub fn chgat           (n: c_int, attrs: attr_t, color_pair: c_short, opts: *const c_void) -> c_int;
    pub fn clearok         (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn clear           () -> c_int;
    pub fn clrtobot        () -> c_int;
    pub fn clrtoeol        () -> c_int;
    pub fn color_content   (color: c_short, red: *mut c_short, green: *mut c_short, blue: *mut c_short) -> c_int;
    pub fn color_set       (color_pair: c_short, opts: *mut c_void) -> c_int;
    pub fn copywin         (src: *const WINDOW, dst: *mut WINDOW, src_tr: c_int, src_tc: c_int, dst_tr: c_int, dst_tc: c_int, dst_br: c_int, dst_bc: c_int, _overlay: c_int) -> c_int;
    pub fn curs_set        (visibility: c_int) -> c_int;
    pub fn def_prog_mode   () -> c_int;
    pub fn def_shell_mode  () -> c_int;
    pub fn delay_output    (ms: c_int) -> c_int;
    pub fn delch           () -> c_int;
    pub fn deleteln        () -> c_int;
    pub fn delscreen       (scr: *mut SCREEN);
    pub fn delwin          (win: *mut WINDOW) -> c_int;
    pub fn derwin          (win: *mut WINDOW, nlines: c_int, ncols: c_int, begy: c_int, begx: c_int) -> *mut WINDOW;
    pub fn doupdate        () -> c_int;
    pub fn dupwin          (win: *mut WINDOW) -> *mut WINDOW;
    pub fn echochar        (ch: chtype) -> c_int;
    pub fn echo            () -> c_int;
    pub fn endwin          () -> c_int;
    pub fn erasechar       () -> c_char;
    pub fn erase           () -> c_int;
    pub fn filter          ();
    pub fn flash           () -> c_int;
    pub fn flushinp        () -> c_int;
    pub fn getbkgd         (win: *mut WINDOW) -> chtype;
    pub fn getnstr         (str: *mut c_char, n: c_int) -> c_int;
    pub fn getstr          (str: *mut c_char) -> c_int;
    pub fn getwin          (filep: *mut FILE) -> *mut WINDOW;
    pub fn halfdelay       (tenths: c_int) -> c_int;
    pub fn has_colors      () -> bool;
    pub fn has_ic          () -> bool;
    pub fn has_il          () -> bool;
    pub fn hline           (ch: chtype, n: c_int) -> c_int;
    pub fn idcok           (win: *mut WINDOW, flag: bool);
    pub fn idlok           (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn immedok         (win: *mut WINDOW, flag: bool);
    pub fn inchnstr        (ch: *mut chtype, n: c_int) -> c_int;
    pub fn inchstr         (ch: *mut chtype) -> c_int;
    pub fn inch            () -> chtype;
    pub fn init_color      (color: c_short, red: c_short, green: c_short, blue: c_short) -> c_int;
    pub fn init_pair       (color_pair: c_short, fg: c_short, bg: c_short) -> c_int;
    pub fn initscr         () -> *mut WINDOW;
    pub fn innstr          (str: *mut c_char, n: c_int) -> c_int;
    pub fn insch           (ch: chtype) -> c_int;
    pub fn insdelln        (n: c_int) -> c_int;
    pub fn insertln        () -> c_int;
    pub fn insnstr         (str: *const c_char, n: c_int) -> c_int;
    pub fn insstr          (str: *const c_char) -> c_int;
    pub fn instr           (str: *mut c_char) -> c_int;
    pub fn intrflush       (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn isendwin        () -> bool;
    pub fn is_linetouched  (win: *mut WINDOW, line: c_int) -> bool;
    pub fn is_wintouched   (win: *mut WINDOW) -> bool;
    pub fn keyname         (c: c_int) -> *mut c_char;
    pub fn keypad          (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn killchar        () -> c_char;
    pub fn leaveok         (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn longname        () -> *mut c_char;
    pub fn meta            (win: *mut WINDOW, flag: bool) -> c_int;
    #[link_name = "move"]
    pub fn move_           (y: c_int, x: c_int) -> c_int;
    pub fn mvaddch         (y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn mvaddchnstr     (y: c_int, x: c_int, ch: *const chtype, n: c_int) -> c_int;
    pub fn mvaddchstr      (y: c_int, x: c_int, ch: *const chtype) -> c_int;
    pub fn mvaddnstr       (y: c_int, x: c_int, str: *const c_char, n: c_int) -> c_int;
    pub fn mvaddstr        (y: c_int, x: c_int, str: *const c_char) -> c_int;
    pub fn mvchgat         (y: c_int, x: c_int, n: c_int, attrs: attr_t, color_pair: c_short, opts: *const c_void) -> c_int;
    pub fn mvcur           (oldrow: c_int, oldcol: c_int, newrow: c_int, newcol: c_int) -> c_int;
    pub fn mvdelch         (y: c_int, x: c_int) -> c_int;
    pub fn mvderwin        (win: *mut WINDOW, pary: c_int, parx: c_int) -> c_int;
    pub fn mvgetch         (y: c_int, x: c_int) -> c_int;
    pub fn mvgetnstr       (y: c_int, x: c_int, str: *mut c_char, n: c_int) -> c_int;
    pub fn mvgetstr        (y: c_int, x: c_int, str: *mut c_char) -> c_int;
    pub fn mvhline         (y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;
    pub fn mvinch          (y: c_int, x: c_int) -> chtype;
    pub fn mvinchnstr      (y: c_int, x: c_int, ch: *mut chtype, n: c_int) -> c_int;
    pub fn mvinchstr       (y: c_int, x: c_int, ch: *mut chtype) -> c_int;
    pub fn mvinnstr        (y: c_int, x: c_int, str: *mut c_char, n: c_int) -> c_int;
    pub fn mvinsch         (y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn mvinsnstr       (y: c_int, x: c_int, str: *const c_char, n: c_int) -> c_int;
    pub fn mvinsstr        (y: c_int, x: c_int, str: *const c_char) -> c_int;
    pub fn mvinstr         (y: c_int, x: c_int, str: *mut c_char) -> c_int;
    pub fn mvprintw        (y: c_int, x: c_int, fmt: *const c_char, ...) -> c_int;
    pub fn mvscanw         (y: c_int, x: c_int, fmt: *const c_char, ...) -> c_int;
    pub fn mvvline         (y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;
    pub fn mvwaddchnstr    (win: *mut WINDOW, y: c_int, x: c_int, ch: *const chtype, n: c_int) -> c_int;
    pub fn mvwaddchstr     (win: *mut WINDOW, y: c_int, x: c_int, ch: *const chtype) -> c_int;
    pub fn mvwaddch        (win: *mut WINDOW, y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn mvwaddnstr      (win: *mut WINDOW, y: c_int, x: c_int, str: *const c_char, n: c_int) -> c_int;
    pub fn mvwaddstr       (win: *mut WINDOW, y: c_int, x: c_int, str: *const c_char) -> c_int;
    pub fn mvwchgat        (win: *mut WINDOW, y: c_int, x: c_int, n: c_int, attrs: attr_t, color_pair: c_short, opts: *const c_void) -> c_int;
    pub fn mvwdelch        (win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    pub fn mvwgetch        (win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    pub fn mvwgetnstr      (win: *mut WINDOW, y: c_int, x: c_int, str: *mut c_char, n: c_int) -> c_int;
    pub fn mvwgetstr       (win: *mut WINDOW, y: c_int, x: c_int, str: *mut c_char) -> c_int;
    pub fn mvwhline        (win: *mut WINDOW, y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;
    pub fn mvwinchnstr     (win: *mut WINDOW, y: c_int, x: c_int, ch: *mut chtype, n: c_int) -> c_int;
    pub fn mvwinchstr      (win: *mut WINDOW, y: c_int, x: c_int, ch: *mut chtype) -> c_int;
    pub fn mvwinch         (win: *mut WINDOW, y: c_int, x: c_int) -> chtype;
    pub fn mvwinnstr       (win: *mut WINDOW, y: c_int, x: c_int, str: *mut c_char, n: c_int) -> c_int;
    pub fn mvwinsch        (win: *mut WINDOW, y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn mvwinsnstr      (win: *mut WINDOW, y: c_int, x: c_int, str: *const c_char, n: c_int) -> c_int;
    pub fn mvwinsstr       (win: *mut WINDOW, y: c_int, x: c_int, str: *const c_char) -> c_int;
    pub fn mvwinstr        (win: *mut WINDOW, y: c_int, x: c_int, str: *mut c_char) -> c_int;
    pub fn mvwin           (win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    pub fn mvwprintw       (win: *mut WINDOW, y: c_int, x: c_int, fmt: *const c_char, ...) -> c_int;
    pub fn mvwscanw        (win: *mut WINDOW, y: c_int, x: c_int, fmt: *const c_char, ...) -> c_int;
    pub fn mvwvline        (win: *mut WINDOW, y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;
    pub fn napms           (ms: c_int) -> c_int;
    pub fn newpad          (nlines: c_int, ncols: c_int) -> *mut WINDOW;
    pub fn newterm         (termtype: *const c_char, outfd: *mut FILE, infd: *mut FILE) -> *mut SCREEN;
    pub fn newwin          (nlines: c_int, ncols: c_int, begy: c_int, begx: c_int) -> *mut WINDOW;
    pub fn nl              () -> c_int;
    pub fn nocbreak        () -> c_int;
    pub fn nodelay         (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn noecho          () -> c_int;
    pub fn nonl            () -> c_int;
    pub fn noqiflush       ();
    pub fn noraw           () -> c_int;
    pub fn notimeout       (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn overlay         (src: *const WINDOW, dst: *mut WINDOW) -> c_int;
    pub fn overwrite       (src: *const WINDOW, dst: *mut WINDOW) -> c_int;
    pub fn pair_content    (color_pair: c_short, fg: *mut c_short, bg: *mut c_short) -> c_int;
    pub fn pechochar       (win: *mut WINDOW, ch: chtype) -> c_int;
    pub fn pnoutrefresh    (win: *mut WINDOW, py: c_int, px: c_int, sy1: c_int, sx1: c_int, sy2: c_int, sx2: c_int) -> c_int;
    pub fn prefresh        (win: *mut WINDOW, py: c_int, px: c_int, sy1: c_int, sx1: c_int, sy2: c_int, sx2: c_int) -> c_int;
    pub fn printw          (fmt: *const c_char, ...) -> c_int;
    pub fn putwin          (win: *mut WINDOW, filep: *mut FILE) -> c_int;
    pub fn qiflush         ();
    pub fn raw             () -> c_int;
    pub fn redrawwin       (win: *mut WINDOW) -> c_int;
    pub fn refresh         () -> c_int;
    pub fn reset_prog_mode () -> c_int;
    pub fn reset_shell_mode() -> c_int;
    pub fn resetty         () -> c_int;
    pub fn ripoffline      (line: c_int, init: ::core::option::Option<unsafe extern "C" fn(win: *mut WINDOW, ncols: c_int) -> c_int>) -> c_int;
    pub fn savetty         () -> c_int;
    pub fn scanw           (fmt: *const c_char, ...) -> c_int;
    pub fn scr_dump        (filename: *const c_char) -> c_int;
    pub fn scr_init        (filename: *const c_char) -> c_int;
    pub fn scr_restore     (filename: *const c_char) -> c_int;
    pub fn scr_set         (filename: *const c_char) -> c_int;
    pub fn scrl            (n: c_int) -> c_int;
    pub fn scroll          (win: *mut WINDOW) -> c_int;
    pub fn scrollok        (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn set_term        (new: *mut SCREEN) -> *mut SCREEN;
    pub fn setscrreg       (top: c_int, bot: c_int) -> c_int;
    pub fn slk_attroff     (attrs: chtype) -> c_int;
    pub fn slk_attr_off    (attrs: attr_t, opts: *mut c_void) -> c_int;
    pub fn slk_attron      (attrs: chtype) -> c_int;
    pub fn slk_attr_on     (attrs: attr_t, opts: *mut c_void) -> c_int;
    pub fn slk_attrset     (attrs: chtype) -> c_int;
    pub fn slk_attr_set    (attrs: attr_t, color_pair: c_short, opts: *mut c_void) -> c_int;
    pub fn slk_clear       () -> c_int;
    pub fn slk_color       (color_pair: c_short) -> c_int;
    pub fn slk_init        (fmt: c_int) -> c_int;
    pub fn slk_label       (labnum: c_int) -> *mut c_char;
    pub fn slk_noutrefresh () -> c_int;
    pub fn slk_refresh     () -> c_int;
    pub fn slk_restore     () -> c_int;
    pub fn slk_set         (labnum: c_int, label: *const c_char, justify: c_int) -> c_int;
    pub fn slk_touch       () -> c_int;
    pub fn standend        () -> c_int;
    pub fn standout        () -> c_int;
    pub fn start_color     () -> c_int;
    pub fn subpad          (orig: *mut WINDOW, nlines: c_int, ncols: c_int, begy: c_int, begx: c_int) -> *mut WINDOW;
    pub fn subwin          (orig: *mut WINDOW, nlines: c_int, ncols: c_int, begy: c_int, begx: c_int) -> *mut WINDOW;
    pub fn syncok          (win: *mut WINDOW, flag: bool) -> c_int;
    pub fn termattrs       () -> chtype;
    pub fn term_attrs      () -> attr_t;
    pub fn termname        () -> *mut c_char;
    pub fn timeout         (delay: c_int);
    pub fn touchline       (win: *mut WINDOW, start: c_int, count: c_int) -> c_int;
    pub fn touchwin        (win: *mut WINDOW) -> c_int;
    pub fn typeahead       (fd: c_int) -> c_int;
    pub fn untouchwin      (win: *mut WINDOW) -> c_int;
    pub fn use_env         (flag: bool);
    pub fn vidattr         (attrs: chtype) -> c_int;
    pub fn vid_attr        (attrs: attr_t, color_pair: c_short, opts: *mut c_void) -> c_int;
    pub fn vidputs         (attrs: chtype, putc: ::core::option::Option<unsafe extern "C" fn(ch: c_int) -> c_int>) -> c_int;
    pub fn vid_puts        (attrs: attr_t, color_pair: c_short, opts: *mut c_void, putc: ::core::option::Option<unsafe extern "C" fn(ch: c_int) -> c_int>) -> c_int;
    pub fn vline           (ch: chtype, n: c_int) -> c_int;
    pub fn vw_printw       (win: *mut WINDOW, fmt: *const c_char, varglist: va_list) -> c_int;
    pub fn vwprintw        (win: *mut WINDOW, fmt: *const c_char, varglist: va_list) -> c_int;
    pub fn vw_scanw        (win: *mut WINDOW, fmt: *const c_char, varglist: va_list) -> c_int;
    pub fn vwscanw         (win: *mut WINDOW, fmt: *const c_char, varglist: va_list) -> c_int;
    pub fn waddchnstr      (win: *mut WINDOW, ch: *const chtype, n: c_int) -> c_int;
    pub fn waddchstr       (win: *mut WINDOW, ch: *const chtype) -> c_int;
    pub fn waddch          (win: *mut WINDOW, ch: chtype) -> c_int;
    pub fn waddnstr        (win: *mut WINDOW, str: *const c_char, n: c_int) -> c_int;
    pub fn waddstr         (win: *mut WINDOW, str: *const c_char) -> c_int;
    pub fn wattroff        (win: *mut WINDOW, attrs: chtype) -> c_int;
    pub fn wattron         (win: *mut WINDOW, attrs: chtype) -> c_int;
    pub fn wattrset        (win: *mut WINDOW, attrs: chtype) -> c_int;
    pub fn wattr_get       (win: *mut WINDOW, attrs: *mut attr_t, color_pair: *mut c_short, opts: *mut c_void) -> c_int;
    pub fn wattr_off       (win: *mut WINDOW, attrs: attr_t, opts: *mut c_void) -> c_int;
    pub fn wattr_on        (win: *mut WINDOW, attrs: attr_t, opts: *mut c_void) -> c_int;
    pub fn wattr_set       (win: *mut WINDOW, attrs: attr_t, color_pair: c_short, opts: *mut c_void) -> c_int;
    pub fn wbkgdset        (win: *mut WINDOW, ch: chtype);
    pub fn wbkgd           (win: *mut WINDOW, ch: chtype) -> c_int;
    pub fn wborder         (win: *mut WINDOW, ls: chtype, rs: chtype, ts: chtype, bs: chtype, tl: chtype, tr: chtype, bl: chtype, br: chtype) -> c_int;
    pub fn wchgat          (win: *mut WINDOW, n: c_int, attrs: attr_t, color_pair: c_short, opts: *const c_void) -> c_int;
    pub fn wclear          (win: *mut WINDOW) -> c_int;
    pub fn wclrtobot       (win: *mut WINDOW) -> c_int;
    pub fn wclrtoeol       (win: *mut WINDOW) -> c_int;
    pub fn wcolor_set      (win: *mut WINDOW, color: c_short, opts: *mut c_void) -> c_int;
    pub fn wcursyncup      (win: *mut WINDOW);
    pub fn wdelch          (win: *mut WINDOW) -> c_int;
    pub fn wdeleteln       (win: *mut WINDOW) -> c_int;
    pub fn wechochar       (win: *mut WINDOW, ch: chtype) -> c_int;
    pub fn werase          (win: *mut WINDOW) -> c_int;
    pub fn wgetch          (win: *mut WINDOW) -> c_int;
    pub fn wgetnstr        (win: *mut WINDOW, str: *mut c_char, n: c_int) -> c_int;
    pub fn wgetstr         (win: *mut WINDOW, str: *mut c_char) -> c_int;
    pub fn whline          (win: *mut WINDOW, ch: chtype, n: c_int) -> c_int;
    pub fn winchnstr       (win: *mut WINDOW, ch: *mut chtype, n: c_int) -> c_int;
    pub fn winchstr        (win: *mut WINDOW, ch: *mut chtype) -> c_int;
    pub fn winch           (win: *mut WINDOW) -> chtype;
    pub fn winnstr         (win: *mut WINDOW, str: *mut c_char, n: c_int) -> c_int;
    pub fn winsch          (win: *mut WINDOW, ch: chtype) -> c_int;
    pub fn winsdelln       (win: *mut WINDOW, n: c_int) -> c_int;
    pub fn winsertln       (win: *mut WINDOW) -> c_int;
    pub fn winsnstr        (win: *mut WINDOW, str: *const c_char, n: c_int) -> c_int;
    pub fn winsstr         (win: *mut WINDOW, str: *const c_char) -> c_int;
    pub fn winstr          (win: *mut WINDOW, str: *mut c_char) -> c_int;
    pub fn wmove           (win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    pub fn wnoutrefresh    (win: *mut WINDOW) -> c_int;
    pub fn wprintw         (win: *mut WINDOW, fmt: *const c_char, ...) -> c_int;
    pub fn wredrawln       (win: *mut WINDOW, beg_line: c_int, num_lines: c_int) -> c_int;
    pub fn wrefresh        (win: *mut WINDOW) -> c_int;
    pub fn wscanw          (win: *mut WINDOW, fmt: *const c_char, ...) -> c_int;
    pub fn wscrl           (win: *mut WINDOW, n: c_int) -> c_int;
    pub fn wsetscrreg      (win: *mut WINDOW, top: c_int, bot: c_int) -> c_int;
    pub fn wstandend       (win: *mut WINDOW) -> c_int;
    pub fn wstandout       (win: *mut WINDOW) -> c_int;
    pub fn wsyncdown       (win: *mut WINDOW);
    pub fn wsyncup         (win: *mut WINDOW);
    pub fn wtimeout        (win: *mut WINDOW, delay: c_int);
    pub fn wtouchln        (win: *mut WINDOW, y: c_int, n: c_int, changed: c_int) -> c_int;
    pub fn wvline          (win: *mut WINDOW, ch: chtype, n: c_int) -> c_int;
}

/* Wide-character functions */

extern "C" {
    pub fn addnwstr      (wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn addwstr       (wstr: *const wchar_t) -> c_int;
    pub fn add_wch       (wch: *const cchar_t) -> c_int;
    pub fn add_wchnstr   (wch: *const cchar_t, n: c_int) -> c_int;
    pub fn add_wchstr    (wch: *const cchar_t) -> c_int;
    pub fn bkgrnd        (wch: *const cchar_t) -> c_int;
    pub fn bkgrndset     (wch: *const cchar_t);
    pub fn border_set    (ls: *const cchar_t, rs: *const cchar_t, ts: *const cchar_t, bs: *const cchar_t, tl: *const cchar_t, tr: *const cchar_t, bl: *const cchar_t, br: *const cchar_t) -> c_int;
    pub fn box_set       (win: *mut WINDOW, verch: *const cchar_t, horch: *const cchar_t) -> c_int;
    pub fn echo_wchar    (wch: *const cchar_t) -> c_int;
    pub fn erasewchar    (wch: *mut wchar_t) -> c_int;
    pub fn getbkgrnd     (wch: *mut cchar_t) -> c_int;
    pub fn getcchar      (wcval: *const cchar_t, wch: *mut wchar_t, attrs: *mut attr_t, color_pair: *mut c_short, opts: *mut c_void) -> c_int;
    pub fn getn_wstr     (wstr: *mut wint_t, n: c_int) -> c_int;
    pub fn get_wch       (wch: *mut wint_t) -> c_int;
    pub fn get_wstr      (wstr: *mut wint_t) -> c_int;
    pub fn hline_set     (wch: *const cchar_t, n: c_int) -> c_int;
    pub fn innwstr       (wstr: *mut wchar_t, n: c_int) -> c_int;
    pub fn ins_nwstr     (wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn ins_wch       (wch: *const cchar_t) -> c_int;
    pub fn ins_wstr      (wstr: *const wchar_t) -> c_int;
    pub fn inwstr        (wstr: *mut wchar_t) -> c_int;
    pub fn in_wch        (wch: *mut cchar_t) -> c_int;
    pub fn in_wchnstr    (wch: *mut cchar_t, n: c_int) -> c_int;
    pub fn in_wchstr     (wch: *mut cchar_t) -> c_int;
    pub fn key_name      (w: wchar_t) -> *mut c_char;
    pub fn killwchar     (wch: *mut wchar_t) -> c_int;
    pub fn mvaddnwstr    (y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn mvaddwstr     (y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
    pub fn mvadd_wch     (y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
    pub fn mvadd_wchnstr (y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn mvadd_wchstr  (y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
    pub fn mvgetn_wstr   (y: c_int, x: c_int, wstr: *mut wint_t, n: c_int) -> c_int;
    pub fn mvget_wch     (y: c_int, x: c_int, wch: *mut wint_t) -> c_int;
    pub fn mvget_wstr    (y: c_int, x: c_int, wstr: *mut wint_t) -> c_int;
    pub fn mvhline_set   (y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn mvinnwstr     (y: c_int, x: c_int, wstr: *mut wchar_t, n: c_int) -> c_int;
    pub fn mvins_nwstr   (y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn mvins_wch     (y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
    pub fn mvins_wstr    (y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
    pub fn mvinwstr      (y: c_int, x: c_int, wstr: *mut wchar_t) -> c_int;
    pub fn mvin_wch      (y: c_int, x: c_int, wch: *mut cchar_t) -> c_int;
    pub fn mvin_wchnstr  (y: c_int, x: c_int, wch: *mut cchar_t, n: c_int) -> c_int;
    pub fn mvin_wchstr   (y: c_int, x: c_int, wch: *mut cchar_t) -> c_int;
    pub fn mvvline_set   (y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn mvwaddnwstr   (win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn mvwaddwstr    (win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
    pub fn mvwadd_wch    (win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
    pub fn mvwadd_wchnstr(win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn mvwadd_wchstr (win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
    pub fn mvwgetn_wstr  (win: *mut WINDOW, y: c_int, x: c_int, wstr: *mut wint_t, n: c_int) -> c_int;
    pub fn mvwget_wch    (win: *mut WINDOW, y: c_int, x: c_int, wch: *mut wint_t) -> c_int;
    pub fn mvwget_wstr   (win: *mut WINDOW, y: c_int, x: c_int, wstr: *mut wint_t) -> c_int;
    pub fn mvwhline_set  (win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn mvwinnwstr    (win: *mut WINDOW, y: c_int, x: c_int, wstr: *mut wchar_t, n: c_int) -> c_int;
    pub fn mvwins_nwstr  (win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn mvwins_wch    (win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
    pub fn mvwins_wstr   (win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
    pub fn mvwin_wch     (win: *mut WINDOW, y: c_int, x: c_int, wch: *mut cchar_t) -> c_int;
    pub fn mvwin_wchnstr (win: *mut WINDOW, y: c_int, x: c_int, wch: *mut cchar_t, n: c_int) -> c_int;
    pub fn mvwin_wchstr  (win: *mut WINDOW, y: c_int, x: c_int, wch: *mut cchar_t) -> c_int;
    pub fn mvwinwstr     (win: *mut WINDOW, y: c_int, x: c_int, wstr: *mut wchar_t) -> c_int;
    pub fn mvwvline_set  (win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn pecho_wchar   (win: *mut WINDOW, wch: *const cchar_t) -> c_int;
    pub fn setcchar      (wch: *mut cchar_t, wch: *const wchar_t, attrs: attr_t, color_pair: c_short, opts: *const c_void) -> c_int;
    pub fn slk_wset      (labnum: c_int, label: *const wchar_t, justify: c_int) -> c_int;
    pub fn unget_wch     (wch: wchar_t) -> c_int;
    pub fn vline_set     (wch: *const cchar_t, n: c_int) -> c_int;
    pub fn waddnwstr     (win: *mut WINDOW, wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn waddwstr      (win: *mut WINDOW, wstr: *const wchar_t) -> c_int;
    pub fn wadd_wch      (win: *mut WINDOW, wch: *const cchar_t) -> c_int;
    pub fn wadd_wchnstr  (win: *mut WINDOW, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn wadd_wchstr   (win: *mut WINDOW, wch: *const cchar_t) -> c_int;
    pub fn wbkgrnd       (win: *mut WINDOW, wch: *const cchar_t) -> c_int;
    pub fn wbkgrndset    (win: *mut WINDOW, wch: *const cchar_t);
    pub fn wborder_set   (win: *mut WINDOW, ls: *const cchar_t, rs: *const cchar_t, ts: *const cchar_t, bs: *const cchar_t, tl: *const cchar_t, tr: *const cchar_t, bl: *const cchar_t, br: *const cchar_t) -> c_int;
    pub fn wecho_wchar   (win: *mut WINDOW, wch: *const cchar_t) -> c_int;
    pub fn wgetbkgrnd    (win: *mut WINDOW, wch: *mut cchar_t) -> c_int;
    pub fn wgetn_wstr    (win: *mut WINDOW, wstr: *mut wint_t, n: c_int) -> c_int;
    pub fn wget_wch      (win: *mut WINDOW, wch: *mut wint_t) -> c_int;
    pub fn wget_wstr     (win: *mut WINDOW, wstr: *mut wint_t) -> c_int;
    pub fn whline_set    (win: *mut WINDOW, wch: *const cchar_t, n: c_int) -> c_int;
    pub fn winnwstr      (win: *mut WINDOW, wstr: *mut wchar_t, n: c_int) -> c_int;
    pub fn wins_nwstr    (win: *mut WINDOW, wstr: *const wchar_t, n: c_int) -> c_int;
    pub fn wins_wch      (win: *mut WINDOW, wch: *const cchar_t) -> c_int;
    pub fn wins_wstr     (win: *mut WINDOW, wstr: *const wchar_t) -> c_int;
    pub fn winwstr       (win: *mut WINDOW, wstr: *mut wchar_t) -> c_int;
    pub fn win_wch       (win: *mut WINDOW, wch: *mut cchar_t) -> c_int;
    pub fn win_wchnstr   (win: *mut WINDOW, wch: *mut cchar_t, n: c_int) -> c_int;
    pub fn win_wchstr    (win: *mut WINDOW, wch: *mut cchar_t) -> c_int;
    pub fn wunctrl       (wc: *mut cchar_t) -> *mut wchar_t;
    pub fn wvline_set    (win: *mut WINDOW, wch: *const cchar_t, n: c_int) -> c_int;
}

/* Quasi-standard */

extern "C" {
    pub fn getattrs  (win: *mut WINDOW) -> chtype;
    pub fn getbegx   (win: *mut WINDOW) -> c_int;
    pub fn getbegy   (win: *mut WINDOW) -> c_int;
    pub fn getmaxx   (win: *mut WINDOW) -> c_int;
    pub fn getmaxy   (win: *mut WINDOW) -> c_int;
    pub fn getparx   (win: *mut WINDOW) -> c_int;
    pub fn getpary   (win: *mut WINDOW) -> c_int;
    pub fn getcurx   (win: *mut WINDOW) -> c_int;
    pub fn getcury   (win: *mut WINDOW) -> c_int;
    pub fn traceoff  ();
    pub fn traceon   ();
    pub fn unctrl    (c: chtype) -> *mut c_char;

    pub fn crmode    () -> c_int;
    pub fn nocrmode  () -> c_int;
    pub fn draino    (ms: c_int) -> c_int;
    pub fn resetterm () -> c_int;
    pub fn fixterm   () -> c_int;
    pub fn saveterm  () -> c_int;
    pub fn setsyx    (y: c_int, x: c_int);

    pub fn mouse_set (mbe: mmask_t) -> c_int;
    pub fn mouse_on  (mbe: mmask_t) -> c_int;
    pub fn mouse_off (mbe: mmask_t) -> c_int;
    pub fn request_mouse_pos() -> c_int;
    pub fn wmouse_position  (win: *mut WINDOW, y: *mut c_int, x: *mut c_int);
    pub fn getmouse  () -> mmask_t;
}

/* ncurses */

extern "C" {
    pub fn assume_default_colors(fg: c_int, bg: c_int) -> c_int;
    pub fn curses_version       () -> *const c_char;
    pub fn has_key              (key: c_int) -> bool;
    pub fn is_keypad            (win: *const WINDOW) -> bool;
    pub fn is_leaveok           (win: *const WINDOW) -> bool;
    pub fn is_pad               (pad: *const WINDOW) -> bool;
    pub fn set_tabsize          (tabsize: c_int) -> c_int;
    pub fn use_default_colors   () -> c_int;
    pub fn wresize              (win: *mut WINDOW, nlines: c_int, ncols: c_int) -> c_int;

    pub fn has_mouse            () -> bool;
    pub fn mouseinterval        (wait: c_int) -> c_int;
    pub fn mousemask            (mast: mmask_t, oldmask: *mut mmask_t) -> mmask_t;
    pub fn mouse_trafo          (y: *mut c_int, x: *mut c_int, to_screen: bool) -> bool;
    pub fn nc_getmouse          (event: *mut MEVENT) -> c_int;
    pub fn ungetmouse           (event: *mut MEVENT) -> c_int;
    pub fn wenclose             (win: *const WINDOW, y: c_int, x: c_int) -> bool;
    pub fn wmouse_trafo         (win: *const WINDOW, y: *mut c_int, x: *mut c_int, to_screen: bool) -> bool;
}

/* PDCurses */

extern "C" {
    pub fn addrawch          (ch: chtype) -> c_int;
    pub fn insrawch          (ch: chtype) -> c_int;
    pub fn is_termresized    () -> bool;
    pub fn mvaddrawch        (y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn mvdeleteln        (y: c_int, x: c_int) -> c_int;
    pub fn mvinsertln        (y: c_int, x: c_int) -> c_int;
    pub fn mvinsrawch        (y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn mvwaddrawch       (win: *mut WINDOW, y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn mvwdeleteln       (win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    pub fn mvwinsertln       (win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    pub fn mvwinsrawch       (win: *mut WINDOW, y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn raw_output        (flag: bool) -> c_int;
    pub fn resize_term       (nlines: c_int, ncols: c_int) -> c_int;
    pub fn resize_window     (win: *mut WINDOW, nlines: c_int, ncols: c_int) -> *mut WINDOW;
    pub fn waddrawch         (win: *mut WINDOW, ch: chtype) -> c_int;
    pub fn winsrawch         (win: *mut WINDOW, ch: chtype) -> c_int;
    pub fn wordchar          () -> c_char;

    pub fn slk_wlabel        (labnum: c_int) -> *mut wchar_t;

    pub fn PDC_debug         (fmt: *const c_char, ...);
    pub fn PDC_get_version   (ver: *mut PDC_VERSION);
    pub fn PDC_ungetch       (ch: c_int) -> c_int;
    pub fn PDC_set_blink     (blinkon: bool) -> c_int;
    pub fn PDC_set_bold      (boldon: bool) -> c_int;
    pub fn PDC_set_line_color(color: c_short) -> c_int;
    pub fn PDC_set_title     (title: *const c_char);

    pub fn PDC_clearclipboard() -> c_int;
    pub fn PDC_freeclipboard (contents: *mut c_char) -> c_int;
    pub fn PDC_getclipboard  (contents: *mut *mut c_char, length: *mut c_long) -> c_int;
    pub fn PDC_setclipboard  (contents: *const c_char, length: c_long) -> c_int;

    pub fn PDC_get_key_modifiers   () -> c_ulong;
    pub fn PDC_return_key_modifiers(flag: bool) -> c_int;
}

/* NetBSD */

extern "C" {
    pub fn touchoverlap(win1: *const WINDOW, win2: *mut WINDOW) -> c_int;
    pub fn underend    () -> c_int;
    pub fn underscore  () -> c_int;
    pub fn wunderend   (win: *mut WINDOW) -> c_int;
    pub fn wunderscore (win: *mut WINDOW) -> c_int;
}

/*** Functions defined as macros ***/

/* getch() and ungetch() conflict with some DOS libraries */

pub unsafe fn getch()            -> c_int { wgetch(stdscr) }
pub unsafe fn ungetch(ch: c_int) -> c_int { PDC_ungetch(ch) }

pub unsafe fn COLOR_PAIR (n: c_short) -> attr_t { ((n as chtype) << PDC_COLOR_SHIFT) & A_COLOR }
pub unsafe fn PAIR_NUMBER(n: attr_t) -> c_short { (((n as chtype) & A_COLOR) >> PDC_COLOR_SHIFT) as c_short }

/* These will _only_ work as macros */

pub unsafe fn getbegyx(win: *mut WINDOW, y: &mut c_int, x: &mut c_int) { *y = getbegy(win); *x = getbegx(win) }
pub unsafe fn getmaxyx(win: *mut WINDOW, y: &mut c_int, x: &mut c_int) { *y = getmaxy(win); *x = getmaxx(win) }
pub unsafe fn getparyx(win: *mut WINDOW, y: &mut c_int, x: &mut c_int) { *y = getpary(win); *x = getparx(win) }
pub unsafe fn getyx   (win: *mut WINDOW, y: &mut c_int, x: &mut c_int) { *y = getcury(win); *x = getcurx(win) }

pub unsafe fn getsyx  (y: &mut c_int, x: &mut c_int) {
    if (*curscr)._leaveit {
        *y =- 1;
        *x =- 1;
    } else {
        getyx(curscr, y ,x);
    }
}

/* Deprecated */

#[deprecated]
pub unsafe fn PDC_save_key_modifiers(_: c_int) -> c_int { OK }
#[deprecated]
pub unsafe fn PDC_get_input_fd      ()         -> c_int { 0 }

/* return codes from PDC_getclipboard() and PDC_setclipboard() calls */

pub const PDC_CLIP_SUCCESS        : c_int = 0;
pub const PDC_CLIP_ACCESS_ERROR   : c_int = 1;
pub const PDC_CLIP_EMPTY          : c_int = 2;
pub const PDC_CLIP_MEMORY_ERROR   : c_int = 3;

/* PDCurses key modifier masks */

pub const PDC_KEY_MODIFIER_SHIFT  : c_ulong = 1;
pub const PDC_KEY_MODIFIER_CONTROL: c_ulong = 2;
pub const PDC_KEY_MODIFIER_ALT    : c_ulong = 4;
pub const PDC_KEY_MODIFIER_NUMLOCK: c_ulong = 8;

/* panel.h */

pub mod panel {
    use libc::{ c_int, c_void };
    use super::WINDOW;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct PANELOBS {
        pub above: *mut PANELOBS,
        pub pan: *mut PANELOBS,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct PANEL {
        pub win: *mut WINDOW,
        pub wstarty: c_int,
        pub wendy: c_int,
        pub wstartx: c_int,
        pub wendx: c_int,
        pub below: *mut PANEL,
        pub above: *mut PANEL,
        pub user: *mut c_void,
        pub obscure: *mut PANEL,
    }

    extern "C" {
        pub fn bottom_panel     (pan: *mut PANEL) -> c_int;
        pub fn del_panel        (pan: *mut PANEL) -> c_int;
        pub fn hide_panel       (pan: *mut PANEL) -> c_int;
        pub fn move_panel       (pan: *mut PANEL, starty: c_int, startx: c_int) -> c_int;
        pub fn new_panel        (win: *mut WINDOW) -> *mut PANEL;
        pub fn panel_above      (pan: *const PANEL) -> *mut PANEL;
        pub fn panel_below      (pan: *const PANEL) -> *mut PANEL;
        pub fn panel_hidden     (pan: *const PANEL) -> c_int;
        pub fn panel_userptr    (pan: *const PANEL) -> *const c_void;
        pub fn panel_window     (pan: *const PANEL) -> *mut WINDOW;
        pub fn replace_panel    (pan: *mut PANEL, win: *mut WINDOW) -> c_int;
        pub fn set_panel_userptr(pan: *mut PANEL, user: *const c_void) -> c_int;
        pub fn show_panel       (pan: *mut PANEL) -> c_int;
        pub fn top_panel        (pan: *mut PANEL) -> c_int;
        pub fn update_panels    ();
    }
}
