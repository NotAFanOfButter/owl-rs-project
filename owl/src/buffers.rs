use crate::ox;
use crate::traits::ToByteVec;
use crate::{OwlError, ToOwlError};
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
// INVARIANT: will not be deleted until it is dropped
// fewer calls can fail, reducing error handling, but they now "expect"
impl<T: ToByteVec> ArrayBuffer<T> {
    /// # Errors
    ///
    /// ## Out of Memory
    /// This function will return an error if we are out of memory, and thus no more data can be buffered.
    /// While you may technically be able to create a buffer, no data can be passed to it;
    /// at this time, I do not see this as a common use case, and disallow it in favour
    /// of a more ergonomic API
    pub fn new(data: Vec<T>, usage: BufferUsage) -> Result<Self, OwlError> 
        where T: ToByteVec {
        let created = Self(Buffer::new());
        created.bind();
        // buffer cannot be immutable, so must be out of memory
        ox::buffer_data(ox::BufferType::Array, data, usage)
            .with_context("creating ArrayBuffer")?;
        Ok(created)
    }
    // TODO: error handling... again.

    /// # Errors
    ///
    /// This function will return an error if the size of the data at the given offset overflows
    /// the buffer, or if the buffer is being mapped. At the moment, I have no clue what the
    /// latter means.
    pub fn update(&mut self, data: Vec<T>, offset: usize) -> Result<(),OwlError> {
        self.bind();
        ox::buffer_subdata(ox::BufferType::Array, data, offset).map_err(|e| {
            match e {
                ox::OxError::BaseError(crate::OriginalError::InvalidOperation) => {
                    e.with_message("offset + data length > buffer size")
                },
                ox::OxError::BaseError(crate::OriginalError::InvalidValue) => {
                    e.with_message("buffer is being mapped")
                },
                _ => e.with_message("no other errors should be produced")
            }
            .with_context("updating ArrayBuffer")
        })
    }
    // let's see if we can't limit the scope to crate.
    pub(crate) fn bind(&self) {
        ox::bind_buffer(ox::BufferType::Array, Some(self.0.id))
            .expect("buffer should not be deleted yet");
    }
    pub(crate) fn unbind() {
        ox::bind_buffer(ox::BufferType::Array, None)
            .expect("binding 0 always succeeds");
    }
}
impl<T: ToByteVec> Drop for ArrayBuffer<T> {
    fn drop(&mut self) {
        ox::delete_buffer(self.0.id);
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
        // buffer cannot be immutable, so must be out of memory
        ox::buffer_data(ox::BufferType::ElementArray, data, usage)
            .with_context("creating ElementBuffer")?;
        Ok(created)
    }
    /// # Errors
    ///
    /// This function will return an error if the size of the data at the given offset overflows
    /// the buffer, or if the buffer is being mapped. At the moment, I have no clue what the
    /// latter means.
    pub fn update(&mut self, data: Vec<T>, offset: usize) -> Result<(),OwlError> {
        self.bind();
        ox::buffer_subdata(ox::BufferType::ElementArray, data, offset).map_err(|e| {
            match e {
                ox::OxError::BaseError(crate::OriginalError::InvalidOperation) => {
                    e.with_message("offset + data length > buffer size")
                },
                ox::OxError::BaseError(crate::OriginalError::InvalidValue) => {
                    e.with_message("buffer is being mapped")
                },
                _ => e.with_message("no other errors should be produced")
            }
            .with_context("updating ElementBuffer")
        })
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
