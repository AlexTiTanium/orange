use crate::GL;

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
}

pub mod program;
pub mod utils;
