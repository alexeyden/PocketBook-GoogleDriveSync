mod sys;

use std::{ffi::*, future::Future};

pub struct Hourglass {
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
}

impl Hourglass {
    const PADDING: c_int = 40;

    pub async fn show(s: &str) -> Self {
        let (w, h) = unsafe { (sys::ScreenWidth(), sys::ScreenHeight()) };

        let text_width = w - Self::PADDING;
        let mut hg = Hourglass {
            x: w / 2 - text_width / 2,
            y: h / 2 + Self::PADDING,
            w: text_width,
            h: 0,
        };

        unsafe {
            sys::ShowHourglassAt(w / 2, h / 2 - 20);
        }

        hg.update_text(s).await;

        hg
    }

    pub async fn update_text(&mut self, s: &str) {
        let w = unsafe { sys::ScreenWidth() };

        let text_width = w - Self::PADDING;

        let ss = std::ffi::CString::new(s).unwrap();

        unsafe {
            sys::FillArea(self.x, self.y, self.w, self.h, sys::WHITE);

            let text_height = sys::TextRectHeight(text_width, ss.as_ptr(), sys::ALIGN_CENTER);
            sys::DrawTextRect(
                self.x,
                self.y,
                self.w,
                text_height,
                ss.as_ptr(),
                sys::ALIGN_CENTER,
            );

            sys::PartialUpdateBW(self.x, self.y, self.w, text_height.max(self.h));

            self.h = text_height;
        }

        sched_point().await;
    }

    pub async fn hide(self) {
        unsafe {
            sys::FillArea(self.x, self.y, self.w, self.h, sys::WHITE);
            sys::PartialUpdateBW(self.x, self.y, self.w, self.h);
            sys::HideHourglass();
        }

        sched_point().await;
    }
}

pub async fn show_message_error(title: &str, msg: &str) {
    let title = std::ffi::CString::new(title).unwrap();
    let msg = std::ffi::CString::new(msg).unwrap();

    unsafe {
        sys::Message(sys::ICON_ERROR, title.as_ptr(), msg.as_ptr(), 5000);
    }

    sched_point().await;
}

pub async fn show_message_info(title: &str, msg: &str) {
    let title = std::ffi::CString::new(title).unwrap();
    let msg = std::ffi::CString::new(msg).unwrap();

    unsafe {
        sys::Message(sys::ICON_INFORMATION, title.as_ptr(), msg.as_ptr(), 5000);
    }

    sched_point().await;
}

fn close() {
    unsafe { sys::CloseApp() };
}

pub async fn sched_point() {
    unsafe {
        sys::SendEvent(handler, EVT_POLL, 0, 0);
    }
    std::future::ready(()).await;
}

const EVT_POLL: c_int = 9999;

/// copied from noop_waker crate
mod noop_waker {
    use core::{
        ptr,
        task::{RawWaker, RawWakerVTable, Waker},
    };

    #[inline]
    #[must_use]
    pub fn noop_waker() -> Waker {
        let raw = RawWaker::new(ptr::null(), &NOOP_WAKER_VTABLE);

        // SAFETY: the contracts for RawWaker and RawWakerVTable are upheld
        unsafe { Waker::from_raw(raw) }
    }

    const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

    unsafe fn noop_clone(_p: *const ()) -> RawWaker {
        // SAFETY: this retains all of the waker's resources, of which there are none
        RawWaker::new(ptr::null(), &NOOP_WAKER_VTABLE)
    }

    unsafe fn noop(_p: *const ()) {}
}

static mut GLOBAL_HANDLER: Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>> = None;

unsafe extern "C" fn handler(evt: c_int, _p1: c_int, _p2: c_int) -> c_int {
    let waker = noop_waker::noop_waker();
    let mut cx = std::task::Context::from_waker(&waker);

    let fut = { &mut GLOBAL_HANDLER }.as_mut().unwrap().as_mut();

    match evt {
        sys::EVT_INIT => unsafe {
            sys::ClearScreen();

            let font = sys::GetThemeFont(
                b"menu.font.normal\0".as_ptr() as *const c_char,
                b"\0".as_ptr() as *const c_char,
            );
            sys::SetFont(font, sys::BLACK);

            sys::SendEvent(handler, EVT_POLL, 0, 0);
        },
        sys::EVT_EXIT | sys::EVT_KEYPRESS => close(),
        EVT_POLL => {
            if fut.poll(&mut cx).is_ready() {
                close();
                return 0;
            }
            sys::SendEvent(handler, EVT_POLL, 0, 0);
        }
        _ => {}
    }

    0
}

pub fn start(cb: impl Future<Output = ()> + 'static) {
    unsafe { GLOBAL_HANDLER = Some(Box::pin(cb)) };
    unsafe { sys::InkViewMain(handler) };
}
