use gl as GL;
use gl::types as GLT;
use std::ffi::CString;
use std::ptr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderError {
  #[error("Can't compile shader")]
  Compilation,
}

#[derive(Debug, Error)]
pub enum ProgramError {
  #[error("Can't link shader")]
  Linking,
}

#[repr(u32)]
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Type {
  Vertex = GL::VERTEX_SHADER,
  Fragment = GL::FRAGMENT_SHADER,
}

pub type Shader = GLT::GLuint;
pub type Program = GLT::GLuint;

pub fn compile(gl: &GL::Gl, shader_type: Type, source: &str) -> Result<Shader, ShaderError> {
  let id = unsafe { gl.CreateShader(shader_type as u32) };
  let cs_source = CString::new(source).expect("CString::new failed");

  unsafe {
    gl.ShaderSource(id, 1, &cs_source.as_ptr(), ptr::null());
    gl.CompileShader(id);
  };

  check_shader_errors(&gl, id).unwrap();

  Ok(id)
}

fn check_shader_errors(gl: &GL::Gl, id: Shader) -> Result<(), ShaderError> {
  let mut success: GLT::GLint = 1;
  let mut len: GLT::GLint = 0;

  unsafe {
    gl.GetShaderiv(id, GL::COMPILE_STATUS, &mut success);
  }

  if success == 0 {
    unsafe {
      gl.GetShaderiv(id, GL::INFO_LOG_LENGTH, &mut len);
    }

    let mut buffer = Vec::with_capacity(len as usize);
    let buf_ptr = buffer.as_mut_ptr() as *mut gl::types::GLchar;

    unsafe {
      buffer.set_len(len as usize);
    }

    unsafe {
      gl.GetShaderInfoLog(id, len, ptr::null_mut(), buf_ptr);
    }

    let cs_error = unsafe { CString::from_vec_unchecked(buffer) };
    let all_errors = cs_error.into_string().unwrap();

    println!("[OpenGL] Shader compilation error:");
    let errors = all_errors.split("\n");
    for error in errors {
      println!("[OpenGL] {:?}", error);
    }

    return Err(ShaderError::Compilation);
  }

  Ok(())
}

pub fn create_program(gl: &GL::Gl, vertex: Shader, fragment: Shader) -> Result<Program, ProgramError> {
  let program = unsafe { gl.CreateProgram() };

  unsafe {
    gl.AttachShader(program, vertex);
    gl.AttachShader(program, fragment);
    gl.LinkProgram(program);
  }

  check_program_errors(&gl, program).unwrap();

  unsafe {
    gl.DeleteProgram(vertex);
    gl.DeleteProgram(fragment);
  }

  Ok(program)
}

fn check_program_errors(gl: &GL::Gl, id: Program) -> Result<(), ProgramError> {
  let mut success: GLT::GLint = 1;
  let mut len: GLT::GLint = 0;

  unsafe {
    gl.GetProgramiv(id, GL::LINK_STATUS, &mut success);
  }

  if success == 0 {
    unsafe {
      gl.GetProgramiv(id, GL::INFO_LOG_LENGTH, &mut len);
    }

    let mut buffer = Vec::with_capacity(len as usize);
    let buf_ptr = buffer.as_mut_ptr() as *mut gl::types::GLchar;

    unsafe {
      buffer.set_len(len as usize);
    }

    unsafe {
      gl.GetProgramInfoLog(id, len, ptr::null_mut(), buf_ptr);
    }

    let cs_error = unsafe { CString::from_vec_unchecked(buffer) };
    let all_errors = cs_error.into_string().unwrap();

    println!("[OpenGL] Shader program error:");
    let errors = all_errors.split("\n");
    for error in errors {
      println!("[OpenGL] {:?}", error);
    }

    return Err(ProgramError::Linking);
  }

  Ok(())
}
