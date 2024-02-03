//! # Warning
//! ## Types
//! many u32's (and usizes) in this module are actually "u31's", by necessity of conversion to the (non-negative)
//! i32's requested by OpenGL.
//! ## Creation and Deletion
//! where a function is documented as taking a "valid" id / name of an object, it is given that the
//! object has not previously been deleted.

#![warn(clippy::missing_panics_doc)]
#![allow(non_snake_case)]
// #![allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)] // gl type weirdness means this kind of thing is often required

use bitflags::bitflags;

/// # Notes
/// values clamped to [0,1]
#[inline]
pub fn ClearColour(red: f32, green: f32, blue: f32, alpha: f32) {
    // SAFETY: FFI
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
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
#[inline]
pub fn Clear(flags: ClearFlags) {
    // SAFETY: FFI
    unsafe {
        gl::Clear(flags.into());
    }
}

/// # GL Invariants
/// length of buffers >= 0
///
/// # Panics
/// This will panic if you request more than `i32::MAX` buffers at once.
/// You don't need that many.
#[inline]
pub fn GenBuffers(buffers: &mut [u32]) {
    // SAFETY: the pointer to the slice is aligned, and
    //         will not be mutated elsewhere for the duration of this call.
    //         the write is constrained by the length of the slice, and so
    //         will not go out of bounds
    unsafe {
        gl::GenBuffers(i32::try_from(buffers.len()).expect("number of buffers > i32::MAX"),
             buffers.as_mut_ptr());
    }
}
/// Ease of use for [`GenBuffers`]
#[inline]
pub fn GenBuffer(buffer: &mut u32) {
    GenBuffers(std::slice::from_mut(buffer));
}

/// # GL Invariants
/// length of buffers >= 0
///
/// # Panics
/// This will panic if you try to delete more than `i32::MAX` buffers at once.
/// You don't need that many.
#[inline]
pub fn DeleteBuffers(buffers: &[u32]) {
    // SAFETY: the pointer to the buffer slice is non-null, aligned,
    //         and initialised, over the length of the slice.
    //         It will not be mutated anywhere else for the duration of this call.
    unsafe {
        gl::DeleteBuffers(i32::try_from(buffers.len()).expect("number of buffers > i32::MAX"),
            buffers.as_ptr());
    }
}
#[inline]
pub fn DeleteBuffer(buffer: u32) {
    DeleteBuffers(&[buffer]);
}

/// # GL Invariants
/// target: is an accepted buffer type
///
/// # User Invariants
/// buffer: is a valid buffer returned by `glGenBuffers` or 0
///
/// # Errors
/// `GL_INVALID_VALUE`: buffer was not returned by `glGenBuffers`, 0, or was deleted
#[inline]
pub fn BindBuffer(target: BufferType, buffer: u32) {
    // SAFETY: FFI
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
///
/// # Notes
/// Any data after `isize::MAX` bytes will be truncated - OpenGL limitation
#[inline]
pub fn BufferData<T>(target: BufferType, data: &[T], usage: BufferUsage) {
    // SAFETY: the pointer to the data slice is non-null, aligned,
    //         and initialised over the length of the slice.
    unsafe {
        #[allow(clippy::cast_possible_wrap)]
        gl::BufferData(target.into(),
            std::mem::size_of_val(data) as isize,
            data.as_ptr().cast(), usage.into());
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
/// `offset`: measured in bytes
/// Any data after `isize::MAX` bytes into the `data` will be truncated - OpenGL limitation
///
/// # Errors
/// `GL_INVALID_OPERATON`: zero is bound to target, target is being mapped
/// `GL_INVALID_VALUE`: offset + size > buffer size
///
/// # Panics
/// This will panic if the offset is > `isize::MAX`
/// that's the maximum amount of data a buffer can store anyway.
#[inline]
pub fn BufferSubData<T>(target: BufferType, data: &[T], offset: usize) {
    // SAFETY: the pointer to the data slice is non-null, aligned,
    //         and initialised over the length of the slice.
    unsafe {
        #[allow(clippy::cast_possible_wrap)]
        gl::BufferSubData(target.into(),
            isize::try_from(offset).expect("offset > isize::MAX"),
             std::mem::size_of_val(data) as isize,
             data.as_ptr().cast());
    }
}

/// # GL Invariants
/// length of `vertex_arrays` >= 0
///
/// # Panics
/// This will panic if you request more than `i32::MAX` vertex arrays at once.
/// You don't need that many
#[inline]
pub fn GenVertexArrays(vertex_arrays: &mut [u32]) {
    // SAFETY: the pointer to the slice is aligned, and
    //         will not be mutated elsewhere for the duration of this call.
    //         the write is constrained by the length of the slice, and so
    //         will not go out of bounds
    unsafe {
        gl::GenVertexArrays(i32::try_from(vertex_arrays.len()).expect("number of vertex arrays > i32::MAX"),
            vertex_arrays.as_mut_ptr().cast());
    }
}
/// Ease of use for [`GenVertexArrays`]
#[inline]
pub fn GenVertexArray(vertex_array: &mut u32) {
    GenVertexArrays(std::slice::from_mut(vertex_array));
}
/// # GL Invariants
/// length of `vertex_arrays` >= 0
///
/// # Panics
/// This will panic if you pass more than `i32::MAX` vertex arrays at once.
/// You don't need that many
#[inline]
pub fn DeleteVertexArrays(vertex_arrays: &[u32]) {
    // SAFETY: the pointer to the slice is non-null, aligned,
    //         and initialised over the length of the slice.
    unsafe {
        gl::DeleteVertexArrays(i32::try_from(vertex_arrays.len()).expect("number of vertex arrays > i32::MAX"),
            vertex_arrays.as_ptr());
    }
}
#[inline]
pub fn DeleteVertexArray(vertex_array: u32) {
    DeleteVertexArrays(&[vertex_array]);
}

/// # User Invariants
/// `vertex_array`: is a valid vertex array returned by `glGenVertexArrays` or 0
///
/// # Errors
/// `GL_INVALID_VALUE`: `vertex_array` was not returned by `glGenVertexArrays`, 0 or was deleted
#[inline]
pub fn BindVertexArray(vertex_array: u32) {
    // SAFETY: FFI
    unsafe { gl::BindVertexArray(vertex_array) };
}

/// # User Invariants
/// vertex array object must be bound
/// `attribute_index`: < `GL_MAX_VERTEX_ATTRIBS`
/// 
/// # Errors
/// `GL_INVALID_OPERATON`: no vertex array object is bound
/// `GL_INVALID_VALUE`: `attribute_index` >= `GL_MAX_VERTEX_ATTRIBS`
#[inline]
pub fn EnableVertexAttribArray(attribute_index: u8) {
    // SAFETY: FFI
    unsafe {
        gl::EnableVertexAttribArray(u32::from(attribute_index));
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
///
/// # Errors
/// `GL_INVALID_VALUE`: index >= `GL_MAX_VERTEX_ATTRIBS`
/// `GL_INVALID_OPERATON`: any of the other user invariants are violated
///
/// # Panics
/// This function will panic if the stride or offset > `i32::MAX`.
/// ## Notes
/// The API for this function is suuuuper dumb for legacy reasons, sorry
#[inline]
pub fn VertexAttribPointer(index: u8, size: AttribSize, data_type: DataType, normalised: bool, stride: usize, offset: usize) {
    // SAFETY: cast to void pointer, I'm told it's meant to be a 4-byte integer
    unsafe {
        gl::VertexAttribPointer(u32::from(index), size.into(), data_type.into(),
            normalised.into(), i32::try_from(stride).expect("stride > i32::MAX"),
            i32::try_from(offset).expect("offset > i32::MAX") as *const std::ffi::c_void);
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
/// `GL_INVALID_VALUE`: index >= `GL_MAX_VERTEX_ATTRIBS`
/// `GL_INVALID_OPERATON`: any of the other user invariants are violated
///
/// # Panics
/// This function will panic if the stride > `i32::MAX`.
/// ## Notes
/// The API for this function is suuuuper dumb for legacy reasons, sorry
#[inline]
pub fn VertexAttribIPointer(index: u8, size: IntegralAttribSize, data_type: IntegralDataType, stride: usize, offset: usize) {
    // SAFETY: cast to void pointer, I'm told it's meant to be a 4-byte integer
    unsafe {
        gl::VertexAttribIPointer(u32::from(index), size.into(), data_type.into(),
            i32::try_from(stride).expect("stride > i32::MAX"),
            offset as *const std::ffi::c_void);
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
    // SAFETY: the pointer to the slice is aligned, and
    //         will not be mutated elsewhere for the duration of this call.
    //         the caller is required to verify that the slice is of the
    //         correct size, such that the 
    unsafe {
        gl::GetIntegerv(parameter.into(), data.as_mut_ptr());
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
/// `shader_type`: accepted value (GLenum)
///
/// # Notes
/// returns 0 if the process fails
#[inline]
pub fn CreateShader(shader_type: ShaderType) -> u32 {
    // SAFETY: FFI
    unsafe {
        gl::CreateShader(shader_type.into())
    }
}

/// # User Invariants
/// shader: valid shader generated by OpenGL or 0
///
/// # Errors
/// `GL_INVALID_VALUE`: shader is not a value generated by OpenGL, 0 or was deleted
#[inline]
pub fn DeleteShader(shader: u32) {
    // SAFETY: FFI
    unsafe {
        gl::DeleteShader(shader);
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
///
/// # Panics
/// This function will panic if the number of sources > `i32::MAX`.
#[inline]
pub fn ShaderSource<CS>(shader: u32, sources: &[CS]) 
    where CS: AsRef<std::ffi::CStr> {
    let sources: Vec<_> = sources.iter().map(|cs| cs.as_ref().as_ptr()).collect();
    // SAFETY: the pointer to the slice is aligned, and
    //         will not be mutated elsewhere for the duration of this call.
    //         the read is constrained by the length of the slice, and so
    //         will not go out of bounds;
    //         the null pointer is expected and tested
    unsafe {
        gl::ShaderSource(shader, i32::try_from(sources.len()).expect("number of sources > i32::MAX"),
            sources.as_ptr(), std::ptr::null());
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
#[inline]
pub fn CompileShader(shader: u32) {
    // SAFETY: FFI
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
#[inline]
pub fn AttachShader(program: u32, shader: u32) {
    // SAFETY: FFI
    unsafe {
        gl::AttachShader(program, shader);
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
#[inline]
pub fn DetachShader(program: u32, shader: u32) {
    // SAFETY: FFI
    unsafe {
        gl::DetachShader(program, shader);
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
#[inline]
pub fn GetShaderiv(shader: u32, parameter: ShaderParameter, data: &mut i32) {
    // SAFETY: always returns a single value ==> ptr never out of bounds
    unsafe {
        gl::GetShaderiv(shader, parameter.into(), data);
    }
}

/// Only for interop with the weird [`GetShaderiv`]
pub use gl::TRUE as glTrue;
/// Only for interop with the weird [`GetShaderiv`]
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
/// # Panics
/// This function panics if buffer.len() > `i32::MAX`
///
/// # Notes
/// length: None (~NULL) specifies that no length should be returned
#[inline]
pub fn GetShaderInfoLog(shader: u32, buffer: &mut [std::ffi::c_char], length: Option<&mut i32>) {
    // SAFETY: the pointer to the slice is aligned, and
    //         will not be mutated elsewhere for the duration of this call.
    //         the write is constrained by the length of the slice, and so
    //         will not go out of bounds;
    //         the null pointer is expected and tested
    unsafe {
        gl::GetShaderInfoLog(shader, i32::try_from(buffer.len()).expect("buffer length > i32::max"),
            length.map_or(std::ptr::null_mut(),|l| l as *mut _),
            buffer.as_mut_ptr());
    }
}

/// # Notes
/// returns 0 if the process fails
#[inline]
pub fn CreateProgram() -> u32 {
    // SAFETY: FFI
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
#[inline]
pub fn LinkProgram(program: u32) {
    // SAFETY: FFI
    unsafe {
        gl::LinkProgram(program);
    }
}

// NB: if ever introduce GL_COMPUTE_WORK_GROUP_SIZE, it returns an array
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
///                        parameter is `GeometryVerticesOut``GeometryInputType`pe,
///                            `GeometryOutputType` without a geometry shader
#[inline]
pub fn GetProgramiv(program: u32, parameter: ProgramParameter, data: &mut i32) {
    // SAFETY: With the provided parameters, it will only write 1 value to the pointer,
    //         so will not access other memory
    unsafe {
        gl::GetProgramiv(program, parameter.into(), data as *mut _);
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
/// # Panics
/// This function panics if buffer.len() > `i32::MAX`
///
/// # Notes
/// length: None (~NULL) specifies that no length should be returned
#[inline]
pub fn GetProgramInfoLog(program: u32, buffer: &mut [std::ffi::c_char], length: Option<&mut i32>) {
    // SAFETY: the pointer to the slice is aligned, and
    //         will not be mutated elsewhere for the duration of this call.
    //         the write is constrained by the length of the slice, and so
    //         will not go out of bounds;
    //         the null pointer is expected and tested
    unsafe {
        gl::GetProgramInfoLog(program, i32::try_from(buffer.len()).expect("buffer length > i32::MAX"),
            length.map_or(std::ptr::null_mut(), |l| l as *mut _),
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
#[inline]
pub fn UseProgram(program: u32) {
    // SAFETY: FFI
    unsafe {
        gl::UseProgram(program);
    }
}

/// # User Invariants
/// program: value generated by OpenGL
///
/// # Errors
/// `GL_INVALID_VALUE`: program is not a value generated by OpenGL, or was deleted
#[inline]
pub fn DeleteProgram(program: u32) {
    // SAFETY: FFI
    unsafe {
        gl::DeleteProgram(program);
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
#[inline]
pub fn DrawElements(mode: DrawMode, count: usize, index_type: IndexType, offset: usize) {
    // SAFETY: cast to void pointer, probably meant to be a size_t ~? usize
    unsafe {
        gl::DrawElements(mode.into(), i32::try_from(count).expect("count > i32::MAX"),
            index_type.into(), offset as *const std::ffi::c_void);
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
///
/// # Panics
/// This function panics if first,count > `i32::MAX`
#[inline]
pub fn DrawArrays(mode: DrawMode, first: usize, count: usize) {
    // SAFETY: FFI
    unsafe {
        gl::DrawArrays(mode.into(), i32::try_from(first).expect("first > i32::MAX"),
            i32::try_from(count).expect("count > i32::MAX"));
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
#[inline]
pub fn GetError() -> Option<Error> {
    // SAFETY: FFI
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
            gl::INVALID_VALUE => Self::InvalidValue,
            gl::INVALID_OPERATION => Self::InvalidOperation,
            gl::INVALID_FRAMEBUFFER_OPERATION => Self::InvalidFramebufferOperation,
            gl::OUT_OF_MEMORY => Self::OutOfMemory,
            gl::STACK_UNDERFLOW => Self::StackUnderflow,
            gl::STACK_OVERFLOW => Self::StackOverflow,
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
            #[allow(clippy::cast_possible_wrap)]    // BGRA guaranteed < i32::MAX
            AttribSize::Bgra => gl::BGRA as Self,
            _ => val as Self,
        }
    }
}
impl From<IntegralAttribSize> for gl::types::GLint {
    fn from(val: IntegralAttribSize) -> Self {
        val as Self
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
impl DataType {
    #[must_use]
    pub const fn size_bytes(&self) -> usize {
        match self {
            Self::Byte | Self::UnsignedByte => 1,
            Self::Short | Self::UnsignedShort | Self::HalfFloat => 2,
            Self::Int  | Self::UnsignedInt | Self::Float | Self::Fixed | Self::Int2_10_10_10Rev |
                Self::UnsignedInt2_10_10_10Rev | Self::UnsignedInt10f11f11fRev => 4,
            Self::Double => 8,
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
impl IntegralDataType {
    #[must_use]
    pub const fn size_bytes(&self) -> usize {
        match self {
            Self::Byte | Self::UnsignedByte => 1,
            Self::Short | Self::UnsignedShort=> 2,
            Self::Int | Self::UnsignedInt => 4,
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
            Self::InvalidValue => "invalid value",
            Self::InvalidOperation => "invalid operation",
            Self::InvalidFramebufferOperation => "invalid framebuffer operation",
            Self::OutOfMemory => "out of memory",
            Self::StackUnderflow => "stack underflow",
            Self::StackOverflow => "stack overflow",
        };
        write!(f, "{s}")
    }
}
impl std::error::Error for Error {}
