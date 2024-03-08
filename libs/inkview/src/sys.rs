use std::ffi::*;

pub type MainHandler = unsafe extern "C" fn(c_int, c_int, c_int) -> c_int;
pub type FontPtr = *mut c_void;

pub const EVT_INIT: c_int = 21;
pub const EVT_EXIT: c_int = 22;
pub const EVT_KEYPRESS: c_int = 25;

pub const ICON_INFORMATION: c_int = 1;
pub const ICON_ERROR: c_int = 4;

pub const ALIGN_CENTER: c_int = 2;

pub const BLACK: c_int = 0x000000;
pub const WHITE: c_int = 0xffffff;

#[link(name = "inkview")]
extern "C" {
    pub fn InkViewMain(handler: MainHandler);
    pub fn CloseApp();
    pub fn GetThemeFont(name: *const c_char, default: *const c_char) -> FontPtr;
    pub fn SetFont(font: FontPtr, color: c_int);
    pub fn ClearScreen();
    pub fn FillArea(x: c_int, y: c_int, w: c_int, h: c_int, color: c_int);
    pub fn ScreenWidth() -> c_int;
    pub fn ScreenHeight() -> c_int;
    pub fn DrawTextRect(x: c_int, y: c_int, w: c_int, h: c_int, s: *const c_char, flags: c_int);
    pub fn PartialUpdateBW(x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn TextRectHeight(width: c_int, s: *const c_char, flags: c_int) -> c_int;
    pub fn ShowHourglassAt(x: c_int, y: c_int);
    pub fn HideHourglass();
    pub fn Message(icon: c_int, title: *const c_char, msg: *const c_char, tout: c_int);
    pub fn SendEvent(handler: MainHandler, t: c_int, p1: c_int, p2: c_int);
}
