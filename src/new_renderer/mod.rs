extern crate glium;

use crate::StdError;
use nalgebra_glm as glm;

pub mod opengl;

pub struct GlRenderer {
    display: glium::Display<glutin::surface::WindowSurface>
}
impl GlRenderer {
    pub fn new(
        window_title: &str,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> (winit::window::Window, Self) 
    {
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(window_title)
            .build(event_loop);

        (window, Self { display })
    }
    pub fn draw<V: glium::Vertex>(
        &mut self,
        vertices: &[V],
        indices: &[u32],
        vert: &str,
        frag: &str,
    ) -> Result<(), StdError> 
    {
        let indices = glium::IndexBuffer::new(
            &self.display, 
            glium::index::PrimitiveType::TrianglesList, 
            indices
        )?;

        let vertex_buffer = glium::VertexBuffer::new(
            &self.display,
            vertices
        )?;
        
        let program = glium::Program::from_source(
            &self.display, 
            vert, 
            frag, 
            None
        )?;

        let test_uniform = glm::vec4(1.0f32, 0.0, 0.0, 1.0);
        let uniform = glium::uniforms::UniformsStorage::new("u_Color", test_uniform);

        Ok(())
    }
}