use std::{collections::HashMap, hash::Hash};
use crate::renderer_core::{lg_shader::LgShader, lg_texture::LgTexture, lg_uniform::LgUniform};
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
    pub(crate) fn set_vao(&mut self, key: K) -> bool {
        let mut present = true;
        self.vaos.entry(key).or_insert_with(|| {
            present = false;
            GlVertexArray::new().unwrap()
        });
        
        present
    }
    pub(crate) fn set_program<S: LgShader>(&mut self, key: K, shaders: &[(K, &S)]) {
        let shaders = self.set_shaders(shaders);

        self.programs.entry(key).or_insert_with(|| {
            let mut program = GlProgram::new().unwrap();
            program.set_shaders(shaders).unwrap();
            program.link().unwrap();
            
            program
        });
    }
    pub(crate) fn set_texture<T: LgTexture>(&mut self, key: K, texture: &T, location: u32) {
        self.textures.entry(key).or_insert_with(|| {
            let gl_tex = GlTexture::new().unwrap();
            gl_tex.bind(location).unwrap();
            gl_tex.load(texture).unwrap();
            
            gl_tex
        });
    }
    pub(crate) fn set_uniforms(&mut self, uniforms: &[(K, &impl LgUniform)]) {
        for (key, ubo) in uniforms {
            self.buffers.entry(key.clone()).or_insert_with(|| {
                let usage = match ubo.u_type() {
                    crate::renderer_core::lg_uniform::LgUniformType::STRUCT => gl::UNIFORM_BUFFER,
                    crate::renderer_core::lg_uniform::LgUniformType::STORAGE_BUFFER => gl::SHADER_STORAGE_BUFFER,
                    crate::renderer_core::lg_uniform::LgUniformType::COMBINED_IMAGE_SAMPLER => gl::SAMPLER_2D,
                };
                
                let buffer = GlBuffer::new(usage).unwrap();
                
                buffer.bind().unwrap();
                buffer.bind_base(ubo.binding()).unwrap();
                buffer.set_data_full(
                    ubo.data_size(), 
                    ubo.get_raw_data(),
                    gl::STATIC_DRAW,
                ).unwrap();
                buffer.unbind().unwrap();
                
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
    
    fn set_shaders<S: LgShader>(&mut self, shaders: &[(K, &S)]) -> Vec<gl::types::GLuint> {
        let mut result = Vec::new();
        for s in shaders {
            let shader = self.shaders.entry(s.0.clone()).or_insert_with(|| {
                GlShader::new(s.1.src_code(), s.1.stage().to_gl_stage()).unwrap()
            });
            result.push(shader.id());
        }
        
        result
    }
}