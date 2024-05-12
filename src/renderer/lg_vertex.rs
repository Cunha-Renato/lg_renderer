pub trait GlVertex {
    /// Name, components, offset
    unsafe fn gl_info() -> Vec<(String, i32, i32)>;
}

#[macro_export]
macro_rules! lg_vertex {
    ($struct_name:ident, $($fields:tt), *) => {
        impl lg_renderer::renderer::lg_vertex::GlVertex for $struct_name {
            unsafe fn gl_info() -> Vec<(String, i32, i32)> {
                const fn size_of_raw<T>(_: *const T) -> usize {
                    core::mem::size_of::<T>()
                }
                let mut result = Vec::new();
                $(
                    let dummy = core::mem::MaybeUninit::<$struct_name>::uninit();
                    let dummy_ptr = dummy.as_ptr();
                    let member_ptr = core::ptr::addr_of!((*dummy_ptr).$fields);
                    let member_offset = member_ptr as i32 - dummy_ptr as i32;
                    
                    result.push((
                        String::from(stringify!($fields)),
                        (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
                        member_offset
                    ));
                )*
                
                result
            }
        }
    };
}