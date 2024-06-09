use std::ffi::CString;

use crate::{gl_check, StdError};

use super::GlError;

#[derive(Debug, Default)]
pub struct GlProgram {
    id: gl::types::GLuint,
    pub shaders: Vec<gl::types::GLuint>,
}
impl GlProgram {
    pub(crate) fn new() -> Result<Self, GlError> {
        let id: u32;
        gl_check!(id = gl::CreateProgram(), "Failed to create shader program!")?;

        Ok(Self {
            id,
            shaders: Vec::new()
        })
    }
    pub(crate) fn id(&self) -> gl::types::GLuint {
        self.id
    }
    pub(crate) fn contains(&self, shaders: &[gl::types::GLuint]) -> bool {
        shaders.iter().all(|s| self.shaders.contains(s))
    }
    pub(crate) fn set_shaders(&mut self, shaders: Vec<gl::types::GLuint>) -> Result<(), GlError> {
        for s in &shaders {
            gl_check!(gl::AttachShader(self.id, *s), "Failed to attach shader!")?;
        }
        
        self.shaders = shaders;
        Ok(())
    }
    pub(crate) fn add_shader(&mut self, shader: gl::types::GLuint) -> Result<(), GlError> {
        gl_check!(gl::AttachShader(self.id, shader), "Failed to attach shader!")?;
        
        self.shaders.push(shader);
        Ok(())
    }
    pub(crate) fn use_prog(&self) -> Result<(), GlError> {
        gl_check!(gl::UseProgram(self.id), "Failed to use shader program!")
    }
    pub(crate) fn unuse(&self) -> Result<(), GlError> {
        gl_check!(gl::UseProgram(0), "Failed to unuse shader program!")
    }
    pub(crate) fn get_attrib_location(&self, attrib: &str) -> Result<gl::types::GLuint, StdError>
    {
        let attrib = CString::new(attrib)?;
        let location: u32;
        gl_check!(location = gl::GetAttribLocation(self.id, attrib.as_ptr()) as gl::types::GLuint, "Failed to get attribute location!")?;
            
        Ok(location)
    }
    pub(crate) fn link(&self) -> Result<(), GlError>{
        gl_check!(gl::LinkProgram(self.id), "Failed to link shader program!")
    }
}
impl Drop for GlProgram {
    fn drop(&mut self) {
        gl_check!(gl::DeleteProgram(self.id), "Failed do delete shader program!").unwrap();
    }
}