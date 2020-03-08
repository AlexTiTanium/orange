mod shader;
use gl as GL;
use gl::types as GLT;
use state::Store;
use std::mem;
use std::ptr;
use std::str;

type VertexBufferObject = GLT::GLuint;
type VertexArrayObject = GLT::GLuint;

pub struct RenderTarget {
  vbo: VertexBufferObject,
  vao: VertexArrayObject,
  program: shader::Program,
}

pub static SHADER_BASIC_VERT: &'static str = include_str!("../../../resources/shader_basic_vert.glsl");
pub static SHADER_BASIC_FRAG: &'static str = include_str!("../../../resources/shader_basic_frag.glsl");

pub fn create_target(gl: &GL::Gl) -> RenderTarget {
  let vertices: [f32; 6] = [-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];

  let mut vbo: GLT::GLuint = 0;
  let mut vao: GLT::GLuint = 0;

  unsafe {
    gl.GenBuffers(1, &mut vbo);
    gl.BindBuffer(GL::ARRAY_BUFFER, vbo);

    gl.BufferData(
      GL::ARRAY_BUFFER,
      mem::size_of_val(&vertices) as GLT::GLsizeiptr,
      vertices.as_ptr() as *const GLT::GLvoid,
      GL::STATIC_DRAW,
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);

    gl.GenVertexArrays(1, &mut vao);

    gl.BindVertexArray(vao);
    gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    gl.VertexAttribPointer(
      0,                                                    // index of the generic vertex attribute ("layout (location = 0)")
      2,                                                    // the number of components per generic vertex attribute
      gl::FLOAT,                                            // data type
      gl::FALSE,                                            // normalized (int-to-float conversion)
      (2 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
      ptr::null(),                                          // offset of the first component
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
  }

  let vert_shader = shader::compile(&gl, shader::Type::Vertex, &SHADER_BASIC_VERT).unwrap();
  let frag_shader = shader::compile(&gl, shader::Type::Fragment, &SHADER_BASIC_FRAG).unwrap();
  let program = shader::create_program(&gl, vert_shader, frag_shader).unwrap();

  RenderTarget { program, vao, vbo }
}

pub fn step(gl: &GL::Gl, target: &RenderTarget) {
  unsafe {
    gl.UseProgram(target.program);
    gl.BindVertexArray(target.vao);
    gl.DrawArrays(GL::TRIANGLES, 0, 3);
  }
}
