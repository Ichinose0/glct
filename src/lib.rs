use std::ffi::c_void;

pub mod shader;

pub fn init<F>(load: F) 
where F: FnMut(&'static str) -> *const c_void
{
    gl::load_with(load);
}