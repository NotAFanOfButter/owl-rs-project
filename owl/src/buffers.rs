use crate::ox;
use crate::{OwlError, UnmessagedError};
pub use ox::BufferUsage;

mod traits;
pub use traits::ToByteVec;

/// A struct to couple the name / id of a buffer with ownership of its data.
/// For now, it does not actually contain the data stored in OpenGL memory
#[derive(Clone, Debug)]
struct Buffer {
    id: ox::Buffer,
}
impl Buffer {
    fn new() -> Self {
        Buffer { id: ox::gen_buffer() }
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
impl PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Buffer {}
/// A wrapper around [Buffer], that allows functions using it to specify the `ARRAY_BUFFER` target
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayBuffer(Buffer);
impl ArrayBuffer {
    // INVARIANT: buffer will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    /// # Errors
    /// out of memory
    pub fn new<T>(data: Vec<T>, usage: BufferUsage) -> Result<Self,OwlError> {
        let id = ox::gen_buffer();
        ox::buffer_data(ox::BufferType::Array, &data, usage)
            .map_err(|e| e.with_message("failed to buffer data"))?;
        Ok(Self(Buffer { id }))
    }
    pub fn update<T>(data: Vec<T>, offset: usize) -> Result<Self, OwlError> {

    }
    // let's see if we can't limit the scope to crate.
    pub(crate) fn bind(&self) {
        ox::bind_buffer(ox::BufferType::Array, Some(self.0.id))
            .expect("buffer should not be deleted yet")
    }
    pub(crate) fn unbind(&self) {
        ox::bind_buffer(ox::BufferType::Array, None)
            .expect("binding 0 always succeeds")
    }
}
impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        ox::delete_buffer(self.0.id)
    }
}

/// A wrapper around [Buffer], that allows functions using it to specify the `ARRAY_BUFFER` target
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementBuffer(Buffer);
impl ElementBuffer {
    // INVARIANT: buffer will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    /// # Errors
    /// out of memory
    pub fn new<T>(data: Vec<T>, usage: BufferUsage) -> Result<Self,OwlError> {
        let id = ox::gen_buffer();
        ox::buffer_data(ox::BufferType::ElementArray, &data, usage)
            .map_err(|e| e.with_message("failed to buffer data"))?;
        Ok(Self(Buffer { id }))
    }
    /// let's see if we can't limit the scope to crate.
    pub(crate) fn bind(&self) {
        ox::bind_buffer(ox::BufferType::Array, Some(self.0.id))
            .expect("buffer should not be deleted yet")
    }
    pub(crate) fn unbind(&self) {
        ox::bind_buffer(ox::BufferType::Array, None)
            .expect("binding 0 always succeeds")
    }
}
impl Drop for ElementBuffer {
    fn drop(&mut self) {
        ox::delete_buffer(self.0.id)
    }
}
