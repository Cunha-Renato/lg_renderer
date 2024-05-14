use std::{collections::HashMap, hash::Hash};
use crate::{renderer::{lg_shader::Shader, lg_texture::Texture, lg_uniform::LgUniform}, StdError};
use super::{gl_buffer::GlBuffer, gl_program::GlProgram, gl_shader::GlShader, gl_texture::GlTexture, gl_vertex_array::GlVertexArray};

#[derive(Default)]
pub(crate) struct GlStorage<K: Eq + PartialEq + Hash> {
    pub(crate) buffers: HashMap<gl::types::GLuint, GlBuffer>,
    shaders: HashMap<K, GlShader>,

    pub(crate) vaos: HashMap<K, GlVertexArray>,
    pub(crate) programs: HashMap<K, GlProgram>,
    pub(crate) textures: HashMap<K, GlTexture>,
    pub(crate) ubos: HashMap<K, Vec<gl::types::GLuint>>,
}
impl<K: Clone + Eq + PartialEq + Hash> GlStorage<K> {
    pub(crate) unsafe fn set_vao(&mut self, key: K) {
        self.vaos.entry(key).or_insert(GlVertexArray::new());
    }
    pub(crate) unsafe fn set_program<S: Shader>(&mut self, key: K, shaders: &[(K, &S)]) -> Result<(), StdError> {
        let shaders = self.set_shaders(shaders)?;

        self.programs.entry(key).or_insert_with(|| {
            let mut program = GlProgram::new();
            program.set_shaders(shaders);
            program.link().unwrap();
            
            program
        });

        Ok(())
    }
    pub(crate) unsafe fn set_texture<T: Texture>(&mut self, key: K, texture: &T) {
        self.textures.entry(key).or_insert_with(|| {
            let gl_tex = GlTexture::new();
            gl_tex.bind();
            gl_tex.load(texture);
            
            gl_tex
        });
    }
    pub(crate) unsafe fn set_uniforms(&mut self, key: K, uniforms: &[LgUniform]) {
        // FIX_ME: Ugly ass code.
        self.ubos.entry(key).or_insert_with(|| {
            let mut buffer_ids = Vec::new();
            for uniform in uniforms {
                let usage = match uniform.u_type() {
                    crate::renderer::lg_uniform::LgUniformType::STRUCT => gl::UNIFORM_BUFFER,
                    crate::renderer::lg_uniform::LgUniformType::STORAGE_BUFFER => gl::SHADER_STORAGE_BUFFER,
                    crate::renderer::lg_uniform::LgUniformType::COMBINED_IMAGE_SAMPLER => gl::SAMPLER_2D,
                };

                let buffer = GlBuffer::new(usage);
                buffer_ids.push(buffer.id());
                
                buffer.bind();
                buffer.bind_base(uniform.binding());
                buffer.set_data_full(
                    uniform.data.size(), 
                    uniform.data(), 
                    gl::STATIC_DRAW
                );
                buffer.unbind();
                self.buffers.entry(buffer.id()).or_insert(buffer);
            }
                
            buffer_ids
        });
    }
    
    pub(crate) unsafe fn set_buffer(&mut self, buffer: GlBuffer) {
        self.buffers.entry(buffer.id()).or_insert(buffer);
    }
    unsafe fn set_shaders<S: Shader>(&mut self, shaders: &[(K, &S)]) -> Result<Vec<gl::types::GLuint>, StdError> {
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