pub mod OpenGL {
  //include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
  //include!("./bindings/gl-41-debug.rs");
  include!("./bindings/gl-41-release.rs");
}

pub use crate::OpenGL as GL; // OpenGl constants
pub use crate::OpenGL::types as GLT; // OpenGl Types
pub use crate::OpenGL::Gl; // OpenGl instance

// OpenGl type mappings
pub type RenderID = GLT::GLuint;
pub type SizeIntPtr = GLT::GLsizeiptr;
pub type ConstVoid = *const GLT::GLvoid;
pub type Shader = GLT::GLuint;
pub type Program = GLT::GLuint;

// Modules imports here
pub mod buffers;
pub mod render;
pub mod shader;

pub use buffers::ibo::IndexBuffer;
pub use buffers::layout::{Layout, VertexBufferElement};
pub use buffers::vao::VertexArray;
pub use buffers::vbo::VertexBuffer;
pub use render::RenderTarget;
