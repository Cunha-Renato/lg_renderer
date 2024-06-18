#![allow(non_camel_case_types)]

use std::hash::Hash;

use crate::{opengl::{gl_init::init_opengl, gl_renderer::GlRenderer}, StdError};
use self::{lg_shader::LgShader, lg_texture::LgTexture, lg_uniform::LgUniform, lg_vertex::GlVertex};

pub mod lg_vertex;
pub mod lg_texture;
pub mod lg_uniform;
pub mod lg_shader;

pub(crate) trait GraphicsApi {
    fn init(&mut self) -> Result<(), StdError>;
    fn shutdown(&mut self) -> Result<(), StdError>;
}

pub enum RendererAPI<K: Eq + PartialEq + Hash> {
    OPEN_GL(GlRenderer<K>),
    VULKAN(()),
}

pub enum CreationApiInfo {
    OPEN_GL,
    VULKAN,
}
pub struct CreationWindowInfo<'a> {
    pub event_loop: Option<&'a winit::event_loop::EventLoop<()>>,
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl<'a> CreationWindowInfo<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self { 
            event_loop: None, 
            title: title.to_string(), 
            width, 
            height 
        }
    }
}

pub struct LgRendererCreationInfo<'a> {
    pub renderer_api: CreationApiInfo,
    pub window_info: CreationWindowInfo<'a>,
}

pub struct LgRenderer<K: Clone +  Default + Eq + PartialEq + Hash> {
    api: RendererAPI<K>,
}
impl<K: Clone + Default + Eq + PartialEq + Hash> LgRenderer<K> {
    pub fn new(info: LgRendererCreationInfo) -> Result<(winit::window::Window, Self), StdError> {
        match &info.renderer_api {
            CreationApiInfo::OPEN_GL => {
                let (window, gl_specs) = init_opengl(info.window_info)?;
                
                Ok((window, Self {
                    api: RendererAPI::OPEN_GL(GlRenderer::new(gl_specs)?)
                }))
            },
            CreationApiInfo::VULKAN => todo!(),
        }
    }
    pub fn init(&mut self) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.init(),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
    pub fn shutdown(&mut self) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.shutdown(),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
    pub fn get_core(&self) -> &RendererAPI<K> {
        &self.api
    }

    pub fn set_vsync(&mut self, v_sync: bool) {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_vsync(v_sync),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
    pub fn is_vsync(&self) -> bool {
         match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.is_vsync(),
            RendererAPI::VULKAN(_) => todo!(),
        }       
    }

    pub fn begin(&self) -> Result<(), StdError> {
        Ok(match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.begin()?,
            RendererAPI::VULKAN(_) => todo!()
        })
    }
    pub fn end(&mut self) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.end()?,
            RendererAPI::VULKAN(_) => todo!()
        };
        
        Ok(())
    }
    pub fn resize(&self, new_size: (u32, u32)) -> Result<(), StdError> {
        match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.resize(new_size)?,
            RendererAPI::VULKAN(_) => todo!(),
        };
        
        Ok(())
    }
    pub unsafe fn read_uniform_buffer<T: Clone>(&self, key: K, index: usize) -> Result<T, StdError> {
        match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.read_buffer::<T>(key),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
    pub fn set_uniform_buffer_data(&self, key: K, data: &Vec<u8>) -> Result<(), StdError> {
        match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_buffer_data(key, data),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
}
impl<K: Clone + Default + Eq + PartialEq + Hash> LgRenderer<K> {
    pub fn set_program<S: LgShader>(&mut self, shaders: (K, &[(K, &S)])) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_program(shaders)?,
            RendererAPI::VULKAN(_) => todo!(),
        }
        
        Ok(())
    }

    pub fn set_vao(&mut self, id: K) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_vao(id)?,
            RendererAPI::VULKAN(_) => todo!(),
        }
        
        Ok(())
    }

    pub fn set_vertices<V: GlVertex>(&mut self, vertices: &[V]) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_vertices(vertices),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }

    pub fn set_indices(&mut self, indices: &[u32]) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_indices(indices),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }

    pub fn set_uniforms(&mut self, ubos: Vec<(K, &impl LgUniform)>) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_uniforms(ubos),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }

    pub fn set_textures<T: LgTexture>(&mut self, textures: &[(K, &T, u32)]) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.set_textures(textures),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
    pub fn draw(&mut self) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.draw(),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
    pub fn draw_instanced<V: GlVertex>(&mut self, instance_data: &[V]) -> Result<(), StdError> {
        match &mut self.api {
            RendererAPI::OPEN_GL(gl) => gl.draw_instanced(instance_data),
            RendererAPI::VULKAN(_) => todo!(),
        }
    }
}