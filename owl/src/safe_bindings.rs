//! # Warning
//! ## Types
//! many u32's (and usizes) in this module are actually "u31's", by necessity of conversion to the (non-negative)
//! i32's requested by openGL.
//! ## Creation and Deletion
//! where a function is documented as taking a "valid" id / name of an object, it is given that the
//! object has not previously been deleted.

#![allow(non_snake_case)]
#![allow(dead_code)]

use gl;
use bitflags::bitflags;

/// # Notes
/// values clamped to [0,1]
pub fn ClearColour(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha)
    }
}

bitflags! (
    #[derive(Copy, Clone)]
    pub struct ClearFlags: u32 {
        const ColourBuffer = 1;
        const StencilBuffer = 1 << 1;
        const DepthBuffer = 1 << 2;
    }
);

impl From<ClearFlags> for gl::types::GLbitfield {
    fn from(val: ClearFlags) -> Self {
        let mut bitfield = 0;
        if val.contains(ClearFlags::ColourBuffer) {
            bitfield |= gl::COLOR_BUFFER_BIT;
        }
        if val.contains(ClearFlags::StencilBuffer) {
            bitfield |= gl::STENCIL_BUFFER_BIT;
        }
        if val.contains(ClearFlags::DepthBuffer) {
            bitfield |= gl::DEPTH_BUFFER_BIT;
        }
        bitfield
    }
}

/// # GL Invariants
/// flag: only 3 buffer bits can be set
pub fn Clear(flags: ClearFlags) {
    unsafe {
        gl::Clear(flags.into())
    }
}

/// # GL Invariants
/// length of buffers >= 0
pub fn GenBuffers(buffers: &mut [u32]) {
    unsafe {
        // n: >= 0
        gl::GenBuffers(buffers.len() as i32, buffers.as_mut_ptr())
    }
}
/// Ease of use for GenBuffers
pub fn GenBuffer(buffer: &mut u32) {
    GenBuffers(std::slice::from_mut(buffer));
}

/// # GL Invariants
/// length of buffers >= 0
pub fn DeleteBuffers(buffers: &[u32]) {
    unsafe {
        // n: >= 0
        gl::DeleteBuffers(buffers.len() as i32, buffers.as_ptr())
    }
}
pub fn DeleteBuffer(buffer: u32) {
    DeleteBuffers(&[buffer])
}

/// # GL Invariants
/// target: is an accepted buffer type
///
/// # User Invariants
/// buffer: is a valid buffer returned by `glGenBuffers` or 0
///
/// # Errors
/// `GL_INVALID_VALUE`: buffer was not returned by `glGenBuffers`, 0, or was deleted
pub fn BindBuffer(target: BufferType, buffer: u32) {
    unsafe { gl::BindBuffer(target.into(), buffer) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferType {
    Array,
    AtomicCounter,
    CopyRead,
    CopyWrite,
    DispatchIndirect,
    DrawIndirect,
    ElementArray,
    PixelPack,
    PixelUnpack,
    Query,
    ShaderStorage,
    Texture,
    TransformFeedback,
    Uniform
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy
}
/// # GL Invariants
/// target: accepted buffer target (GLenum),
/// size >= 0,
/// usage: accepted buffer usage (GLenum)
///
/// # User Invariants
/// `GL_BUFFER_IMMUTABLE_STORAGE` flag of target must be `GL_FALSE`,
/// A buffer must be bound
///
/// # Errors
/// `GL_INVALID_OPERATON`: `GL_BUFFER_IMMUTABLE_STORAGE` flag of target set to `GL_TRUE`, no buffer bound
/// `GL_OUT_OF_MEMORY`
pub fn BufferData<T>(target: BufferType, data: &[T], usage: BufferUsage) {
    unsafe {
        gl::BufferData(target.into(), std::mem::size_of_val(data) as isize, data.as_ptr().cast(), usage.into())
    }
}
/// # GL Invariants
/// target: accepted buffer target (GLenum),
/// size, offest >= 0,
/// usage: accepted buffer usage (GLenum)
///
/// # User Invariants
/// offset + size(data) <= buffer size
/// alignment *must* be respected
/// offset: respects alignment
///
/// # Notes
/// offset: measured in bytes
///
/// # Errors
/// `GL_INVALID_OPERATON`: zero is bound to target, target is being mapped
/// `GL_INVALID_VALUE`: offset + size > buffer size
pub fn BufferSubData<T>(target: BufferType, data: &[T], offset: usize) {
    unsafe {
        gl::BufferSubData(target.into(), offset as isize, std::mem::size_of_val(data) as isize, data.as_ptr().cast())
    }
}

/// # GL Invariants
/// length of vertex_arrays >= 0
pub fn GenVertexArrays(vertex_arrays: &mut [u32]) {
    unsafe {
        // n >= 0
        gl::GenVertexArrays(vertex_arrays.len() as i32, vertex_arrays.as_mut_ptr().cast())
    }
}
/// Ease of use for GenVertexArrays
pub fn GenVertexArray(vertex_array: &mut u32) {
    GenVertexArrays(std::slice::from_mut(vertex_array));
}
/// # GL Invariants
/// length of vertex_arrays >= 0
pub fn DeleteVertexArrays(vertex_arrays: &[u32]) {
    unsafe {
        // n: >= 0
        gl::DeleteVertexArrays(vertex_arrays.len() as i32, vertex_arrays.as_ptr())
    }
}
pub fn DeleteVertexArray(vertex_array: u32) {
    DeleteVertexArrays(&[vertex_array])
}

/// # User Invariants
/// vertex_array: is a valid vertex array returned by `glGenVertexArrays` or 0
///
/// # Errors
/// `GL_INVALID_VALUE`: vertex_array was not returned by `glGenVertexArrays`, 0 or was deleted
pub fn BindVertexArray(vertex_array: u32) {
    unsafe { gl::BindVertexArray(vertex_array) }
}

/// # User Invariants
/// vertex array object must be bound
/// attribute_index: < `GL_MAX_VERTEX_ATTRIBS`
/// 
/// # Errors
/// `GL_INVALID_OPERATON`: no vertex array object is bound
/// `GL_INVALID_VALUE`: attribute_index >= `GL_MAX_VERTEX_ATTRIBS`
pub fn EnableVertexAttribArray(attribute_index: u8) {
    unsafe {
        gl::EnableVertexAttribArray(attribute_index as u32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttribSize {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Bgra
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntegralAttribSize {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}
/// # GL Invariants
/// size: 1,2,3,4 or `GL_BGRA`
/// type: accepted value (GLenum)
/// stride: >= 0
/// 
/// # User Invariants
/// index: < `GL_MAX_VERTEX_ATTRIBS`
/// size: `GL_BGRA`; type: `GL_UNSIGNED_BYTE`, `GL_INT_2_10_10_10_REV`,
///     `GL_UNSIGNED_INT_2_10_10_10_REV`
/// type: `GL_INT_2_10_10_10_REV`, `GL_UNSIGNED_INT_2_10_10_10_REV`; size: 4, `GL_BGRA`
/// type: `GL_UNSIGNED_INT_10f_11f_11f_REV`; size: 3
/// size: `GL_BGRA`; normalized: `GL_FALSE`
/// array buffer bound to 0; offset: != 0
/// # Errors
/// `GL_INVALID_VALUE`: index >= GL_MAX_VERTEX_ATTRIBS
/// `GL_INVALID_OPERATON`: any of the other user invariants are violated
pub fn VertexAttribPointer(index: u8, size: AttribSize, data_type: DataType, normalised: bool, stride: usize, offset: usize) {
    unsafe {
        gl::VertexAttribPointer(index as u32, size.into(), data_type.into(), normalised.into(),
            stride as i32, offset as *const _)
    }
}
/// # GL Invariants
/// size: 1,2,3,4
/// type: accepted value (GLenum)
/// stride: >= 0
/// 
/// # User Invariants
/// index: < `GL_MAX_VERTEX_ATTRIBS`
/// array buffer bound to 0; offset: != 0
///
/// # Errors
/// `GL_INVALID_VALUE`: index >= GL_MAX_VERTEX_ATTRIBS
/// `GL_INVALID_OPERATON`: any of the other user invariants are violated
pub fn VertexAttribIPointer(index: u8, size: IntegralAttribSize, data_type: IntegralDataType, stride: usize, offset: usize) {
    unsafe {
        gl::VertexAttribIPointer(index as u32, size.into(), data_type.into(),
            stride as i32, offset as *const _)
    }
}

// TODO: all parameters... eish
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Parameter {
    MaxVertexAttribs,
    ArrayBufferBinding,
    ElementBufferBinding,
    MaxComputeShaderStorageBlocks,
}
/// # GL Invariants
/// parameter: an accepted value (GLenum)
///
/// # Safety
/// data: large enough to accomodate the parameter
pub unsafe fn GetIntegerv(parameter: Parameter, data: &mut [i32]) {
    unsafe {
        gl::GetIntegerv(parameter.into(), data.as_mut_ptr())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Compute,
    Vertex,
    TessControl,
    TessEvaluation,
    Geometry,
    Fragment
}
/// # GL Invariants
/// shader_type: accepted value (GLenum)
///
/// # Notes
/// returns 0 if the process fails
pub fn CreateShader(shader_type: ShaderType) -> u32 {
    unsafe {
        gl::CreateShader(shader_type.into())
    }
}

/// # User Invariants
/// shader: valid shader generated by OpenGL or 0
///
/// # Errors
/// `GL_INVALID_VALUE`: shader is not a value generated by OpenGL, 0 or was deleted
pub fn DeleteShader(shader: u32) {
    unsafe {
        gl::DeleteShader(shader)
    }
}

/// # GL Invariants
/// length of sources >= 0
///
/// # User Invariants
/// shader: is a valid shader
///
/// # Errors
/// `GL_INVALID_VALUE`: shader is not a value generated by OpenGL, 0, or was deleted
/// `GL_INVALID_OPERATON`: shader is not a valid shader object
pub fn ShaderSource<CS>(shader: u32, sources: &[CS]) 
    where CS: AsRef<std::ffi::CStr> {
    let sources: Vec<_> = sources.iter().map(|cs| cs.as_ref().as_ptr()).collect();
    unsafe {
        gl::ShaderSource(shader, sources.len() as i32, sources.as_ptr(), std::ptr::null())
    }
}

/// # User Invariants
/// shader: is a valid shader
///
/// # Errors
/// `GL_INVALID_VALUE`: shader is not a value generated by OpenGL, 0 or was deleted
/// `GL_INVALID_OPERATON`: shader is not a valid shader object
///
/// # Notes
/// success or failure stored in `GL_COMPILE_STATUS` flag, accessed through `glGetShaderiv`
pub fn CompileShader(shader: u32) {
    unsafe {
        gl::CompileShader(shader);
    }
}

/// # User Invariants
/// program: valid program object
/// shader: valid shader object
///
/// # Errors
/// `GL_INVALID_VALUE`: program, shader not values generated by OpenGL or deleted
/// `GL_INVALID_OPERATON`: program, shader not valid program, shader objects respectively
/// `GL_INVALID_OPERATON`: shader already attached to program
pub fn AttachShader(program: u32, shader: u32) {
    unsafe {
        gl::AttachShader(program, shader)
    }
}

/// # User Invariants
/// program: valid program object
/// shader: valid shader object
///
/// # Errors
/// `GL_INVALID_VALUE`: program, shader not values generated by OpenGL or deleted
/// `GL_INVALID_OPERATON`: program, shader not valid program, shader objects respectively
/// `GL_INVALID_OPERATON`: shader is not attached to program
pub fn DetachShader(program: u32, shader: u32) {
    unsafe {
        gl::DetachShader(program, shader)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShaderParameter {
    ShaderType,
    DeleteStatus,
    CompileStatus,
    InfoLogLength,
    ShaderSourceLength
}
/// # GL Invariants
/// parameter: accepted value (GLenum)
///
/// # User Invariants
/// shader: valid shader object
///
/// # Errors
/// `GL_INVALID_VALUE`: shader is not a value generated by OpenGL
/// `GL_INVALID_OPERATON`: shader is not a valid shader object
pub fn GetShaderiv(shader: u32, parameter: ShaderParameter, data: &mut i32) {
    unsafe {
        // SAFETY: always returns a single value ==> ptr never out of bounds
        gl::GetShaderiv(shader, parameter.into(), data)
    }
}

/// Only for interop with the weird GetShaderiv
pub use gl::TRUE as glTrue;
/// Only for interop with the weird GetShaderiv
pub use gl::FALSE as glFalse;

/// # GL Invariants
/// buffer length: >= 0
///
/// # User Invariants
/// shader: valid shader object
///
/// # Errors
/// `GL_INVALID_VALUE`: shader is not a value generated by OpenGL
/// `GL_INVALID_OPERATON`: shader is not a valid shader object
///
/// # Notes
/// length: None (~NULL) specifies that no length should be returned
pub fn GetShaderInfoLog(shader: u32, buffer: &mut [std::ffi::c_char], length: Option<&mut i32>) {
    unsafe {
        gl::GetShaderInfoLog(shader, buffer.len() as i32,
            length.map(|l| l as *mut _).unwrap_or(std::ptr::null_mut()),
            buffer.as_mut_ptr());
    }
}

/// # Notes
/// returns 0 if the process fails
pub fn CreateProgram() -> u32 {
    unsafe {
        gl::CreateProgram()
    }
}

/// # User Invariants
/// program: valid program object; if active program object, transform feedback mode is inactive
///
/// # Errors:
/// `GL_INVALID_VALUE`: program is not a value generated by OpenGL or was deleted
/// `GL_INVALID_OPERATON`: program is not a program object
/// `GL_INVALID_OPERATON`: program is the currently active program object, and transform feedback
///                         mode is active
pub fn LinkProgram(program: u32) {
    unsafe {
        gl::LinkProgram(program)    
    }
}

pub enum ProgramParameter {
    DeleteStatus,
    LinkStatus,
    ValidateStatus, 
    InfoLogLength, 
    AttachedShaders, 
    ActiveAttributes, 
    ActiveAttributeMaxLength, 
    ActiveUniforms, 
    ActiveUniformBlocks, 
    ActiveUniformBlockMaxNameLength, 
    ActiveUniformMaxLength, 
    TransformFeedbackBufferMode, 
    TransformFeedbackVaryings, 
    TransformFeedbackVaryingMaxLength, 
    GeometryVerticesOut, 
    GeometryInputType, 
    GeometryOutputType
}
/// # GL Invariants
/// parameter: accepted value (GLenum)
///
/// # User Invariants
/// program: valid program object
///
/// # Errors
/// `GL_INVALID_VALUE`: program is not a value generated by OpenGL
/// `GL_INVALID_OPERATON`: program is not a valid shader object, or
///                        parameter is GeometryVerticesOut, GeometryInputType,
///                            GeometryOutputType without a geometry shader
pub fn GetProgramiv(program: u32, parameter: ProgramParameter, data: &mut i32) {
    unsafe {
        // SAFETY: always returns a single value ==> ptr never out of bounds
        gl::GetProgramiv(program, parameter.into(), data)
    }
}

/// # GL Invariants
/// buffer length: >= 0
///
/// # User Invariants
/// program: valid program object
///
/// # Errors
/// `GL_INVALID_VALUE`: program is not a value generated by OpenGL
/// `GL_INVALID_OPERATON`: program is not a valid program object
///
/// # Notes
/// length: None (~NULL) specifies that no length should be returned
pub fn GetProgramInfoLog(program: u32, buffer: &mut [std::ffi::c_char], length: Option<&mut i32>) {
    unsafe {
        gl::GetProgramInfoLog(program, buffer.len() as i32,
            length.map(|l| l as *mut _).unwrap_or(std::ptr::null_mut()),
            buffer.as_mut_ptr());
    }
}

/// # User Invariants
/// program: valid program object or 0
///
/// # Errors:
/// `GL_INVALID_VALUE`: program is not a value generated by OpenGL or 0
/// `GL_INVALID_OPERATON`: program is not a valid program object
/// `GL_INVALID_OPERATON`: transform feedback mode is active
pub fn UseProgram(program: u32) {
    unsafe {
        gl::UseProgram(program)
    }
}

/// # User Invariants
/// program: value generated by OpenGL
///
/// # Errors
/// `GL_INVALID_VALUE`: program is not a value generated by OpenGL, or was deleted
pub fn DeleteProgram(program: u32) {
    unsafe {
        gl::DeleteProgram(program)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DrawMode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IndexType {
    UnsignedByte,
    UnsignedShort,
    UnsignedInt
}
/// # GL Invariants
/// mode: accepted value (GLenum)
/// count: >= 0
///
/// # User Invariants
/// mode: incompatible with primitive type of the geometry shader
/// _see second error below_
///
/// # Errors
/// `GL_INVALID_OPERATON`: a geometry shader is active and mode is incompatible with the input primitive type 
///                         of the geometry shader in the currently installed program object.
/// `GL_INVALID_OPERATON`: non-zero buffer object name is bound to an enabled array or the element array
///                         and the buffer object's data store is currently mapped
pub fn DrawElements(mode: DrawMode, count: usize, index_type: IndexType, offset: usize) {
    unsafe {
        gl::DrawElements(mode.into(), count as i32, index_type.into(), offset as *const _)
    }
}

/// # GL Invariants
/// mode: accepted value (GLenum)
/// count: >= 0
///
/// # User Invariants
/// mode: incompatible with primitive type of the geometry shader
/// _see second error below_
///
/// # Errors
/// `GL_INVALID_OPERATON`: a geometry shader is active and mode is incompatible with the input primitive type 
///                         of the geometry shader in the currently installed program object.
/// `GL_INVALID_OPERATON`: non-zero buffer object name is bound to an enabled array or the element array
///                         and the buffer object's data store is currently mapped
pub fn DrawArrays(mode: DrawMode, first: usize, count: usize) {
    unsafe {
        gl::DrawArrays(mode.into(), first as i32, count as i32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
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
    UnsignedInt2_10_10_10Rev,
    UnsignedInt10f11f11fRev
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntegralDataType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
}

/// # Notes
/// `GL_INVALID_ENUM`: not supported, as rust enums *should* make this impossible
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    // InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    StackOverflow
}
pub fn GetError() -> Option<Error> {
    let e = unsafe { gl::GetError() };
    if e == 0 {
        None
    } else {
        // glGetError always returns a valid error code
        Some(e.into())
    }
}


// Enum Conversions
impl From<BufferType> for gl::types::GLenum {
    fn from(val: BufferType) -> Self {
        match val {
            BufferType::Array => gl::ARRAY_BUFFER,
            BufferType::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            BufferType::CopyRead => gl::COPY_READ_BUFFER,
            BufferType::CopyWrite => gl::COPY_WRITE_BUFFER,
            BufferType::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            BufferType::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            BufferType::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::PixelPack => gl::PIXEL_PACK_BUFFER,
            BufferType::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            BufferType::Query => gl::QUERY_BUFFER,
            BufferType::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
            BufferType::Texture => gl::TEXTURE_BUFFER,
            BufferType::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferType::Uniform => gl::UNIFORM_BUFFER,
        }
    }
} 
impl From<BufferUsage> for gl::types::GLenum {
    fn from(val: BufferUsage) -> Self {
        match val {
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
            BufferUsage::StreamRead => gl::STREAM_READ,
            BufferUsage::StreamCopy => gl::STREAM_COPY,
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
            BufferUsage::StaticRead => gl::STATIC_READ,
            BufferUsage::StaticCopy => gl::STATIC_COPY,
            BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsage::DynamicRead => gl::DYNAMIC_READ,
            BufferUsage::DynamicCopy => gl::DYNAMIC_COPY
        }
    }
}
impl From<gl::types::GLenum> for Error {
    fn from(val: gl::types::GLenum) -> Self {
        match val {
            gl::INVALID_ENUM => unreachable!("No invalid enum errors supported"),
            gl::INVALID_VALUE => Error::InvalidValue,
            gl::INVALID_OPERATION => Error::InvalidOperation,
            gl::INVALID_FRAMEBUFFER_OPERATION => Error::InvalidFramebufferOperation,
            gl::OUT_OF_MEMORY => Error::OutOfMemory,
            gl::STACK_UNDERFLOW => Error::StackUnderflow,
            gl::STACK_OVERFLOW => Error::StackOverflow,
            _ => unreachable!("Error type used with function other than glGetError")
        }
    }
}
impl From<Parameter> for gl::types::GLenum {
    fn from(val: Parameter) -> Self {
        match val {
            Parameter::MaxVertexAttribs => gl::MAX_VERTEX_ATTRIBS,
            Parameter::ArrayBufferBinding => gl::ARRAY_BUFFER_BINDING,
            Parameter::ElementBufferBinding => gl::ELEMENT_ARRAY_BUFFER_BINDING,
            Parameter::MaxComputeShaderStorageBlocks => gl::MAX_COMPUTE_SHADER_STORAGE_BLOCKS,
        }
    }
}
impl From<AttribSize> for gl::types::GLint {
    fn from(val: AttribSize) -> Self {
        match val {
            AttribSize::Bgra => gl::BGRA as gl::types::GLint,
            _ => val as gl::types::GLint,
        }
    }
}
impl From<IntegralAttribSize> for gl::types::GLint {
    fn from(val: IntegralAttribSize) -> Self {
        val as gl::types::GLint
    }
}
impl From<ShaderType> for gl::types::GLenum {
    fn from(val: ShaderType) -> Self {
        match val {
            ShaderType::Compute => gl::COMPUTE_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::TessControl => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}
impl From<ShaderParameter> for gl::types::GLenum {
    fn from(val: ShaderParameter) -> Self {
        match val {
            ShaderParameter::ShaderType => gl::SHADER_TYPE,
            ShaderParameter::DeleteStatus => gl::DELETE_STATUS,
            ShaderParameter::CompileStatus => gl::COMPILE_STATUS,
            ShaderParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
            ShaderParameter::ShaderSourceLength => gl::SHADER_SOURCE_LENGTH,
        }
    }
}
impl From<ProgramParameter> for gl::types::GLenum {
    fn from(val: ProgramParameter) -> Self {
        match val {
            ProgramParameter::DeleteStatus => gl::DELETE_STATUS,
            ProgramParameter::LinkStatus => gl::LINK_STATUS,
            ProgramParameter::ValidateStatus => gl::VALIDATE_STATUS,
            ProgramParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
            ProgramParameter::AttachedShaders => gl::ATTACHED_SHADERS,
            ProgramParameter::ActiveAttributes => gl::ACTIVE_ATTRIBUTES,
            ProgramParameter::ActiveAttributeMaxLength => gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
            ProgramParameter::ActiveUniforms => gl::ACTIVE_UNIFORMS,
            ProgramParameter::ActiveUniformBlocks => gl::ACTIVE_UNIFORM_BLOCKS,
            ProgramParameter::ActiveUniformBlockMaxNameLength => gl::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
            ProgramParameter::ActiveUniformMaxLength => gl::ACTIVE_UNIFORM_MAX_LENGTH,
            ProgramParameter::TransformFeedbackBufferMode => gl::TRANSFORM_FEEDBACK_BUFFER_MODE,
            ProgramParameter::TransformFeedbackVaryings => gl::TRANSFORM_FEEDBACK_VARYINGS,
            ProgramParameter::TransformFeedbackVaryingMaxLength => gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
            ProgramParameter::GeometryVerticesOut => gl::GEOMETRY_VERTICES_OUT,
            ProgramParameter::GeometryInputType => gl::GEOMETRY_INPUT_TYPE,
            ProgramParameter::GeometryOutputType => gl::GEOMETRY_OUTPUT_TYPE,
        }
    }
}
impl From<DrawMode> for gl::types::GLenum {
    fn from(val: DrawMode) -> Self {
        match val {
            DrawMode::Points => gl::POINTS,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::LineLoop => gl::LINE_LOOP,
            DrawMode::Lines => gl::LINES,
            DrawMode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            DrawMode::LinesAdjacency => gl::LINES_ADJACENCY,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            DrawMode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            DrawMode::Patches => gl::PATCHES,
        }
    }
}
impl From<DataType> for gl::types::GLenum {
    fn from(val: DataType) -> Self {
        match val {
            DataType::Byte => gl::BYTE,
            DataType::UnsignedByte => gl::UNSIGNED_BYTE,
            DataType::Short => gl::SHORT,
            DataType::UnsignedShort => gl::UNSIGNED_SHORT,
            DataType::Int => gl::INT,
            DataType::UnsignedInt => gl::UNSIGNED_INT,
            DataType::HalfFloat => gl::HALF_FLOAT,
            DataType::Float => gl::FLOAT,
            DataType::Double => gl::DOUBLE,
            DataType::Fixed => gl::FIXED,
            DataType::Int2_10_10_10Rev => gl::INT_2_10_10_10_REV,
            DataType::UnsignedInt2_10_10_10Rev => gl::UNSIGNED_INT_2_10_10_10_REV,
            DataType::UnsignedInt10f11f11fRev => gl::UNSIGNED_INT_10F_11F_11F_REV,
        }
    }
}
impl From<IntegralDataType> for gl::types::GLenum {
    fn from(val: IntegralDataType) -> Self {
        match val {
            IntegralDataType::Byte => gl::BYTE,
            IntegralDataType::UnsignedByte => gl::UNSIGNED_BYTE,
            IntegralDataType::Short => gl::SHORT,
            IntegralDataType::UnsignedShort => gl::UNSIGNED_SHORT,
            IntegralDataType::Int => gl::INT,
            IntegralDataType::UnsignedInt => gl::UNSIGNED_INT,
        }
    }
}
impl From<IndexType> for gl::types::GLenum {
    fn from(val: IndexType) -> Self {
        match val {
            IndexType::UnsignedByte => gl::UNSIGNED_BYTE,
            IndexType::UnsignedShort => gl::UNSIGNED_SHORT,
            IndexType::UnsignedInt => gl::UNSIGNED_INT,
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::InvalidValue => "invalid value",
            Error::InvalidOperation => "invalid operation",
            Error::InvalidFramebufferOperation => "invalid framebuffer operation",
            Error::OutOfMemory => "out of memory",
            Error::StackUnderflow => "stack underflow",
            Error::StackOverflow => "stack overflow",
        };
        write!(f, "{s}")
    }
}
impl std::error::Error for Error {}
