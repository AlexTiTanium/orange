use crate::shaders::ProgramError;
use crate::shaders::ShaderError;
use crate::shaders::ShaderType;
use crate::Gl;
use crate::ProgramID;
use crate::ShaderID;
use crate::GL;
use crate::GLT;

use std::ffi::CString;
use std::ptr;

pub fn compile(gl: &Gl, shader_type: ShaderType, source: &str) -> Result<ShaderID, ShaderError> {
  let id = unsafe { gl.CreateShader(shader_type as u32) };
  let cs_source = CString::new(source).expect("CString::new failed");

  unsafe {
    gl.ShaderSource(id, 1, &cs_source.as_ptr(), ptr::null());
    gl.CompileShader(id);
  };

  check_shader_errors(&gl, id).unwrap();

  Ok(id)
}

fn check_shader_errors(gl: &Gl, id: ShaderID) -> Result<(), ShaderError> {
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
    let buf_ptr = buffer.as_mut_ptr() as *mut GLT::GLchar;

    unsafe {
      buffer.set_len(len as usize);
    }

    unsafe {
      gl.GetShaderInfoLog(id, len, ptr::null_mut(), buf_ptr);
    }

    let cs_error = unsafe { CString::from_vec_unchecked(buffer) };
    let all_errors = cs_error.into_string().unwrap();

    println!("[OpenGL] Shader compilation error:");
    let errors = all_errors.split('\n');
    for error in errors {
      println!("[OpenGL] {:?}", error);
    }

    return Err(ShaderError::Compilation);
  }

  Ok(())
}

pub fn check_program_errors(gl: &Gl, id: ProgramID) -> Result<(), ProgramError> {
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
    let buf_ptr = buffer.as_mut_ptr() as *mut GLT::GLchar;

    unsafe {
      buffer.set_len(len as usize);
    }

    unsafe {
      gl.GetProgramInfoLog(id, len, ptr::null_mut(), buf_ptr);
    }

    let cs_error = unsafe { CString::from_vec_unchecked(buffer) };
    let all_errors = cs_error.into_string().unwrap();

    println!("[OpenGL] Shader program error:");
    let errors = all_errors.split('\n');
    for error in errors {
      println!("[OpenGL] {:?}", error);
    }

    return Err(ProgramError::Linking);
  }

  Ok(())
}
