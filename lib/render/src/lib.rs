use gl as GL;
use gl::types as GLT;
use state::Store;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::vec::Vec;

pub static SHADER_BASIC_VERT: &'static str =
  include_str!("../../../resources/shader_basic_vert.glsl");

pub fn init(_store: &Store, gl: &GL::Gl) {
  let vertices: [f32; 6] = [-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];

  let mut vbo: GLT::GLuint = 0;
  let vertShaderSource = CString::new(SHADER_BASIC_VERT).expect("CString::new failed");
  let mut vertexShader: GLT::GLuint = 0;
  let mut shaderCompileSuccess: GLT::GLint = 1;
  let mut infoLog: GLT::GLchar = 0;
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
      0,         // index of the generic vertex attribute ("layout (location = 0)")
      3,         // the number of components per generic vertex attribute
      gl::FLOAT, // data type
      gl::FALSE, // normalized (int-to-float conversion)
      (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
      ptr::null(),                                          // offset of the first component
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);

    gl.Viewport(0, 0, 600, 800);
    gl.ClearColor(0.3, 0.3, 0.5, 1.0);

    vertexShader = gl.CreateShader(GL::VERTEX_SHADER);

    gl.ShaderSource(vertexShader, 1, &vertShaderSource.as_ptr(), ptr::null());
    gl.CompileShader(vertexShader);

    gl.GetShaderiv(vertexShader, GL::COMPILE_STATUS, &mut shaderCompileSuccess);

    if shaderCompileSuccess == 0 {
      let mut len = 0;
      gl.GetShaderiv(vertexShader, GL::INFO_LOG_LENGTH, &mut len);

      let mut buffer = Vec::with_capacity(len as usize);
      let buf_ptr = buffer.as_mut_ptr() as *mut gl::types::GLchar;
      gl.GetShaderInfoLog(vertexShader, len, ptr::null_mut(), buf_ptr);
      buffer.set_len(len as usize);

      panic!(
        "Vertext shader error: {:?}",
        CString::from_vec_unchecked(buffer)
      );
    }
  }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  // allocate buffer of correct size
  let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
  // fill it with len spaces
  buffer.extend([b' '].iter().cycle().take(len));
  // convert buffer to CString
  unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn step(_store: &Store, gl: &GL::Gl) {
  unsafe {
    // gl.ClearColor(1.0, 0.0, 0.0, 0.0);
    // gl.Clear(GL::COLOR_BUFFER_BIT);
  }
}
