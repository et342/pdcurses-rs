//! Interface extensions.
//!

use super::*;

#[derive(Debug)]
pub enum GetStr {
    Utf(String),
    NonUtf(Vec<u8>),
}

impl GetStr {
    #[must_use = "extracted data will be lost"]
    pub fn string(&mut self) -> Option<String> {
        if let GetStr::Utf(it) = self {
            return Some(core::mem::take(it));
        }
        None
    }

    #[must_use = "extracted data will be lost"]
    pub fn bytes(&mut self) -> Option<Vec<u8>> {
        if let GetStr::NonUtf(it) = self {
            return Some(core::mem::take(it));
        }
        None
    }
}

#[inline(always)]
pub fn getstr() -> Result<GetStr, ()> {
    self::wgetstr(crate::stdscr())
}

pub fn wgetstr(win: WINDOW) -> Result<GetStr, ()> {
    /*
        VERCHECK 3.9
            if PDC_WIDE is defined
                wgetnstr reads max(N, MAXLINE=255) of wide characters
            otherwise
                wgetnstr reads N narrow characters
    */

    static mut BUF: [i8; BUF_LEN] = [0; BUF_LEN];
    const  BUF_LEN: usize = 2048;

    if ERR == unsafe { sys::wgetstr(win, BUF.as_mut_ptr()) } {
        return Err(());
    }

    let len = unsafe {
        libc::strnlen(BUF.as_ptr(), BUF_LEN)
    };

    let bytes = unsafe {
        core::slice::from_raw_parts(BUF.as_ptr() as *const u8, len)
    };

    if let Ok(it) = core::str::from_utf8(bytes) {
        return Ok(GetStr::Utf(it.to_owned()));
    }

    Ok(GetStr::NonUtf(bytes.to_owned()))
}

#[inline(always)]
pub fn get_wstr() -> Result<Vec<u16>, ()> {
    self::wget_wstr(stdscr())
}

pub fn wget_wstr(win: WINDOW) -> Result<Vec<u16>, ()> {
    static mut BUF: [u16; BUF_LEN] = [0; BUF_LEN];
    const  BUF_LEN: usize = 2048;

    if ERR == unsafe { sys::wgetn_wstr(win, BUF.as_mut_ptr(), BUF_LEN as i32) } {
        return Err(());
    }

    let len = unsafe {
        libc::wcslen(BUF.as_ptr())
    };

    let wcs = unsafe {
        &BUF[..len]
    };

    Ok(wcs.to_owned())
}
