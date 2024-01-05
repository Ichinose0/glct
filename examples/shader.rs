use glct::shader::{Shader, ShaderKind, Program};

const FRAGMENT: &str = include_str!("shader.frag");
const VERTEX: &str = include_str!("shader.vert");

fn main() {
    let fragment = Shader::new(ShaderKind::Fragment,FRAGMENT);
    let vertex = Shader::new(ShaderKind::Vertex,VERTEX);
    let program = Program::create(&[fragment,vertex]);
    program.use_program();
}