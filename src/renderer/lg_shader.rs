use crate::StdError;

#[derive(Debug, Clone, Copy)]
pub enum ShaderStage {
    VERTEX,
    FRAGMENT,
    COMPUTE,
}
impl ShaderStage {
    pub fn to_shaderc_stage(&self) -> Result<shaderc::ShaderKind, StdError> {
        match self {
            ShaderStage::VERTEX => Ok(shaderc::ShaderKind::Vertex),
            ShaderStage::FRAGMENT => Ok(shaderc::ShaderKind::Fragment),
            ShaderStage::COMPUTE => Err("Invalid ShaderStage! (Shader)".into()),
        }
    }
    pub(crate) fn to_gl_stage(&self) -> gl::types::GLenum {
        match self {
            ShaderStage::VERTEX => gl::VERTEX_SHADER,
            ShaderStage::FRAGMENT => gl::FRAGMENT_SHADER,
            ShaderStage::COMPUTE => gl::COMPUTE_SHADER,
        }
    }
}

pub trait LgShader {
    fn bytes(&self) -> &[u8];
    fn src_code(&self) -> &str;
    fn stage(&self) -> ShaderStage;
}