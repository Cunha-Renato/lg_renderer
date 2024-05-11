use std::{collections::{hash_map::Entry, HashMap}, hash::Hash};

use crate::{renderer::{lg_shader::Shader, lg_texture::Texture, lg_uniform::LgUniform}, StdError};

use super::{gl_buffer::GlBuffer, gl_program::GlProgram, gl_shader::GlShader, gl_texture::GlTexture, gl_vertex_array::GlVertexArray};

#[derive(Default)]
pub(crate) struct GlStorage<K: Eq + PartialEq + Hash> {
    buffers: HashMap<gl::types::GLuint, GlBuffer>,
    shaders: HashMap<gl::types::GLuint, GlShader>,

    pub(crate) vaos: HashMap<K, GlVertexArray>,
    pub(crate) programs: HashMap<K, GlProgram>,
    pub(crate) textures: HashMap<K, GlTexture>,
    pub(crate) ubos: HashMap<K, gl::types::GLuint>,
}
impl<K: Eq + PartialEq + Hash> GlStorage<K> {
    pub(crate) unsafe fn set_vao(&mut self, key: K) {
        self.vaos.entry(key).or_insert(GlVertexArray::new());
    }
    pub(crate) unsafe fn set_program<S: Shader>(&mut self, key: K, shaders: &[&S]) -> Result<(), StdError> {
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
    pub(crate) unsafe fn set_uniform(&mut self, key: K, uniform: &LgUniform) -> &GlBuffer {
        let usage = match uniform.u_type() {
            crate::renderer::lg_uniform::LgUniformType::STRUCT => gl::UNIFORM_BUFFER,
            crate::renderer::lg_uniform::LgUniformType::STORAGE_BUFFER => gl::SHADER_STORAGE_BUFFER,
            crate::renderer::lg_uniform::LgUniformType::COMBINED_IMAGE_SAMPLER => gl::SAMPLER_2D,
        };

        // FIX_ME: Ugly ass code.
        let ubo = self.ubos.entry(key).or_insert_with(|| {
            let buffer = GlBuffer::new(usage);
            let buffer_id = buffer.id();
            self.buffers.entry(buffer_id).or_insert(buffer);

            buffer_id
        });
        
        self.buffers.get(ubo).unwrap()
    }
    
    pub(crate) unsafe fn set_buffer(&mut self, buffer: GlBuffer) {
        self.buffers.entry(buffer.id()).or_insert(buffer);
    }
    unsafe fn set_shaders<S: Shader>(&mut self, shaders: &[&S]) -> Result<Vec<gl::types::GLuint>, StdError> {
        let mut result = Vec::new();
        for s in shaders {
            let shader = GlShader::new(
                s.src_code(), 
                s.stage().to_gl_stage()
            )?;

            result.push(shader.id());
            self.shaders.entry(shader.id()).or_insert(shader);
        }
        
        Ok(result)
    }
}