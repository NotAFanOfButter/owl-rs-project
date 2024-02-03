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
pub use safe_bindings::IntegralDataType;

pub use safe_bindings::ClearColour as clear_colour;
pub use safe_bindings::Clear as clear;
pub use safe_bindings::ClearFlags;

//
// Errors
//
fn last_error() -> Option<OxError> {
    let mut error = None;
    while let Some(e) = safe_bindings::GetError() {
        error = Some(e.into());
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
    log::trace!("generated {count} buffers: {:?}", buffer_ids.as_slice());
    buffer_ids.into_iter().map(Buffer).collect()
}
#[allow(clippy::must_use_candidate)]    // basically constructor, if it's called, it will be used
pub fn gen_buffer() -> Buffer {
    let mut buffer_id = 0;
    safe_bindings::GenBuffer(&mut buffer_id);
    log::trace!("generated buffer: {buffer_id}");
    Buffer(buffer_id)
}
pub fn delete_buffers(buffers: Vec<Buffer>) {
    let buffer_ids: Vec<u32> = buffers.into_iter().map(|b| b.0).collect();
    safe_bindings::DeleteBuffers(buffer_ids.as_slice());
    log::trace!("deleted {} buffers: {:?}", buffer_ids.len(), buffer_ids);
}
pub fn delete_buffer(buffer: Buffer) {
    safe_bindings::DeleteBuffer(buffer.0);
    log::trace!("deleted buffer: {}", buffer.0);
}

pub use safe_bindings::BufferType;
/// Bind target 0 (no bound buffer) if provided "None"
/// # Errors
/// Invalid Value: buffer was deleted
pub fn bind_buffer(target: BufferType, buffer: Option<Buffer>) -> Result<(),OxError> {
    log::trace!("binding buffer: {buffer:?} to {target:?}");
    safe_bindings::BindBuffer(target, buffer.map_or(0, |b| b.0));
    last_error_as_result()
}
pub use safe_bindings::BufferUsage;
use crate::traits::ToByteVec;
/// # Errors
/// `GL_INVALID_OPERATON`: `GL_BUFFER_IMMUTABLE_STORAGE` flag of target set to `GL_TRUE`, no buffer bound
/// `GL_OUT_OF_MEMORY`
pub fn buffer_data<T>(target: BufferType, data: Vec<T>, usage: BufferUsage) -> Result<(),OxError> 
    where T: ToByteVec {
    log::trace!("buffering data of length {} to {target:?} for use {usage:?}", data.len());
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
    log::trace!("buffering subdata of length {} to {target:?} at offset {offset}", subdata.len());
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
    log::trace!("generated {count} vertex arrays: {:?}", va_ids.as_slice());
    va_ids.into_iter().map(VertexArray).collect()
}
#[allow(clippy::must_use_candidate)]    // basically constructor, if it's called, it will be used
pub fn gen_vertex_array() -> VertexArray {
    let mut id = 0;
    safe_bindings::GenVertexArray(&mut id);
    log::trace!("generated vertex array: {id}");
    VertexArray(id)
}
pub fn delete_vertex_arrays(vertex_arrays: Vec<VertexArray>) {
    let ids: Vec<u32> = vertex_arrays.into_iter().map(|v| v.0).collect();
    safe_bindings::DeleteVertexArrays(ids.as_slice());
    log::trace!("deleted {} vertex arrays: {:?}", ids.len(), ids);
}
pub fn delete_vertex_array(vertex_array: VertexArray) {
    safe_bindings::DeleteVertexArray(vertex_array.0);
    log::trace!("deleted vertex array: {}", vertex_array.0);
}

/// # Errors
/// `GL_INVALID_VALUE`: `vertex_array` was deleted
pub fn bind_vertex_array(vertex_array: Option<VertexArray>) -> Result<(),OxError>{
    log::trace!("binding vertex array: {vertex_array:?}");
    safe_bindings::BindVertexArray(vertex_array.map_or(0, |va| va.0));
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_OPERATON`: no vertex array object is bound
/// `GL_INVALID_VALUE`: `attribute_index` >= `GL_MAX_VERTEX_ATTRIBS`
pub fn enable_vertex_attrib_array(attribute_index: u8) -> Result<(),OxError> {
    log::trace!("enabling vertex attribute array {attribute_index}");
    safe_bindings::EnableVertexAttribArray(attribute_index);
    last_error_as_result()
}

pub use safe_bindings::AttribSize;
/// Subenum of [`DataType`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            DataTypeUnsized::Byte => Self::Byte,
            DataTypeUnsized::UnsignedByte => Self::UnsignedByte,
            DataTypeUnsized::Short => Self::Short,
            DataTypeUnsized::UnsignedShort => Self::UnsignedShort,
            DataTypeUnsized::Int => Self::Int,
            DataTypeUnsized::UnsignedInt => Self::UnsignedInt,
            DataTypeUnsized::HalfFloat => Self::HalfFloat,
            DataTypeUnsized::Float => Self::Float,
            DataTypeUnsized::Double => Self::Double,
            DataTypeUnsized::Fixed => Self::Fixed
        }
    }
}
/// Subenum of [`DataType`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            DataTypeSize3::Byte => Self::Byte,
            DataTypeSize3::UnsignedByte => Self::UnsignedByte,
            DataTypeSize3::Short => Self::Short,
            DataTypeSize3::UnsignedShort => Self::UnsignedShort,
            DataTypeSize3::Int => Self::Int,
            DataTypeSize3::UnsignedInt => Self::UnsignedInt,
            DataTypeSize3::HalfFloat => Self::HalfFloat,
            DataTypeSize3::Float => Self::Float,
            DataTypeSize3::Double => Self::Double,
            DataTypeSize3::Fixed => Self::Fixed,
            DataTypeSize3::UnsignedInt10f11f11fRev => Self::UnsignedInt10f11f11fRev
        }
    }
}
/// Subenum of [`DataType`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            DataTypeSize4::Byte => Self::Byte,
            DataTypeSize4::UnsignedByte => Self::UnsignedByte,
            DataTypeSize4::Short => Self::Short,
            DataTypeSize4::UnsignedShort => Self::UnsignedShort,
            DataTypeSize4::Int => Self::Int,
            DataTypeSize4::UnsignedInt => Self::UnsignedInt,
            DataTypeSize4::HalfFloat => Self::HalfFloat,
            DataTypeSize4::Float => Self::Float,
            DataTypeSize4::Double => Self::Double,
            DataTypeSize4::Fixed => Self::Fixed,
            DataTypeSize4::Int2_10_10_10Rev => Self::Int2_10_10_10Rev,
            DataTypeSize4::UnsignedInt2_10_10_10Rev => Self::UnsignedInt2_10_10_10Rev,
        }
    }
}
/// Subenum of [`DataType`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataTypeSizeBgra {
    UnsignedByte,
    Int2_10_10_10Rev,
    UnsignedInt2_10_10_10Rev
}
impl From<DataTypeSizeBgra> for DataType {
    fn from(val: DataTypeSizeBgra) -> Self {
        match val {
            DataTypeSizeBgra::UnsignedByte => Self::UnsignedByte,
            DataTypeSizeBgra::Int2_10_10_10Rev => Self::Int2_10_10_10Rev,
            DataTypeSizeBgra::UnsignedInt2_10_10_10Rev => Self::UnsignedInt2_10_10_10Rev,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatVertexFormat {
    Size1 { normalise: bool, data_type: DataTypeUnsized },
    Size2 { normalise: bool, data_type: DataTypeUnsized },
    Size3 { normalise: bool, data_type: DataTypeSize3 },
    Size4 { normalise: bool, data_type: DataTypeSize4 },
    /// BGRA data types never normalised
    SizeBgra(DataTypeSizeBgra),
}
/// # Notes
/// When specifying a vertex attribute pointer, only certain combinations of size, type, and
/// normalisation are accepted in `spec`.
/// size BGRA can only be used with types: `UnsignedByte`, `Int2_10_10Rev`, `UnsignedInt2_10_10Rev`
/// type `UnsignedInt10f_11f_11fRev` can only be used with size 3
/// types `Int2_10_10Rev` and `UnsignedInt2_10_10Rev` can only be used with size 4 or `BGRA`
/// size `BGRA `can only be used with `normalise`: false
/// # Errors
/// `GL_INVALID_VALUE`: index >= `GL_MAX_VERTEX_ATTRIBS`
/// `GL_INVALID_OPERATON`: array buffer bound to 0, offset != 0
pub fn vertex_attrib_pointer(attribute_index: u8, spec: FloatVertexFormat,
    stride: usize, offset: usize) -> Result<(),OxError> {
    log::trace!("registering pointer for float vertex attribute {attribute_index}, in format {spec:?}, with stride {stride}, at offset {offset}");
    let (size, data_type, normalise) = match spec {
        FloatVertexFormat::Size1 { normalise, data_type } => (AttribSize::One, data_type.into(), normalise),
        FloatVertexFormat::Size2 { normalise, data_type } => (AttribSize::Two, data_type.into(), normalise),
        FloatVertexFormat::Size3 { normalise, data_type } => (AttribSize::Three, data_type.into(), normalise),
        FloatVertexFormat::Size4 { normalise, data_type } => (AttribSize::Four, data_type.into(), normalise),
        FloatVertexFormat::SizeBgra(data_type) => (AttribSize::Bgra, data_type.into(), false)
    };
    safe_bindings::VertexAttribPointer(attribute_index, size, data_type, normalise, stride, offset);
    last_error_as_result()
}

pub use safe_bindings::IntegralAttribSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegralVertexFormat {
    Size1(IntegralDataType),
    Size2(IntegralDataType),
    Size3(IntegralDataType),
    Size4(IntegralDataType),
}

/// # Errors
/// `GL_INVALID_VALUE`: index >= `GL_MAX_VERTEX_ATTRIBS`
/// `GL_INVALID_OPERATON`: array buffer bound to 0, offset != 0
pub fn vertex_attrib_i_pointer(attribute_index: u8, spec: IntegralVertexFormat,
    stride: usize, offset: usize) -> Result<(),OxError> {
    log::trace!("registering pointer for integer vertex attribute {attribute_index}, in format {spec:?}, with stride {stride}, at offset {offset}");
    let (size, data_type) = match spec {
        IntegralVertexFormat::Size1(data_type) => (IntegralAttribSize::One, data_type),
        IntegralVertexFormat::Size2(data_type) => (IntegralAttribSize::Two, data_type),
        IntegralVertexFormat::Size3(data_type) => (IntegralAttribSize::Three, data_type),
        IntegralVertexFormat::Size4(data_type) => (IntegralAttribSize::Four, data_type),
    };
    safe_bindings::VertexAttribIPointer(attribute_index, size, data_type, stride, offset);
    last_error_as_result()
}

//
// get*
//
pub use safe_bindings::Parameter as ParameterQuery;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIntParameter {
    MaxVertexAttribs,
    ArrayBufferBinding,
    ElementBufferBinding,
    MaxComputeShaderStorageBlocks,
}
#[must_use]
pub fn get_uint(parameter: UIntParameter) -> u32 {
    let mut data = [0];
    let parameter = match parameter {
        UIntParameter::MaxVertexAttribs => safe_bindings::Parameter::MaxVertexAttribs,
        UIntParameter::ArrayBufferBinding => safe_bindings::Parameter::ArrayBufferBinding,
        UIntParameter::ElementBufferBinding => safe_bindings::Parameter::ElementBufferBinding,
        UIntParameter::MaxComputeShaderStorageBlocks => safe_bindings::Parameter::MaxComputeShaderStorageBlocks,
    };
    // SAFETY: only parameters that are single values may be used (constrained by UintParameter),
    // so data must always be of length one.
    unsafe {
        safe_bindings::GetIntegerv(parameter, &mut data);
    }
    log::trace!("got {parameter:?} = {}", data[0]);
    // CAST: only parameters corresponding to uints are allowed, so guaranteed positive
    #[allow(clippy::cast_sign_loss)]
    return data[0] as u32;
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
            Self::CompilationFailed { info_log } => write!(f, "shader compilation failed\n{info_log}"),
            Self::CreationFailed => write!(f, "shader creation failed"),
            Self::ProgramCreationFailed => write!(f, "shader program creation failed"),
            Self::LinkingFailed { info_log } => write!(f, "shader program linking failed\n{info_log}"),
        }
    }
}
impl std::error::Error for ShaderError {}

pub use safe_bindings::ShaderType;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Shader(u32);
/// # Errors
///
/// This function will return an error if shader creation fails, no documentation as to why this may happen.
pub fn create_shader(shader_type: ShaderType) -> Result<Shader,OxError> {
    log::trace!("creating {shader_type:?} shader");
    match safe_bindings::CreateShader(shader_type) {
        0 => {
            Err(ShaderError::CreationFailed.into())
        },
        id => {
            log::trace!("created {shader_type:?} shader: {id}");
            Ok(Shader(id))
        }
    }
}
/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn delete_shader(shader: Shader) -> Result<(),OxError> {
    log::trace!("deleting shader: {}", shader.0);
    safe_bindings::DeleteShader(shader.0);
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn shader_source<CS>(shader: Shader, sources: &[CS]) -> Result<(),OxError>
    where CS: AsRef<std::ffi::CStr> {
    log::trace!("adding source(s) to shader {}, sources: {:?}", shader.0, sources.iter().map(AsRef::as_ref).collect::<Vec<_>>());
    safe_bindings::ShaderSource(shader.0, sources);
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
/// `CompilationFailed`
pub fn compile_shader(shader: Shader) -> Result<(),OxError> {
    log::trace!("compiling shader: {}", shader.0);
    safe_bindings::CompileShader(shader.0);
    last_error_as_result()?;
    match get_shader_compile_status(shader).expect("shader valid as it compiled") {
        ShaderCompileStatus::Succeeded => Ok(()),
        ShaderCompileStatus::Failed => Err(ShaderError::CompilationFailed {
            info_log: get_shader_info_log(shader).expect("shader guaranteed to be valid, as it compiled")
        }.into()),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaderCompileStatus {
    Succeeded,
    Failed
}
/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn get_shader_compile_status(shader: Shader) -> Result<ShaderCompileStatus, OxError> {
    log::trace!("getting shader {} compile status", shader.0);
    let mut data = 0;
    safe_bindings::GetShaderiv(shader.0, safe_bindings::ShaderParameter::CompileStatus, &mut data);
    last_error_as_result()?;
    // CAST: `data` is really a bool
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    match data as u8 {
        safe_bindings::glTrue => Ok(ShaderCompileStatus::Succeeded),
        safe_bindings::glFalse => Ok(ShaderCompileStatus::Failed),
        _ => unreachable!("glGetShaderiv with boolean parameter can only return a boolean")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderDeleteStatus {
    Valid,
    Deleted
}
/// # Errors
/// `GL_INVALID_VALUE`: shader has been deleted
pub fn get_shader_delete_status(shader: Shader) -> Result<ShaderDeleteStatus, OxError> {
    let mut data = 0;
    safe_bindings::GetShaderiv(shader.0, safe_bindings::ShaderParameter::DeleteStatus, &mut data);
    log::trace!("got shader {} delete status = {data}", shader.0);
    last_error_as_result()?;
    // CAST: `data` is really a bool
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    match data as u8 {
        safe_bindings::glTrue => Ok(ShaderDeleteStatus::Deleted),
        safe_bindings::glFalse => Ok(ShaderDeleteStatus::Valid),
        _ => unreachable!("glGetShaderiv with boolean parameter can only return a boolean")
    }
}
/// # Errors
/// `GL_INVALID_OPERATION`: shader has been deleted
pub fn get_shader_info_log_length(shader: Shader) -> Result<usize, OxError> {
    log::trace!("getting shader {} log length", shader.0);
    let mut data = 0;
    safe_bindings::GetShaderiv(shader.0, safe_bindings::ShaderParameter::InfoLogLength, &mut data);
    last_error_as_result()?;
    // CAST: the log length is unsigned
    #[allow(clippy::cast_sign_loss)]
    Ok(data as usize)
}
/// # Errors
/// `GL_INVALID_OPERATION`: shader has been deleted
pub fn get_shader_info_log(shader: Shader) -> Result<String, OxError> {
    log::trace!("getting shader {} info log", shader.0);
    let mut buffer = vec![0; get_shader_info_log_length(shader)?];
    // shader valid as it successfully got the length -> no need for error checking
    safe_bindings::GetShaderInfoLog(shader.0, buffer.as_mut_slice(), None);
    // SAFETY: `buffer` is valid for length-1 reads, and properly aligned
    //         * it is contained in one allocated object
    //         * non-nul: c_str can only contain one nul - at the end (excluded here)
    //         always points to length - 1 values, guaranteed to be initialised above
    //         shadowed, so the memory cannot be mutated while the slice lives
    //         shader_info_log may be at most of length i32::MAX (get_shader_* relies on get_shader_iv, which returns an i32)
    let buffer = unsafe { std::slice::from_raw_parts(buffer.as_ptr().cast(), buffer.len()-1) };
    let utf8_buffer = String::from_utf8_lossy(buffer);
    Ok(utf8_buffer.to_string())
}

//
// Shader Programs
//
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShaderProgram(u32);
/// # Errors
///
/// This function will return an error if program creation fails, no documentation as to why this may happen.
pub fn create_program() -> Result<ShaderProgram, OxError> {
    log::trace!("creating shader program");
    match safe_bindings::CreateProgram() {
        0 => Err(ShaderError::ProgramCreationFailed.into()),
        p => {
            log::trace!("created shader program {p}");
            Ok(ShaderProgram(p))
        }
    }
}
/// # Errors
/// `GL_INVALID_VALUE`: program was deleted
pub fn delete_program(program: ShaderProgram) -> Result<(),OxError> {
    log::trace!("deleting shader program: {}", program.0);
    safe_bindings::DeleteProgram(program.0);
    last_error_as_result()
}

/// # Errors
/// `GL_INVALID_VALUE`: program was deleted
/// `GL_INVALID_OPERATION`: program is active, and transform feedback mode is active
pub fn link_program(program: ShaderProgram) -> Result<(),OxError> {
    log::trace!("linking shader program: {}", program.0);
    safe_bindings::LinkProgram(program.0);
    last_error_as_result()?;
    match get_program_link_status(program).expect("program linked, so must be valid") {
        LinkStatus::Succeeded => Ok(()),
        LinkStatus::Failed => Err(ShaderError::LinkingFailed {
            info_log: get_program_info_log(program).expect("program linked, so must be valid")
        }.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkStatus {
    Succeeded,
    Failed
}
/// # Errors
/// `GL_INVALID_OPERATION`: program deleted
pub fn get_program_link_status(program: ShaderProgram) -> Result<LinkStatus, OxError> {
    log::trace!("getting shader program {} link status", program.0);
    let mut link_success = 0;
    safe_bindings::GetProgramiv(program.0, safe_bindings::ProgramParameter::LinkStatus, &mut link_success);
    last_error_as_result()?;
    // CAST: `data` is really a bool
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    match link_success as u8 {
        safe_bindings::glTrue => Ok(LinkStatus::Succeeded),
        safe_bindings::glFalse => Ok(LinkStatus::Failed),
        _ => unreachable!("link_success is (in practice) a boolean")
    }
}
/// # Errors
/// `GL_INVALID_OPERATION`: program was deleted
pub fn get_program_info_log_length(program: ShaderProgram) -> Result<usize, OxError> {
    log::trace!("getting shader program {} info log", program.0);
    let mut data = 0;
    safe_bindings::GetProgramiv(program.0, safe_bindings::ProgramParameter::InfoLogLength, &mut data);
    last_error_as_result()?;
    // CAST: the log length is unsigned
    #[allow(clippy::cast_sign_loss)]
    Ok(data as usize)
}

/// # Errors
/// `GL_INVALID_OPERATION`: program was deleted
pub fn get_program_info_log(program: ShaderProgram) -> Result<String, OxError> {
    log::trace!("getting shader program {} info log", program.0);
    let mut buffer = vec![0; get_program_info_log_length(program)?];
    safe_bindings::GetProgramInfoLog(program.0, buffer.as_mut_slice(), None);
    // program valid as it got length -> no need for error checking
    if let Some(c) = buffer.last() {
        assert_eq!(*c, 0, "buffer ends with 0, if it exists at all");
        // SAFETY: nul-terminated, single object, non-null, not mutated during, ends within `isize::MAX bytes`
        let utf8_buffer = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy();
        Ok(utf8_buffer.to_string())
    } else {
        Ok(String::new())
    }
}

/// # Errors
/// `GL_INVALID_VALUE`: shader, program deleted
/// `GL_INVALID_OPERATON`: shader is already attached to program
pub fn attach_shader(program: ShaderProgram, shader: Shader) -> Result<(), OxError> {
    log::trace!("attaching shader {} to shader program {}", shader.0, program.0);
    safe_bindings::AttachShader(program.0, shader.0);
    last_error_as_result()
}
/// # Errors
/// `GL_INVALID_VALUE`: shader, program deleted
/// `GL_INVALID_OPERATON`: shader is not attached to program
pub fn detach_shader(program: ShaderProgram, shader: Shader) -> Result<(), OxError> {
    log::trace!("attaching shader program {} from shader program {}", shader.0, program.0);
    safe_bindings::DetachShader(program.0, shader.0);
    last_error_as_result()
}
/// # Errors
/// `GL_INVALID_VALUE`: program deleted
/// `GL_INVALID_OPERATON`: transform feedback mode is active
pub fn use_program(program: ShaderProgram) -> Result<(),OxError> {
    log::trace!("using shader program {}", program.0);
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
    log::trace!("drawing {count} vertices from elements of type {index_type:?} in mode {mode:?}, starting from {offset}");
    safe_bindings::DrawElements(mode, count, index_type, offset);
    last_error_as_result()
}
/// # Errors
/// `GL_INVALID_OPERATON`: a geometry shader is active and mode is incompatible with the input primitive type 
///                         of the geometry shader in the currently installed program object.
/// `GL_INVALID_OPERATON`: non-zero buffer object name is bound to an enabled array or the element array
///                         and the buffer object's data store is currently mapped
pub fn draw_arrays(mode: DrawMode, first: usize, count: usize) -> Result<(),OxError> {
    log::trace!("drawing {count} vertices from arrays in mode {mode:?}, starting from {first}");
    safe_bindings::DrawArrays(mode, first, count);
    last_error_as_result()
}
