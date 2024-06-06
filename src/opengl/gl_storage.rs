use std::{collections::HashMap, hash::Hash};
use crate::{renderer::{lg_shader::LgShader, lg_texture::LgTexture, lg_uniform::LgUniform}, StdError};
use super::{gl_buffer::GlBuffer, gl_program::GlProgram, gl_shader::GlShader, gl_texture::GlTexture, gl_vertex_array::GlVertexArray};

#[derive(Default)]
pub(crate) struct GlStorage<K: Eq + PartialEq + Hash> {
    pub(crate) buffers: HashMap<K, GlBuffer>,
    pub(crate) textures: HashMap<K, GlTexture>,
    shaders: HashMap<K, GlShader>,

    pub(crate) vaos: HashMap<K, GlVertexArray>,
    pub(crate) programs: HashMap<K, GlProgram>,
}
impl<K: Clone + Eq + PartialEq + Hash> GlStorage<K> {
    pub(crate) unsafe fn set_vao(&mut self, key: K) -> bool {
        let mut present = true;
        self.vaos.entry(key).or_insert_with(|| {
            present = false;
            GlVertexArray::new()
        });
        
        present
    }
    pub(crate) unsafe fn set_program<S: LgShader>(&mut self, key: K, shaders: &[(K, &S)]) -> Result<(), StdError> {
        let shaders = self.set_shaders(shaders)?;

        self.programs.entry(key).or_insert_with(|| {
            let mut program = GlProgram::new();
            program.set_shaders(shaders);
            program.link().unwrap();
            
            program
        });

        Ok(())
    }
    pub(crate) unsafe fn set_texture<T: LgTexture>(&mut self, key: K, texture: &T, location: u32) {
        self.textures.entry(key).or_insert_with(|| {
            let gl_tex = GlTexture::new();
            gl_tex.bind(location);
            gl_tex.load(texture);
            
            gl_tex
        });
    }
    pub(crate) unsafe fn set_uniforms(&mut self, uniforms: &[(K, &impl LgUniform)]) {
        for (key, ubo) in uniforms {
            self.buffers.entry(key.clone()).or_insert_with(|| {
                let usage = match ubo.u_type() {
                    crate::renderer::lg_uniform::LgUniformType::STRUCT => gl::UNIFORM_BUFFER,
                    crate::renderer::lg_uniform::LgUniformType::STORAGE_BUFFER => gl::SHADER_STORAGE_BUFFER,
                    crate::renderer::lg_uniform::LgUniformType::COMBINED_IMAGE_SAMPLER => gl::SAMPLER_2D,
                };
                
                let buffer = GlBuffer::new(usage);
                
                buffer.bind();
                buffer.bind_base(ubo.binding());
                buffer.set_data_full(
                    ubo.data_size(), 
                    ubo.get_raw_data(),
                    gl::STATIC_DRAW,
                );
                buffer.unbind();
                
                buffer
            });
        }
    }
    pub(crate) fn clear(&mut self) {
        self.buffers.clear();
        self.shaders.clear();
        self.textures.clear();
        self.vaos.clear();
        self.programs.clear();
    }
    
    unsafe fn set_shaders<S: LgShader>(&mut self, shaders: &[(K, &S)]) -> Result<Vec<gl::types::GLuint>, StdError> {
        let mut result = Vec::new();
        for s in shaders {
            let shader = self.shaders.entry(s.0.clone()).or_insert_with(|| {
                GlShader::new(s.1.src_code(), s.1.stage().to_gl_stage()).unwrap()
            });
            result.push(shader.id());
        }
        
        Ok(result)
    }
}