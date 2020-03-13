pub use crate::bindings::Gl; // OpenGl instance
pub use crate::bindings::GL; // OpenGl constants
pub use crate::bindings::GLT; // OpenGl Types

// OpenGl type mappings
pub type RenderID = GLT::GLuint;
pub type SizeIntPtr = GLT::GLsizeiptr;
pub type ConstVoid = *const GLT::GLvoid;
pub type ShaderID = GLT::GLuint;
pub type ProgramID = GLT::GLuint;

// Modules imports here
pub mod bindings;
pub mod buffers;
pub mod render;
pub mod shaders;

pub use buffers::ibo::IndexBuffer;
pub use buffers::layout::{Layout, VertexBufferElement};
pub use buffers::vao::VertexArray;
pub use buffers::vbo::VertexBuffer;
pub use render::Renderer;

pub use shaders::program::Program;
pub use shaders::utils;
pub use shaders::ShaderType;
