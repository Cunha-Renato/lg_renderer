use crate::{gl_check, renderer::lg_texture::{LgTexture, TextureFormat, TextureType}};

use super::GlError;

#[derive(Debug, Default)]
pub(crate) struct GlTexture {
    id: gl::types::GLuint,
}
impl GlTexture {
    pub(crate) fn new() -> Result<Self, GlError> {
        let mut id = 0;
        gl_check!(gl::GenTextures(1, &mut id), "Failed to generate texture!")?;
        
        Ok(Self { id })
    }
    pub(crate) fn bind(&self, location: u32) -> Result<(), GlError> {
        gl_check!(gl::ActiveTexture(gl::TEXTURE0 + location), "Failed to activate texture! (binding)")?;
        gl_check!(gl::BindTexture(gl::TEXTURE_2D, self.id), "Failed to bind texture! (binding)")
    }
    pub(crate) fn unbind(&self) -> Result<(), GlError> {
        gl_check!(gl::BindTexture(gl::TEXTURE_2D, 0), "Failed to unbind texture!")
    }
    pub(crate) fn load(&self, texture: &impl LgTexture) -> Result<(), GlError> {
        gl_check!(
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                tex_format_to_opengl(texture.texture_format()) as i32,
                texture.width() as i32, 
                texture.height() as i32, 
                0, 
                tex_format_to_opengl(texture.texture_format()), 
                tex_type_to_opengl(texture.texture_type()), 
                texture.bytes().as_ptr() as *const _,
            ),
            "Failed to load texture!"
        )?;

        gl_check!(gl::GenerateMipmap(gl::TEXTURE_2D), "Failed to generate mip map!")?;
        gl_check!(gl::GenerateTextureMipmap(self.id), "Failed to generate mip map for texture!")
    }
}
impl Drop for GlTexture {
    fn drop(&mut self) {
        gl_check!(gl::DeleteTextures(1, [self.id].as_ptr()), "Failed to delete texture!").unwrap();
    }
}

fn tex_type_to_opengl(tex_type: TextureType) -> gl::types::GLenum {
    match tex_type {
        TextureType::UNSIGNED_BYTE => gl::UNSIGNED_BYTE,
    }
}
fn tex_format_to_opengl(tex_format: TextureFormat) -> gl::types::GLenum {
    match tex_format {
        TextureFormat::RGBA => gl::RGBA,
    }
}