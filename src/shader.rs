use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

pub enum ShaderKind {
    Fragment,
    Vertex
}

#[derive(Debug)]
pub struct Shader {
    inner: u32
}

impl Shader {
    pub fn new(kind: ShaderKind,source: &str) -> Self {
        let shader_type = match kind {
            ShaderKind::Fragment => gl::VERTEX_SHADER,
            ShaderKind::Vertex => gl::FRAGMENT_SHADER,
        };

        let inner = compile_shader(shader_type,source);

        Self {
            inner
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.inner);
        }
    }
}

pub struct Program {
    inner: u32
}

impl Program {
    #[inline]
    pub fn create(shaders: &[Shader]) -> Self {
        let mut raw = vec![];
        for i in shaders {
            raw.push(i.inner);
        }

        let inner = create_program(&raw);

        Self {
            inner
        }
    }

    #[inline]
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.inner);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.inner);
        }
    }
}

#[inline]
fn compile_shader(shader_type: u32, source: &str) -> u32 {
    let mut result = 0;

    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &CString::new(source).unwrap().as_ptr(), null());
        gl::CompileShader(shader);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
        if (result as u8) == gl::FALSE {
            let mut length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
            let mut log = Vec::with_capacity(length as usize);
            gl::GetShaderInfoLog(shader, length, null_mut(), log.as_mut_ptr());
            let log = CStr::from_ptr(log.as_ptr());
            let log_str = log.to_str().unwrap();
            panic!("{:#?}", log_str);
        }

        shader
    }
}

#[inline]
fn create_program(shaders: &[u32]) -> u32 {
    unsafe {
        let program = gl::CreateProgram();
        for i in shaders {
            gl::AttachShader(program, *i);
        }

        gl::LinkProgram(program);

        let mut result = 0;

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut result);

        if (result as u8) == gl::FALSE {
            let mut length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
            let mut log = Vec::with_capacity(length as usize);
            gl::GetProgramInfoLog(program, length, null_mut(), log.as_mut_ptr());
            let log = CStr::from_ptr(log.as_ptr());
            let log_str = log.to_str().unwrap();
            panic!("{:#?}", log_str);
        }

        program
    }
}