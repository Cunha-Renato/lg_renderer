use std::{ffi::CString, hash::Hash};

use glutin::{display::GlDisplay, surface::GlSurface};
use crate::{gl_check, renderer::{lg_shader::LgShader, lg_texture::LgTexture, lg_uniform::LgUniform, lg_vertex::GlVertex}, StdError};
use super::{gl_storage::GlStorage, GlSpecs};

pub(crate) struct GlRenderer<K: Eq + PartialEq + Hash> {
    specs: GlSpecs,
    storage: GlStorage<K>,
}
impl<K: Eq + PartialEq + Hash + Default> GlRenderer<K> {
    pub(crate) fn new(specs: GlSpecs) -> Self {
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            specs.gl_display.get_proc_address(symbol.as_c_str()).cast()
        });
        
        unsafe {
            if cfg!(debug_assertions) {
                gl_check!(gl::Enable(gl::DEBUG_OUTPUT));
                gl_check!(gl::DebugMessageCallback(Some(super::debug_callback), std::ptr::null()));
            }
            
            gl_check!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
            gl_check!(gl::Enable(gl::BLEND));
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Self {
            specs,
            storage: GlStorage::default(),
        }
    }

    pub(crate) unsafe fn draw<V, T, S>(
        &mut self, 
        mesh: (K, &[V], &[u32]), 
        texture: Option<(K, &T)>,
        shaders: (K, &[(K, &S)]),
        ubos: Vec<(K, &impl LgUniform)>,
    ) -> Result<(), StdError>
    where 
        K: Clone,
        V: GlVertex,
        T: LgTexture,
        S: LgShader,
    {
        self.storage.set_vao(mesh.0.clone());
        self.storage.set_program(shaders.0.clone(), shaders.1)?;
        self.storage.set_uniforms(&ubos);
        if let Some(texture) = &texture {
            self.storage.set_texture(texture.0.clone(), texture.1)            
        }

        let vao = self.storage.vaos.get(&mesh.0).unwrap();
        let program = self.storage.programs.get(&shaders.0).unwrap();

        program.use_prog();
        vao.bind();
        vao.vertex_buffer().bind();

        let infos = V::gl_info();
        for info in infos {
            let location = program.get_attrib_location(&info.0)?;

            vao.set_attribute::<V>(location, info.1, 0);
        }

        vao.vertex_buffer().set_data(mesh.1, gl::STATIC_DRAW);
        vao.index_buffer().bind();
        vao.index_buffer().set_data(mesh.2, gl::STATIC_DRAW);

        for (key, uniform) in ubos {
            let ubo = self.storage.buffers.get(&key).unwrap();
            
            ubo.bind();
            ubo.bind_base(uniform.binding());
            if uniform.update_data() {
                ubo.set_data_full(
                    uniform.data_size(), 
                    uniform.get_raw_data(), 
                    gl::STATIC_DRAW
                );
            }
            ubo.unbind();
        }

        if let Some(texture) = texture {
            self.storage.textures.get(&texture.0).unwrap().bind();
        }

        gl_check!(gl::DrawElements(
            gl::TRIANGLES,
            mesh.2.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        ));

        vao.unbind();
        program.unuse();
            
        Ok(())
    }
    pub(crate) unsafe fn begin(&self) {
        gl_check!(gl::ClearColor(0.5, 0.1, 0.2, 1.0));
        gl_check!(gl::Clear(gl::COLOR_BUFFER_BIT));
    }
    pub(crate) unsafe fn end(&self) -> Result<(), StdError>{
        self.specs.gl_surface.swap_buffers(&self.specs.gl_context)?;
        
        Ok(())
    }
    pub(crate) unsafe fn read_buffer<T: Clone>(&self, key: K) -> Result<T, StdError> {
        gl_check!(gl::MemoryBarrier(gl::ALL_BARRIER_BITS));
        
        if let Some(buffer) = self.storage.buffers.get(&key) {
            buffer.bind();
            let data = buffer.map(gl::READ_ONLY) as *const T;
            let result = (*data).clone();
            buffer.unmap();
            buffer.unbind();
            
            return Ok(result);
        }
        
        Err("Couldn't find buffer! (OpenGL)".into())
    }
    pub(crate) unsafe fn set_buffer_data(&self, key: K, data: &Vec<u8>) -> Result<(), StdError> {
        gl_check!(gl::MemoryBarrier(gl::ALL_BARRIER_BITS));
        
        if let Some(buffer) = self.storage.buffers.get(&key) {
            let size = data.len() * std::mem::size_of::<u8>();
            let data = data.as_ptr() as *const std::ffi::c_void;

            buffer.bind();
            buffer.set_data_full(
                size, 
                data,
                gl::STATIC_DRAW
            );
            buffer.unbind();
            
            return Ok(());
        }
        
        Err("Couldn't find buffer! (OpenGL)".into())
    }
    pub(crate) unsafe fn resize(&self, new_size: (u32, u32)) -> Result<(), StdError> {
        self.specs.gl_surface.resize(
            &self.specs.gl_context, 
            std::num::NonZeroU32::new(new_size.0).unwrap(),
            std::num::NonZeroU32::new(new_size.1).unwrap(),
        );

        gl_check!(gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32));

        Ok(())
    }
}