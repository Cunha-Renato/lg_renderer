use crate::{gl_check, renderer::lg_texture::{Texture, TextureFormat, TextureType}};

#[derive(Debug, Default)]
pub(crate) struct GlTexture {
    id: gl::types::GLuint,
}
impl GlTexture {
    pub(crate) unsafe fn new() -> Self {
        let mut id = 0;
        gl_check!(gl::GenTextures(1, &mut id));
        
        Self { id }
    }
    pub(crate) unsafe fn bind(&self) {
        gl_check!(gl::BindTexture(gl::TEXTURE_2D, self.id));
    }
    pub(crate) unsafe fn unbind(&self) {
        gl_check!(gl::BindTexture(gl::TEXTURE_2D, 0));
    }
    pub(crate) unsafe fn load(&self, texture: &impl Texture) {
        gl_check!(gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            tex_format_to_opengl(texture.texture_format()) as i32,
            texture.width() as i32, 
            texture.height() as i32, 
            0, 
            tex_format_to_opengl(texture.texture_format()), 
            tex_type_to_opengl(texture.texture_type()), 
            texture.bytes().as_ptr() as *const _,
        ));

        gl_check!(gl::GenerateMipmap(gl::TEXTURE_2D));
        gl_check!(gl::GenerateTextureMipmap(self.id));
    }
}
impl Drop for GlTexture {
    fn drop(&mut self) {
        unsafe { gl_check!(gl::DeleteTextures(1, [self.id].as_ptr())); }
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