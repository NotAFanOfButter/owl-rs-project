use crate::ox;
use crate::traits::ToByteVec;
use crate::{OwlError, UnmessagedError};
pub use ox::BufferUsage;

/// A struct to couple the name / id of a buffer with ownership of its data.
/// For now, it does not actually contain the data stored in OpenGL memory
#[derive(Clone, Debug)]
struct Buffer<T: ToByteVec> {
    id: ox::Buffer,
    _ghost: std::marker::PhantomData<T>
}
impl<T: ToByteVec> Buffer<T> {
    fn new() -> Self {
        Self {
            id: ox::gen_buffer(),
            _ghost: std::marker::PhantomData,
        }
    }
}
impl<T: ToByteVec> Default for Buffer<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: ToByteVec> PartialEq for Buffer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: ToByteVec> Eq for Buffer<T> {}
/// A wrapper around [Buffer], that allows functions using it to specify the `ARRAY_BUFFER` target
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayBuffer<T: ToByteVec>(Buffer<T>);
impl<T: ToByteVec> ArrayBuffer<T> {
    // INVARIANT: will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    /// # Errors
    /// out of memory
    pub fn new(data: Vec<T>, usage: BufferUsage) -> Result<Self, OwlError> 
        where T: ToByteVec {
        let created = Self(Buffer::new());
        created.bind();
        ox::buffer_data(ox::BufferType::Array, data, usage)
            .map_err(|e| e.with_message("creating ArrayBuffer, failed to buffer data"))?;
        Ok(created)
    }
    pub fn update(&mut self, data: Vec<T>, offset: usize) -> Result<(),OwlError> {
        self.bind();
        ox::buffer_subdata(ox::BufferType::Array, data, offset)
            .map_err(|e| e.with_message("updating ArrayBuffer, failed to replace existing data"))
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
impl<T: ToByteVec> Drop for ArrayBuffer<T> {
    fn drop(&mut self) {
        ox::delete_buffer(self.0.id)
    }
}

pub use ox::IndexType;
/// A wrapper around [Buffer], that allows functions using it to specify the `ARRAY_BUFFER` target
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementBuffer<T: ToByteVec>{
    inner: Buffer<T>,
    pub(crate) inner_type: ox::IndexType
}
impl<T: ToByteVec> ElementBuffer<T> {
    // INVARIANT: buffer will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    /// # Errors
    /// out of memory
    pub fn new(data: Vec<T>, usage: BufferUsage, index_type: IndexType) -> Result<Self, OwlError> 
        where T: ToByteVec {
        let created = Self {
            inner: Buffer::new(),
            inner_type: index_type
        };
        created.bind();
        ox::buffer_data(ox::BufferType::ElementArray, data, usage)
            .map_err(|e| e.with_message("creating ElementBuffer, failed to buffer data"))?;
        Ok(created)
    }
    pub fn update(&mut self, data: Vec<T>, offset: usize) -> Result<(),OwlError> {
        self.bind();
        ox::buffer_subdata(ox::BufferType::ElementArray, data, offset)
            .map_err(|e| e.with_message("updating ElementBuffer, failed to replace existing data"))
    }
    /// let's see if we can't limit the scope to crate.
    pub(crate) fn bind(&self) {
        ox::bind_buffer(ox::BufferType::ElementArray, Some(self.inner.id))
            .expect("buffer should not be deleted yet");
    }
    pub(crate) fn unbind() {
        ox::bind_buffer(ox::BufferType::ElementArray, None).expect("binding 0 always succeeds");
    }
}
impl<T: ToByteVec> Drop for ElementBuffer<T> {
    fn drop(&mut self) {
        ox::delete_buffer(self.inner.id);
    }
}
