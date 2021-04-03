use crate::GL;

use std::ffi::CString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderError {
  #[error("Can't compile shader")]
  Compilation,
}

#[repr(u32)]
#[derive(Clone, Debug)]
pub enum ShaderType {
  Vertex = GL::VERTEX_SHADER,
  Fragment = GL::FRAGMENT_SHADER,
}

#[derive(Debug, Error)]
pub enum ProgramError {
  #[error("Can't link shader")]
  Linking,
  #[error("Uniform `{0:?}` location not found in shader")]
  NoLocation(CString),
  #[error("Uniform `{0:?}` create this location before get")]
  CreateLocation(CString),
}

pub mod program;
pub mod utils;
