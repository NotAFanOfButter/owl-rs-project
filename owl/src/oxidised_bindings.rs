//! # Warning
//! ## Types
//! many u32's (and usizes) in this module are actually "u31's", by necessity of conversion to the (non-negative)
//! i32's requested by openGL.
use crate::safe_bindings;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OxError {
    BaseError(safe_bindings::Error),
    ShaderError(ShaderError)
}
impl From<safe_bindings::Error> for OxError {
    fn from(value: safe_bindings::Error) -> Self {
        Self::BaseError(value)
    }
}
impl From<ShaderError> for OxError {
    fn from(value: ShaderError) -> Self {
        Self::ShaderError(value)
    }
}
impl std::fmt::Display for OxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BaseError(be) => write!(f, "{be}"),
            Self::ShaderError(se) => write!(f, "{se}")
        }
    }
}
impl std::error::Error for OxError {}

pub use safe_bindings::DataType;

pub use safe_bindings::ClearColour as clear_colour;
pub use safe_bindings::Clear as clear;
pub use safe_bindings::ClearFlags;

//
// Errors
//
fn last_error() -> Option<OxError> {
    let mut error = None;
    while let Some(e) = safe_bindings::GetError() {
        error = Some(e.into())
    }
    error
}
fn last_error_as_result() -> Result<(),OxError> {
    match last_error() {
        None => Ok(()),
        Some(e) => Err(e)
    }
}

//
// Buffers
//
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Buffer(u32);

pub fn gen_buffers(count: usize) -> Vec<Buffer> {
    let mut buffer_ids = vec![0; count];
    safe_bindings::GenBuffers(buffer_ids.as_mut_slice());
    buffer_ids.into_iter().map(Buffer).collect()
}
pub fn gen_buffer() -> Buffer {
    let mut buffer_id = 0;
    safe_bindings::GenBuffer(&mut buffer_id);
    Buffer(buffer_id)
}
pub fn delete_buffers(buffers: Vec<Buffer>) {
    let buffer_ids: Vec<u32> = buffers.iter().map(|b| b.0).collect();
    safe_bindings::DeleteBuffers(buffer_ids.as_slice());
}
pub fn delete_buffer(buffer: Buffer) {
    safe_bindings::DeleteBuffer(buffer.0)
}

pub use safe_bindings::BufferType;
/// Bind target 0 (no bound buffer) if provided "None"
/// # Errors
/// Invalid Value: buffer was deleted
pub fn bind_buffer(target: BufferType, buffer: Option<Buffer>) -> Result<(),OxError> {
    safe_bindings::BindBuffer(target, buffer.map_or(0, |b| b.0));
    last_error_as_result()
}
pub use safe_bindings::BufferUsage;
use crate::traits::ToByteVec;
/// # Errors
/// `GL_INVALID_OPERATON`: `GL_BUFFER_IMMUTABLE_STORAGE` flag of target set to `GL_TRUE`
/// `GL_OUT_OF_MEMORY`
pub fn buffer_data<T>(target: BufferType, data: Vec<T>, usage: BufferUsage) -> Result<(),OxError> 
    where T: ToByteVec {
    safe_bindings::BufferData(target, data.to_byte_vec().as_slice(), usage);
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_OPERATON`: zero is bound to target, target is being mapped
/// `GL_INVALID_VALUE`: offset + size > buffer size
/// 
/// # Notes
/// offset in multiples of T
pub fn buffer_subdata<T>(target: BufferType, subdata: Vec<T>, offset: usize) -> Result<(),OxError>
    where T: ToByteVec {
    let byte_vec = subdata.to_byte_vec();
    safe_bindings::BufferSubData(target, byte_vec.as_slice(), offset * std::mem::size_of_val(&byte_vec));
    last_error_as_result()
}


//
// Vertex Array Objects
//
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VertexArray(u32);
pub fn gen_vertex_arrays(count: usize) -> Vec<VertexArray> {
    let mut va_ids = vec![0; count];
    safe_bindings::GenVertexArrays(va_ids.as_mut_slice());
    va_ids.into_iter().map(VertexArray).collect()
}
pub fn gen_vertex_array() -> VertexArray {
    let mut id = 0;
    safe_bindings::GenVertexArray(&mut id);
    VertexArray(id)
}
pub fn delete_vertex_arrays(vertex_arrays: Vec<VertexArray>) {
    let ids: Vec<u32> = vertex_arrays.iter().map(|v| v.0).collect();
    safe_bindings::DeleteVertexArrays(ids.as_slice());
}
pub fn delete_vertex_array(vertex_array: VertexArray) {
    safe_bindings::DeleteVertexArray(vertex_array.0);
}

/// # Errors
/// `GL_INVALID_VALUE`: vertex_array was deleted
pub fn bind_vertex_array(vertex_array: Option<VertexArray>) -> Result<(),OxError>{
    safe_bindings::BindVertexArray(vertex_array.map_or(0, |va| va.0));
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_OPERATON`: no vertex array object is bound
/// `GL_INVALID_VALUE`: attribute_index >= `GL_MAX_VERTEX_ATTRIBS`
pub fn enable_vertex_attrib_array(attribute_index: u8) -> Result<(),OxError> {
    safe_bindings::EnableVertexAttribArray(attribute_index);
    last_error_as_result()
}

pub use safe_bindings::AttribSize;
/// Subenum of DataType
pub enum DataTypeUnsized {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Fixed
}
impl From<DataTypeUnsized> for DataType {
    fn from(val: DataTypeUnsized) -> Self {
        match val {
            DataTypeUnsized::Byte => DataType::Byte,
            DataTypeUnsized::UnsignedByte => DataType::UnsignedByte,
            DataTypeUnsized::Short => DataType::Short,
            DataTypeUnsized::UnsignedShort => DataType::UnsignedShort,
            DataTypeUnsized::Int => DataType::Int,
            DataTypeUnsized::UnsignedInt => DataType::UnsignedInt,
            DataTypeUnsized::HalfFloat => DataType::HalfFloat,
            DataTypeUnsized::Float => DataType::Float,
            DataTypeUnsized::Double => DataType::Double,
            DataTypeUnsized::Fixed => DataType::Fixed
        }
    }
}
/// Subenum of DataType
pub enum DataTypeSize3 {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Fixed,
    UnsignedInt10f11f11fRev
}

impl From<DataTypeSize3> for DataType {
    fn from(val: DataTypeSize3) -> Self {
        match val {
            DataTypeSize3::Byte => DataType::Byte,
            DataTypeSize3::UnsignedByte => DataType::UnsignedByte,
            DataTypeSize3::Short => DataType::Short,
            DataTypeSize3::UnsignedShort => DataType::UnsignedShort,
            DataTypeSize3::Int => DataType::Int,
            DataTypeSize3::UnsignedInt => DataType::UnsignedInt,
            DataTypeSize3::HalfFloat => DataType::HalfFloat,
            DataTypeSize3::Float => DataType::Float,
            DataTypeSize3::Double => DataType::Double,
            DataTypeSize3::Fixed => DataType::Fixed,
            DataTypeSize3::UnsignedInt10f11f11fRev => DataType::UnsignedInt10f11f11fRev
        }
    }
}
/// Subenum of DataType
pub enum DataTypeSize4 {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Fixed,
    Int2_10_10_10Rev,
    UnsignedInt2_10_10_10Rev
}
impl From<DataTypeSize4> for DataType {
    fn from(val: DataTypeSize4) -> Self {
        match val {
            DataTypeSize4::Byte => DataType::Byte,
            DataTypeSize4::UnsignedByte => DataType::UnsignedByte,
            DataTypeSize4::Short => DataType::Short,
            DataTypeSize4::UnsignedShort => DataType::UnsignedShort,
            DataTypeSize4::Int => DataType::Int,
            DataTypeSize4::UnsignedInt => DataType::UnsignedInt,
            DataTypeSize4::HalfFloat => DataType::HalfFloat,
            DataTypeSize4::Float => DataType::Float,
            DataTypeSize4::Double => DataType::Double,
            DataTypeSize4::Fixed => DataType::Fixed,
            DataTypeSize4::Int2_10_10_10Rev => DataType::Int2_10_10_10Rev,
            DataTypeSize4::UnsignedInt2_10_10_10Rev => DataType::UnsignedInt2_10_10_10Rev,
        }
    }
}
/// Subenum of DataType
pub enum DataTypeSizeBgra {
    UnsignedByte,
    Int2_10_10_10Rev,
    UnsignedInt2_10_10_10Rev
}
impl From<DataTypeSizeBgra> for DataType {
    fn from(val: DataTypeSizeBgra) -> Self {
        match val {
            DataTypeSizeBgra::UnsignedByte => DataType::UnsignedByte,
            DataTypeSizeBgra::Int2_10_10_10Rev => DataType::Int2_10_10_10Rev,
            DataTypeSizeBgra::UnsignedInt2_10_10_10Rev => DataType::UnsignedInt2_10_10_10Rev,
        }
    }
}
pub enum VertexFormat {
    Size1 { normalised: bool, data_type: DataTypeUnsized },
    Size2 { normalised: bool, data_type: DataTypeUnsized },
    Size3 { normalised: bool, data_type: DataTypeSize3 },
    Size4 { normalised: bool, data_type: DataTypeSize4 },
    /// BGRA data types never normalised
    SizeBgra(DataTypeSizeBgra),
}
/// # Notes
/// When specifying a vertex attribute pointer, only certain combinations of size, type, and
/// normalisation are accepted.
/// size BGRA can only be used with types: UnsignedByte, Int2_10_10Rev, UnsignedInt2_10_10Rev
/// type UnsignedInt10f_11f_11fRev can only be used with size 3
/// types Int2_10_10Rev and UnsignedInt2_10_10Rev can only be used with size 4 or BGRA
/// size BGRA can only be used with normalised: false
/// # Errors
/// `GL_INVALID_VALUE`: index >= GL_MAX_VERTEX_ATTRIBS
/// `GL_INVALID_OPERATON`: array buffer bound to 0, offset != 0
pub fn vertex_attrib_pointer(attribute_index: u8, spec: VertexFormat,
    stride: usize, offset: usize) -> Result<(),OxError> {
    let (size, data_type, normalised) = match spec {
        VertexFormat::Size1 { normalised, data_type } => (AttribSize::One, data_type.into(), normalised),
        VertexFormat::Size2 { normalised, data_type } => (AttribSize::Two, data_type.into(), normalised),
        VertexFormat::Size3 { normalised, data_type } => (AttribSize::Three, data_type.into(), normalised),
        VertexFormat::Size4 { normalised, data_type } => (AttribSize::Four, data_type.into(), normalised),
        VertexFormat::SizeBgra(data_type) => (AttribSize::Bgra, data_type.into(), false)
    };
    safe_bindings::VertexAttribPointer(attribute_index, size, data_type, normalised, stride, offset);
    last_error_as_result()
}

//
// get*
//
pub use safe_bindings::Parameter as ParameterQuery;
pub enum UIntParameter {
    MaxVertexAttribs,
    ArrayBufferBinding,
    MaxComputeShaderStorageBlocks,
}
pub fn get_uint(parameter: UIntParameter) -> u32 {
    let mut data = [0];
    let parameter = match parameter {
        UIntParameter::MaxVertexAttribs => safe_bindings::Parameter::MaxVertexAttribs,
        UIntParameter::ArrayBufferBinding => safe_bindings::Parameter::ArrayBufferBinding,
        UIntParameter::MaxComputeShaderStorageBlocks => safe_bindings::Parameter::MaxComputeShaderStorageBlocks,
    };
    unsafe {
        // SAFETY: only parameters that are single values may be used (constrained by UintParameter),
        // so data must always be of length one.
        safe_bindings::GetIntegerv(parameter, &mut data);
    }
    data[0] as u32
}

//
// Shaders
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShaderError {
    CompilationFailed { info_log: String },
    CreationFailed,
    ProgramCreationFailed,
    LinkingFailed { info_log: String },
}
impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderError::CompilationFailed { info_log } => write!(f, "shader compilation failed\n{info_log}"),
            ShaderError::CreationFailed => write!(f, "shader creation failed"),
            ShaderError::ProgramCreationFailed => write!(f, "shader program creation failed"),
            ShaderError::LinkingFailed { info_log } => write!(f, "shader program linking failed\n{info_log}"),
        }
    }
}
impl std::error::Error for ShaderError {}

pub use safe_bindings::ShaderType;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Shader(u32);
pub fn create_shader(shader_type: ShaderType) -> Result<Shader,OxError> {
    match safe_bindings::CreateShader(shader_type) {
        0 => Err(ShaderError::CreationFailed.into()),
        id => Ok(Shader(id))
    }
}
/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn delete_shader(shader: Shader) -> Result<(),OxError> {
    safe_bindings::DeleteShader(shader.0);
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn shader_source<CS>(shader: Shader, sources: &[CS]) -> Result<(),OxError>
    where CS: AsRef<std::ffi::CStr> {
    safe_bindings::ShaderSource(shader.0, sources);
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
/// `CompilationFailed`
pub fn compile_shader(shader: Shader) -> Result<(),OxError> {
    safe_bindings::CompileShader(shader.0);
    last_error_as_result()?;
    match get_shader_compile_status(shader).expect("shader valid as it compiled") {
        ShaderCompileStatus::Succeeded => Ok(()),
        ShaderCompileStatus::Failed => Err(ShaderError::CompilationFailed {
            info_log: get_shader_info_log(shader).expect("shader guaranteed to be valid, as it compiled")
        }.into()),
    }
}

#[derive(Copy, Clone)]
pub enum ShaderCompileStatus {
    Succeeded,
    Failed
}
/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn get_shader_compile_status(shader: Shader) -> Result<ShaderCompileStatus, OxError> {
    let mut data = 0;
    safe_bindings::GetShaderiv(shader.0, safe_bindings::ShaderParameter::CompileStatus, &mut data);
    last_error_as_result()?;
    match data as u8 {
        safe_bindings::glTrue => Ok(ShaderCompileStatus::Succeeded),
        safe_bindings::glFalse => Ok(ShaderCompileStatus::Failed),
        _ => unreachable!("glGetShaderiv with boolean parameter can only return a boolean")
    }
}

pub enum ShaderDeleteStatus {
    Valid,
    Deleted
}
/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn get_shader_delete_status(shader: Shader) -> Result<ShaderDeleteStatus, OxError> {
    let mut data = 0;
    safe_bindings::GetShaderiv(shader.0, safe_bindings::ShaderParameter::DeleteStatus, &mut data);
    last_error_as_result()?;
    match data as u8 {
        safe_bindings::glTrue => Ok(ShaderDeleteStatus::Deleted),
        safe_bindings::glFalse => Ok(ShaderDeleteStatus::Valid),
        _ => unreachable!("glGetShaderiv with boolean parameter can only return a boolean")
    }
}
/// # Errors
/// `GL_INVALID_OPERATION`: shader has been deleted
pub fn get_shader_info_log_length(shader: Shader) -> Result<usize, OxError> {
    let mut data = 0;
    safe_bindings::GetShaderiv(shader.0, safe_bindings::ShaderParameter::InfoLogLength, &mut data);
    last_error_as_result()?;
    Ok(data as usize)
}
/// # Errors
/// `GL_INVALID_OPERATION`: shader has been deleted
pub fn get_shader_info_log(shader: Shader) -> Result<String, OxError> {
    let mut buffer = vec![0; get_shader_info_log_length(shader)?];
    safe_bindings::GetShaderInfoLog(shader.0, buffer.as_mut_slice(), None);
    // shader valid as it successfully got the length
    let utf8_buffer: Vec<u8> = buffer.iter().take(buffer.len()-1).map(|&i| i as u8).collect();
    Ok(String::from_utf8_lossy(&utf8_buffer).to_string())
}

//
// Shader Programs
//
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShaderProgram(u32);
pub fn create_program() -> Result<ShaderProgram, OxError> {
    match safe_bindings::CreateProgram() {
        0 => Err(ShaderError::ProgramCreationFailed.into()),
        p => Ok(ShaderProgram(p))
    }
}
/// # Errors:
/// `GL_INVALID_VALUE`: program was deleted
pub fn delete_program(program: ShaderProgram) -> Result<(),OxError> {
    safe_bindings::DeleteProgram(program.0);
    last_error_as_result()
}

/// # Errors:
/// `GL_INVALID_VALUE`: program was deleted
pub fn link_program(program: ShaderProgram) -> Result<(),OxError> {
    safe_bindings::LinkProgram(program.0);
    last_error_as_result()?;
    match get_program_link_status(program).expect("program linked, so must be valid") {
        LinkStatus::Succeeded => Ok(()),
        LinkStatus::Failed => Err(ShaderError::LinkingFailed {
            info_log: get_program_info_log(program).expect("program linked, so must be valid")
        }.into())
    }
}

#[derive(Copy, Clone)]
pub enum LinkStatus {
    Succeeded,
    Failed
}
/// # Errors
/// `GL_INVALID_OPERATION`: program deleted
pub fn get_program_link_status(program: ShaderProgram) -> Result<LinkStatus, OxError> {
    let mut link_success = 0;
    safe_bindings::GetProgramiv(program.0, safe_bindings::ProgramParameter::LinkStatus, &mut link_success);
    last_error_as_result()?;
    match link_success as u8 {
        safe_bindings::glTrue => Ok(LinkStatus::Succeeded),
        safe_bindings::glFalse => Ok(LinkStatus::Failed),
        _ => unreachable!("link_success is (in practice) a boolean")
    }
}
/// # Errors
/// `GL_INVALID_OPERATION`: program was deleted
pub fn get_program_info_log_length(program: ShaderProgram) -> Result<usize, OxError> {
    let mut data = 0;
    safe_bindings::GetProgramiv(program.0, safe_bindings::ProgramParameter::InfoLogLength, &mut data);
    last_error_as_result()?;
    Ok(data as usize)
}

/// # Errors
/// `GL_INVALID_OPERATION`: program was deleted
pub fn get_program_info_log(program: ShaderProgram) -> Result<String, OxError> {
    let mut buffer = vec![0; get_program_info_log_length(program)?];
    safe_bindings::GetProgramInfoLog(program.0, buffer.as_mut_slice(), None);
    // program valid as it got length -> no need for error checking
    let utf8_buffer: Vec<u8> = buffer.iter().take(buffer.len()-1).map(|&i| i as u8).collect();
    Ok(String::from_utf8_lossy(&utf8_buffer).to_string())
}

/// # Errors
/// `GL_INVALID_VALUE`: shader, program deleted
/// `GL_INVALID_OPERATON`: shader is already attached to program
pub fn attach_shader(program: ShaderProgram, shader: Shader) -> Result<(), OxError> {
    safe_bindings::AttachShader(program.0, shader.0);
    last_error_as_result()
}
/// # Errors
/// `GL_INVALID_VALUE`: shader, program deleted
/// `GL_INVALID_OPERATON`: shader is not attached to program
pub fn detach_shader(program: ShaderProgram, shader: Shader) -> Result<(), OxError> {
    safe_bindings::DetachShader(program.0, shader.0);
    last_error_as_result()
}
/// # Errors
/// `GL_INVALID_VALUE`: program deleted
/// `GL_INVALID_OPERATON`: transform feedback mode is active
pub fn use_program(program: ShaderProgram) -> Result<(),OxError> {
    safe_bindings::UseProgram(program.0);
    last_error_as_result()
}

//
// Drawing
//
pub use safe_bindings::{DrawMode, IndexType};

/// # Errors
/// `GL_INVALID_OPERATON`: a geometry shader is active and mode is incompatible with the input primitive type 
///                         of the geometry shader in the currently installed program object.
/// `GL_INVALID_OPERATON`: non-zero buffer object name is bound to an enabled array or the element array
///                         and the buffer object's data store is currently mapped
pub fn draw_elements(mode: DrawMode, count: usize, index_type: IndexType, offset: usize) -> Result<(),OxError> {
    safe_bindings::DrawElements(mode, count, index_type, offset);
    last_error_as_result()
}
/// # Errors
/// `GL_INVALID_OPERATON`: a geometry shader is active and mode is incompatible with the input primitive type 
///                         of the geometry shader in the currently installed program object.
/// `GL_INVALID_OPERATON`: non-zero buffer object name is bound to an enabled array or the element array
///                         and the buffer object's data store is currently mapped
pub fn draw_arrays(mode: DrawMode, first: usize, count: usize) -> Result<(),OxError> {
    safe_bindings::DrawArrays(mode, first, count);
    last_error_as_result()
}
