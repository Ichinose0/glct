use std::ffi::CString;

use gl::types::{GLfloat, GLsizeiptr, GLuint, GLboolean};
use glct::{
    shader::{Program, Shader, ShaderKind},
    wgl::Wgl, AsRaw, vbo::Vao,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    raw_window_handle::HasWindowHandle,
    window::WindowBuilder,
};

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

// Shader sources
static VS_SRC: &'static str = "
#version 150
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";

static FS_SRC: &'static str = "
#version 150
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let handle = window.window_handle().unwrap();

    let wgl = match handle.as_raw() {
        raw_window_handle::RawWindowHandle::Win32(handle) => Wgl::create(isize::from(handle.hwnd)),
        _ => panic!("Unknown error"),
    };

    glct::init(|s| wgl.get_proc_address(s));

    unsafe { wgl.swap_intervals(true) };

    let fragment = Shader::new(ShaderKind::Fragment, FS_SRC);
    let vertex = Shader::new(ShaderKind::Vertex, VS_SRC);
    let program = Program::create(&[fragment, vertex]);


    let vao = Vao::gen(1);
    let vbo = vao.vbo();
    vao.bind();
    vbo.bind();
    vbo.data(&VERTEX_DATA);

    unsafe {
        // Use shader program
        program.use_program();
        gl::BindFragDataLocation(program.as_raw(), 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program.as_raw(), CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            std::ptr::null(),
        );
    }

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::RedrawRequested => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::ClearColor(1.0, 1.0, 0.0, 1.0);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }

                window.pre_present_notify();
                wgl.swap_buffers();
            }
            WindowEvent::Resized(size) => {
                unsafe {
                    gl::Viewport(0,0,size.width as i32,size.height as i32);
                }
            }
            _ => (),
        },
        Event::AboutToWait => {
            window.request_redraw();
        }

        _ => (),
    })
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("Sorry. This example is supported only windows.")
}
