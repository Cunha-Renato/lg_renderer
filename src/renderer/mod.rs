#![allow(non_camel_case_types)]

use std::hash::Hash;

use crate::{opengl::{gl_init::init_opengl, gl_renderer::GlRenderer}, StdError};
use self::{lg_shader::Shader, lg_texture::Texture, lg_uniform::LgUniform, lg_vertex::GlVertex};

pub mod lg_vertex;
pub mod lg_texture;
pub mod lg_shader;
pub mod lg_uniform;

enum RendererAPI<K: Eq + PartialEq + Hash> {
    OPEN_GL(GlRenderer<K>),
    VULKAN(()),
}

pub struct LgRenderer<K: Clone +  Default + Eq + PartialEq + Hash> {
    api: RendererAPI<K>,
}
impl<K: Clone + Default + Eq + PartialEq + Hash> LgRenderer<K> {
    pub fn new_opengl(
        event_loop: &winit::event_loop::EventLoop<()>, 
        window_builder: winit::window::WindowBuilder,
    ) -> Result<(winit::window::Window, Self), StdError>
    {
        let (window, specs) = init_opengl(event_loop, window_builder)?;

        Ok((window, Self {
            api: RendererAPI::OPEN_GL(GlRenderer::new(specs))
        }))
    }
    pub unsafe fn draw<V, T, S>(
        &mut self, 
        mesh: (K, &[V], &[u32]), 
        texture: Option<(K, &T)>,
        shaders: (K, &[(K, &S)]),
        ubos: Vec<(K, &[LgUniform])>,
    ) -> Result<(), StdError>
    where 
        V: GlVertex,
        T: Texture,
        S: Shader,
    {
        match &mut self.api {
            RendererAPI::OPEN_GL(api) => {
                api.draw(
                    mesh, 
                    texture,
                    shaders,
                    ubos
                )?;
            },
            RendererAPI::VULKAN(_) => todo!(),
        }
        
        Ok(())
    }
    pub unsafe fn begin(&self) {
        match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.begin(),
            RendererAPI::VULKAN(_) => todo!()
        }
    }
    pub unsafe fn end(&self) -> Result<(), StdError> {
        match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.end()?,
            RendererAPI::VULKAN(_) => todo!()
        };
        
        Ok(())
    }
    pub unsafe fn resize(&self, new_size: (u32, u32)) -> Result<(), StdError> {
        match &self.api {
            RendererAPI::OPEN_GL(gl) => gl.resize(new_size)?,
            RendererAPI::VULKAN(_) => todo!(),
        };
        
        Ok(())
    }
}