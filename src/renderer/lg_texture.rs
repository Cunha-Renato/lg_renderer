use crate::StdError;

#[derive(Debug, Clone, Copy)]
pub enum TextureType {
    UNSIGNED_BYTE,
}
impl TextureType {
    pub fn from(value: u32) -> Result<Self, StdError> {
        match value {
            0 => Ok(Self::UNSIGNED_BYTE),
            _ => Err("Failed to convert from u32! (TextureFormat)".into())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    RGBA
}
impl TextureFormat {
    pub fn from(value: u32) -> Result<Self, StdError> {
        match value {
            0 => Ok(Self::RGBA),
            _ => Err("Failed to convert from u32! (TextureFormat)".into())
        }
    }
}

pub trait LgTexture {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn bytes(&self) -> &[u8];
    fn size(&self) -> u64;
    fn mip_level(&self) -> u32;
    fn texture_type(&self) -> TextureType;
    fn texture_format(&self) -> TextureFormat;
}