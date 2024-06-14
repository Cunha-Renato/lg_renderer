mod opengl;
mod vulkan;
pub mod renderer_core;

pub mod new_renderer;

pub(crate) type StdError = Box<dyn std::error::Error>;