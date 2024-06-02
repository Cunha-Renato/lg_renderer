#[derive(Clone, Copy, Debug)]
pub enum LgUniformType {
    STRUCT,
    STORAGE_BUFFER,
    COMBINED_IMAGE_SAMPLER
}
pub trait LgUniform {
    fn name(&self) -> &str;
    fn u_type(&self) -> LgUniformType;
    fn binding(&self) -> usize;
    fn set(&self) -> usize;
    fn data_size(&self) -> usize;
    fn get_raw_data(&self) -> *const std::ffi::c_void;
    fn set_data<D>(&mut self, data: &D);
    fn update_data(&self) -> bool;
}