use gl::types::{GLsizeiptr, GLfloat};

use crate::AsRaw;

pub struct Vao {
    inner: u32,
    vbo: Vbo
}

impl Vao {
    pub fn gen(n: usize) -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(n as i32,&mut vao);
        }
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(n as i32,&mut vbo);
        }
        let vbo = Vbo { inner: vbo };

        Self { inner: vao, vbo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.inner);
        }
    }

    pub fn vbo(&self) -> &Vbo {
        &self.vbo
    }
}

impl AsRaw<u32> for Vao {
    fn as_raw(&self) -> u32 {
        self.inner
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1,&self.inner);
        }
    }
}

pub struct Vbo {
    inner: u32   
}

impl Vbo {
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER,self.inner);    
        }
    }

    pub fn data<T>(&self,data: &[T]) {
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER,(data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,std::mem::transmute(&data[0]),gl::STATIC_DRAW);
        }
    }
}

impl AsRaw<u32> for Vbo {
    fn as_raw(&self) -> u32 {
        self.inner
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1,&self.inner);
        }
    }
}