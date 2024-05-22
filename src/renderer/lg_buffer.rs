pub trait LgBuffer {
    fn data_size(&self) -> usize;
    fn get_raw_data(&self) -> *const std::ffi::c_void;
    fn set_data(&mut self, data: impl LgBufferData);
}
pub trait LgBufferData: 'static {
    fn size(&self) -> usize;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_c_void(&self) -> *const std::ffi::c_void {
        let ptr = self as *const Self;
        
        ptr as *const std::ffi::c_void
    }
}

#[macro_export]
macro_rules! impl_lg_buffer_data {
    ($struct_name:ident) => {
        impl lg_renderer::renderer::lg_buffer::LgBufferData for $struct_name {
            fn size(&self) -> usize {
                std::mem::size_of::<Self>()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}