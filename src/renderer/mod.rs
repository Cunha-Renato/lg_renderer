#![allow(non_camel_case_types)]

use std::hash::Hash;

use crate::{opengl::{gl_init::init_opengl, gl_renderer::GlRenderer}, StdError};
use self::{lg_shader::LgShader, lg_texture::LgTexture, lg_uniform::LgUniform, lg_vertex::GlVertex};

pub mod lg_vertex;
pub mod lg_texture;
pub mod lg_shader;
pub mod lg_uniform;

pub(crate) trait GraphicsApi {
    fn init(&mut self) -> Result<(), StdError>;
    fn shutdown(&mut self) -> Result<(), StdError>;
}

enum RendererAPI<K: Eq + PartialEq + Hash> {
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

    pub fn draw<V, T, S>(
        &mut self, 
        mesh: (K, &[V], &[u32]), 
        texture: Option<(K, &T)>,
        shaders: (K, &[(K, &S)]),
        ubos: Vec<(K, &impl LgUniform)>,
    ) -> Result<(), StdError>
    where 
        V: GlVertex,
        T: LgTexture,
        S: LgShader,
    {
        match &mut self.api {
            RendererAPI::OPEN_GL(api) => {
                api.draw(
                    mesh, 
                    texture,
                    shaders,
                    ubos,
                )?;
            },
            RendererAPI::VULKAN(_) => todo!(),
        }
        
        Ok(())
    }
    pub fn draw_instanced<V, I, T, S>(
        &mut self, 
        mesh: (K, &[V], &[u32]), 
        textures: &[(K, &T, u32)],
        shaders: (K, &[(K, &S)]),
        ubos: Vec<(K, &impl LgUniform)>,
        instance_data: &[I]
    ) -> Result<(), StdError>
    where 
        V: GlVertex,
        I: GlVertex,
        T: LgTexture,
        S: LgShader,
    {
        match &mut self.api {
            RendererAPI::OPEN_GL(api) => {
                api.draw_instanced(
                    mesh, 
                    textures,
                    shaders,
                    ubos,
                    instance_data,
                )?;
            },
            RendererAPI::VULKAN(_) => todo!(),
        }
        
        Ok(())
    }
    pub fn begin(&self) -> Result<(), StdError> {
        Ok(match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.begin()?,
            RendererAPI::VULKAN(_) => todo!()
        })
    }
    pub fn end(&self) -> Result<(), StdError> {
        match &self.api {
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