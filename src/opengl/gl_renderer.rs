use std::{ffi::CString, hash::Hash};

use glutin::{display::GlDisplay, surface::GlSurface};
use sllog::error;
use crate::{gl_check, renderer::{lg_shader::LgShader, lg_texture::LgTexture, lg_uniform::LgUniform, lg_vertex::GlVertex, GraphicsApi}, StdError};
use super::{gl_buffer::GlBuffer, gl_storage::GlStorage, GlError, GlSpecs};

struct RendererConfig {
    v_sync: bool,
}
pub(crate) struct GlRenderer<K: Eq + PartialEq + Hash> {
    instance_vbo: GlBuffer,
    storage: GlStorage<K>,
    specs: GlSpecs,
    config: RendererConfig,
}
impl<K: Eq + PartialEq + Hash> GlRenderer<K> {
    pub(crate) fn set_vsync(&mut self, v_sync: bool) {
        self.config.v_sync = v_sync;
        if v_sync {
            self.specs.gl_surface.set_swap_interval(&self.specs.gl_context, glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap())).unwrap();
        } else {
            self.specs.gl_surface.set_swap_interval(&self.specs.gl_context, glutin::surface::SwapInterval::DontWait).unwrap();
        }
    }
    pub(crate) fn is_vsync(&self) -> bool {
        self.config.v_sync
    }
}
impl<K: Eq + PartialEq + Hash + Default> GlRenderer<K> {
    pub(crate) fn new(specs: GlSpecs) -> Result<Self, GlError> {
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            specs.gl_display.get_proc_address(symbol.as_c_str()).cast()
        });
        
        Ok(Self {
            specs,
            config: RendererConfig { v_sync: true },
            storage: GlStorage::default(),
            instance_vbo: GlBuffer::new(gl::ARRAY_BUFFER)?,
        })
    }

    pub(crate) fn draw<V, T, S>(
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
        let vao_present = self.storage.set_vao(mesh.0.clone());
        self.storage.set_program(shaders.0.clone(), shaders.1);
        self.storage.set_uniforms(&ubos);
        if let Some(texture) = &texture {
            self.storage.set_texture(texture.0.clone(), texture.1, 0)            
        }

        let vao = self.storage.vaos.get(&mesh.0).ok_or("Failed to get VAO! (OpenGL)")?;
        let program = self.storage.programs.get(&shaders.0).ok_or("Failed to get Shader Program! (OpenGL)")?;

        program.use_prog()?;
        vao.bind()?;
        vao.index_buffer().bind()?;
        vao.vertex_buffer().bind()?;

        if !vao_present {
            vao.index_buffer().set_data(mesh.2, gl::STATIC_DRAW)?;

            let layout = unsafe { V::gl_info() };
            let stride = std::mem::size_of::<V>();

            vao.vertex_buffer().set_data(mesh.1, gl::STATIC_DRAW)?;
            for info in layout {
                vao.set_attribute(info.0, info.1, stride, info.2)?;
            }
        }

        for (key, uniform) in ubos {
            let ubo = self.storage.buffers.get(&key).ok_or("Failed to get UBO! (OpenGL)")?;
            
            ubo.bind()?;
            ubo.bind_base(uniform.binding())?;
            if uniform.update_data() {
                ubo.set_data_full(
                    uniform.data_size(), 
                    uniform.get_raw_data(), 
                    gl::STATIC_DRAW
                )?;
            }
            ubo.unbind()?;
        }

        if let Some(texture) = texture {
            self.storage.textures.get(&texture.0).ok_or("Failed to get Texture! (OpenGL)")?.bind(0)?;
        }

        gl_check!(gl::DrawElements(
            gl::TRIANGLES,
            mesh.2.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        ), "Failed to draw elements!")?;

        vao.vertex_buffer().unbind()?;        
        vao.index_buffer().unbind()?;
        vao.unbind()?;
        program.unuse()?;

        Ok(())
    }
    pub(crate) fn draw_instanced<V, I, T, S>(
        &mut self, 
        mesh: (K, &[V], &[u32]), 
        textures: &[(K, &T, u32)],
        shaders: (K, &[(K, &S)]),
        ubos: Vec<(K, &impl LgUniform)>,
        
        instance_data: &[I],
    ) -> Result<(), StdError>
    where 
        K: Clone,
        V: GlVertex,
        I: GlVertex,
        T: LgTexture,
        S: LgShader,
    {
        let vao_present = self.storage.set_vao(mesh.0.clone());
        self.storage.set_program(shaders.0.clone(), shaders.1);
        self.storage.set_uniforms(&ubos);
        
        for tex in textures {
            self.storage.set_texture(tex.0.clone(), tex.1, tex.2);
        }

        let vao = self.storage.vaos.get(&mesh.0).ok_or("Failed to get VAO! (OpenGL)")?;
        let program = self.storage.programs.get(&shaders.0).ok_or("Failed to get Shader Program! (OpenGL)")?;

        program.use_prog()?;
        vao.bind()?;

        let layout = unsafe { I::gl_info() };
        let stride = std::mem::size_of::<I>();
        let instance_count = instance_data.len();

        self.instance_vbo.bind()?;
        self.instance_vbo.set_data(instance_data, gl::STATIC_DRAW)?;

        let last_location = unsafe { V::gl_info().last().ok_or("Failed to get last location! (OpenGL)")?.0 };
        for info in layout {
            let location = info.0 + last_location + 1;
            vao.set_attribute(location, info.1, stride, info.2)?;
            
            gl_check!(gl::VertexAttribDivisor(location, 1), "Failed to set VertexAttribDivisor!")?;
        }

        vao.index_buffer().bind()?;
        vao.vertex_buffer().bind()?;

        if !vao_present {
            vao.index_buffer().set_data(mesh.2, gl::STATIC_DRAW)?;

            let layout = unsafe { V::gl_info() };
            let stride = std::mem::size_of::<V>();

            vao.vertex_buffer().set_data(mesh.1, gl::STATIC_DRAW)?;
            for info in layout {
                vao.set_attribute(info.0, info.1, stride, info.2)?;
            }
        }

        for (key, uniform) in ubos {
            let ubo = self.storage.buffers.get(&key).ok_or("Failed to get UBO! (OpenGL)")?;
            
            ubo.bind()?;
            ubo.bind_base(uniform.binding())?;
            if uniform.update_data() {
                ubo.set_data_full(
                    uniform.data_size(), 
                    uniform.get_raw_data(), 
                    gl::STATIC_DRAW
                )?;
            }
            ubo.unbind()?;
        }

        for tex in textures {
            self.storage.textures.get(&tex.0).ok_or("Failed to get Texture! (OpenGL)")?.bind(tex.2)?;
            gl_check!(gl::Uniform1i(tex.2 as i32, tex.2 as i32), "Failed to send Texture to Shader!")?;
        }

        gl_check!(
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                mesh.2.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                instance_count as i32,
            ),
            "Failed to draw Instanced!"
        )?;
        
        self.instance_vbo.unbind()?;
        vao.vertex_buffer().unbind()?;
        vao.index_buffer().unbind()?;
        vao.unbind()?;
        program.unuse()?;

        Ok(())
    }
    pub(crate) fn begin(&self) -> Result<(), GlError> {
        gl_check!(gl::ClearColor(0.5, 0.1, 0.2, 1.0), "Failed to ClearColor!")?;
        gl_check!(gl::ClearDepth(1.0), "Failed to ClearDepth!")?;
        gl_check!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT), "Failed to Clear!")
    }
    pub(crate) fn end(&self) -> Result<(), StdError>{
        self.specs.gl_surface.swap_buffers(&self.specs.gl_context)?;
        
        Ok(())
    }
    pub(crate) unsafe fn read_buffer<T: Clone>(&self, key: K) -> Result<T, StdError> {
        gl_check!(gl::MemoryBarrier(gl::ALL_BARRIER_BITS), "Failed to wait for barriers!")?;
        
        if let Some(buffer) = self.storage.buffers.get(&key) {
            buffer.bind()?;
            let data = buffer.map(gl::READ_ONLY)? as *const T;
            let result = (*data).clone();
            buffer.unmap()?;
            buffer.unbind()?;
            
            return Ok(result);
        }
        
        Err("Couldn't find buffer! (OpenGL)".into())
    }
    pub(crate) fn set_buffer_data(&self, key: K, data: &Vec<u8>) -> Result<(), StdError> {
        gl_check!(gl::MemoryBarrier(gl::ALL_BARRIER_BITS), "Failed to wait for barriers!")?;
        
        if let Some(buffer) = self.storage.buffers.get(&key) {
            let size = data.len() * std::mem::size_of::<u8>();
            let data = data.as_ptr() as *const std::ffi::c_void;

            buffer.bind()?;
            buffer.set_data_full(
                size, 
                data,
                gl::STATIC_DRAW
            )?;
            buffer.unbind()?;
            
            return Ok(());
        }
        
        Err("Couldn't find buffer! (OpenGL)".into())
    }
    pub(crate) fn resize(&self, new_size: (u32, u32)) -> Result<(), StdError> {
        self.specs.gl_surface.resize(
            &self.specs.gl_context, 
            std::num::NonZeroU32::new(new_size.0).unwrap(),
            std::num::NonZeroU32::new(new_size.1).unwrap(),
        );

        gl_check!(gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32), "Failed to set ViewPort on resize!")?;

        Ok(())
    }
}
impl<K: Eq + PartialEq + Hash + Default + Clone> GraphicsApi for GlRenderer<K> {
    fn init(&mut self) -> Result<(), StdError> {
        if true {
            gl_check!(gl::Enable(gl::DEBUG_OUTPUT), "Failed to enable gl::DEBUG_OUTPUT!")?;
            gl_check!(gl::DebugMessageCallback(Some(debug_callback), std::ptr::null()), "Failed to set DebugCallback")?;
        }
        
        gl_check!(gl::Enable(gl::DEPTH_TEST), "Failed to enable DepthTest!")?;
        gl_check!(gl::DepthFunc(gl::LESS), "Failed to set DepthFunc!")?;
        gl_check!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA), "Failed to set BlendFunc!")?;
        gl_check!(gl::Enable(gl::BLEND), "Failed to enable Blend!")?;
        
        gl_check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32), "Failed to set texture parameter!")?;
        gl_check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32), "Failed to set texture parameter!")?;
        gl_check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32), "Failed to set texture parameter!")?;
        gl_check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32), "Failed to set texture parameter!")?;

        self.set_vsync(self.config.v_sync);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), StdError> {
        self.instance_vbo.unbind()?;
        self.storage.clear();

        Ok(())
    }
}
impl<K: Eq + PartialEq + Hash> Drop for GlRenderer<K> {
    fn drop(&mut self) {
        unsafe { 
            loop {
                let err = gl::GetError();
                if err == gl::NO_ERROR {
                    break;
                }
                
                println!("OpenGL error {:08x}", err);
            }
        }
    }
}

extern "system" fn debug_callback(
    source: gl::types::GLenum,
    gltype: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    let source_str = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        _ => "Unknown",
    };

    let severity_str = match severity {
        gl::DEBUG_SEVERITY_HIGH => "High",
        gl::DEBUG_SEVERITY_MEDIUM => "Medium",
        gl::DEBUG_SEVERITY_LOW => "Low",
        gl::DEBUG_SEVERITY_NOTIFICATION => "Notification",
        _ => "Unknown",
    };

    let gltype_str = match gltype {
        gl::DEBUG_TYPE_ERROR => "Error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
        gl::DEBUG_TYPE_PORTABILITY => "Portability",
        gl::DEBUG_TYPE_PERFORMANCE => "Performance",
        gl::DEBUG_TYPE_OTHER => "Other",
        gl::DEBUG_TYPE_MARKER => "Marker",
        gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
        gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
        _ => "Unknown",
    };

    let message_str = unsafe { 
        std::str::from_utf8(std::ffi::CStr::from_ptr(message).to_bytes()).unwrap() 
    };
    error!(
        "OpenGL Debug Message:\n  Source: {}\n  Type: {}\n  ID: {}\n  Severity: {}\n  Message: {}",
        source_str, gltype_str, id, severity_str, message_str
    );
}