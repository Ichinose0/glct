#![allow(non_snake_case)]

use std::{
    ffi::{c_void, CString},
    mem::{size_of, MaybeUninit},
    ptr::{null, null_mut},
};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        Graphics::{
            Gdi::{GetDC, ReleaseDC, HDC},
            OpenGL::*,
        },
    },
};

type wglCreateContextAttribsARB =
    fn(hDC: *mut c_void, hshareContext: *mut c_void, attribList: *const i32) -> *mut c_void;
type wglSwapIntervalEXT = fn(i32);

const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
const WGL_CONTEXT_LAYER_PLANE_ARB: i32 = 0x2093;
const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;

const WGL_CONTEXT_DEBUG_BIT_ARB: i32 = 0x0001;
const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: i32 = 0x0002;

const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;
const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: i32 = 0x00000002;

const ERROR_INVALID_VERSION_ARB: i32 = 0x2095;
const ERROR_INVALID_PROFILE_ARB: i32 = 0x2096;

pub struct Wgl {
    hwnd: HWND,
    hdc: HDC,
    ctx: HGLRC,
    wglCreateContextAttribsARB: wglCreateContextAttribsARB,
    wglSwapIntervalEXT: wglSwapIntervalEXT,
}

impl Wgl {
    #[inline]
    pub fn create(hwnd: isize) -> Self {
        let hwnd = HWND(hwnd);
        let mut hdc = unsafe { GetDC(hwnd) };
        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cRedBits: 0,
            cRedShift: 0,
            cGreenBits: 0,
            cGreenShift: 0,
            cBlueBits: 0,
            cBlueShift: 0,
            cAlphaBits: 0,
            cAlphaShift: 0,
            cAccumBits: 0,
            cAccumRedBits: 0,
            cAccumGreenBits: 0,
            cAccumBlueBits: 0,
            cAccumAlphaBits: 0,
            cDepthBits: 24,
            cStencilBits: 8,
            cAuxBuffers: 0,
            iLayerType: 0,
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: 0,
            dwDamageMask: 0,
        };

        let pixel_format = unsafe { ChoosePixelFormat(hdc, &pfd) };
        let ctx = unsafe {
            SetPixelFormat(hdc, pixel_format, &pfd).unwrap();
            let ctx = wglCreateContext(hdc).unwrap();
            wglMakeCurrent(hdc, ctx).unwrap();
            ctx
        };

        let attribs = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,
            3,
            WGL_CONTEXT_MINOR_VERSION_ARB,
            2,
            WGL_CONTEXT_PROFILE_MASK_ARB,
            WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
            0,
        ];

        let wglCreateContextAttribsARB = load_wgl_function("wglCreateContextAttribsARB");
        if wglCreateContextAttribsARB.is_null() {
            panic!("Could not load wglCreateContextAttribsARB");
        }

        let wglCreateContextAttribsARB: wglCreateContextAttribsARB =
            unsafe { std::mem::transmute(wglCreateContextAttribsARB) };
        let new_ctx = HGLRC((wglCreateContextAttribsARB)(
            hdc.0 as *mut c_void,
            null_mut(),
            attribs.as_ptr(),
        ) as isize);
        unsafe {
            wglDeleteContext(ctx).unwrap();
            wglMakeCurrent(hdc, new_ctx).unwrap();
        }

        let wglSwapIntervalEXT = load_wgl_function("wglSwapIntervalEXT");
        if wglSwapIntervalEXT.is_null() {
            panic!("Could not load wglSwapIntervalEXT");
        }

        let wglSwapIntervalEXT: wglSwapIntervalEXT =
            unsafe { std::mem::transmute(wglSwapIntervalEXT) };

        Self {
            wglCreateContextAttribsARB,
            wglSwapIntervalEXT,
            hdc,
            ctx: new_ctx,
            hwnd,
        }
    }

    #[inline]
    pub unsafe fn make_current(&self) {}

    #[inline]
    pub unsafe fn swap_intervals(&self, interval: bool) {
        ((self.wglSwapIntervalEXT)(interval as i32));
    }

    #[inline]
    pub fn swap_buffers(&self) {
        unsafe {
            SwapBuffers(self.hdc).unwrap();
        }
    }

    #[inline]
    pub fn get_proc_address(&self, procname: &str) -> *const c_void {
        unsafe {
            wglGetProcAddress(PCSTR(format!("{}\0", procname).as_ptr())).unwrap() as *const c_void
        }
    }
}

impl Drop for Wgl {
    fn drop(&mut self) {
        unsafe {
            ReleaseDC(self.hwnd, self.hdc);
            wglDeleteContext(self.ctx).unwrap();
        }
    }
}

#[inline]
fn load_wgl_function(name: &str) -> *const c_void {
    unsafe {
        match wglGetProcAddress(PCSTR(format!("{}\0", name).as_ptr())) {
            Some(ptr) => ptr as *const c_void,
            None => null(),
        }
    }
}
