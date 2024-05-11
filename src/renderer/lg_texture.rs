#[derive(Debug, Clone, Copy)]
pub enum TextureType {
    UNSIGNED_BYTE,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    RGBA
}

pub trait Texture {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn bytes(&self) -> &[u8];
    fn size(&self) -> u64;
    fn mip_level(&self) -> u32;
    fn texture_type(&self) -> TextureType;
    fn texture_format(&self) -> TextureFormat;
}