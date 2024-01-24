use crate::prelude::*;
use crate::{ArrayBuffer,InputAttribute,ElementBuffer,Input,OwlError};
use crate::oxidised_bindings as ox;

pub use ox::{ FloatVertexFormat, IntegralVertexFormat, IntegralDataType, DataTypeSize3, DataTypeSize4, DataTypeSizeBgra, DataTypeUnsized };

pub use crate::traits::Bytes;

pub struct AttributePointer<'a, T: ToByteVec> {
    pub buffer: &'a ArrayBuffer<T>,
    pub stride: Bytes,
    pub offset: Bytes,
}

pub struct VertexArray<E: ToByteVec> {
    inner: ox::VertexArray,
    max_inputs: u8,
    pub(crate) inputs: Vec<Input>,
    pub(crate) elements: Option<ElementBuffer<E>>,
}
#[allow(clippy::must_use_candidate)]
#[allow(clippy::return_self_not_must_use)]
impl<T: ToByteVec> VertexArray<T> {
    // INVARIANT: will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    pub fn new() -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self {
            inner: ox::gen_vertex_array(),
            max_inputs: ox::get_uint(ox::UIntParameter::MaxVertexAttribs) as u8,
            inputs: Vec::new(),
            elements: None
        }
    }
    pub fn with_indices(mut self, buffer: ElementBuffer<T>) -> Self {
        self.bind();
        buffer.bind();
        self.elements = Some(buffer);
        self
    }
    /// # Panics
    ///
    /// This function should never panic. If it does, this is a bug.
    ///
    /// # Errors
    ///
    /// This function will return an error if the maximum number of inputs is exceeded.
    pub fn with_input<U: ToByteVec>(mut self, attribute: InputAttribute, pointer: AttributePointer<U>) -> Result<Self,OwlError> {
        // max inputs should never be allowed to be >= 256
        #[allow(clippy::cast_possible_truncation)]
        let next_index = self.inputs.len() as u8;
        if next_index < self.max_inputs {
            self.bind();
            self.inputs.push(Input::new(next_index, attribute, pointer));
            ox::enable_vertex_attrib_array(next_index)
                .expect("vertex array bound, and next_index <= max_indices");
            Ok(self)
        } else {
            Err(OwlError::custom("maximum inputs reached"))
        }
    }
    pub(crate) fn bind(&self) {
        ox::bind_vertex_array(Some(self.inner))
            .expect("vertex array should not be deleted yet");
    }
    fn unbind() {
        ox::bind_vertex_array(None)
            .expect("binding 0 always succeeds");
    }
}

impl<T: ToByteVec> Default for VertexArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ToByteVec> Drop for VertexArray<T> {
    fn drop(&mut self) {
        ox::delete_vertex_array(self.inner);
    }
}
