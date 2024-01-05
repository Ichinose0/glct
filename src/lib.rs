use std::ffi::c_void;

pub mod shader;
#[cfg(feature = "wgl")]
pub mod wgl;
pub mod vbo;

pub trait AsRaw<R> 
where
    R: Sized + Clone + Copy
{
    fn as_raw(&self) -> R;
}

pub fn init<F>(load: F)
where
    F: FnMut(&'static str) -> *const c_void,
{
    gl::load_with(load);
}
