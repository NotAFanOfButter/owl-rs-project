use crate::prelude::*;
use crate::{ArrayBuffer,Attribute,ElementBuffer,Input,OwlError};
use crate::oxidised_bindings as ox;

pub use ox::{ VertexFormat, DataTypeSize3, DataTypeSize4, DataTypeSizeBgra, DataTypeUnsized };

type Bytes = usize;
pub struct AttributePointer<'a, T: ToByteVec> {
    pub buffer: &'a ArrayBuffer<T>,
    pub stride: Bytes,
    pub offset: Bytes,
    pub format: VertexFormat,
}

pub struct VertexArray<E: ToByteVec> {
    inner: ox::VertexArray,
    max_inputs: u8,
    pub(crate) inputs: Vec<Input>,
    pub(crate) elements: Option<ElementBuffer<E>>,
}
impl<T: ToByteVec> VertexArray<T> {
    // INVARIANT: will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    pub fn new() -> Self {
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
    pub fn with_input<U: ToByteVec>(mut self, attribute: Attribute, pointer: AttributePointer<U>) -> Result<Self,OwlError> {
        // max inputs should never be allowed to be >= 256
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
            .expect("vertex array should not be deleted yet")
    }
    fn unbind(&self) {
        ox::bind_vertex_array(None)
            .expect("binding 0 always succeeds")
    }
}
impl<T: ToByteVec> Drop for VertexArray<T> {
    fn drop(&mut self) {
        ox::delete_vertex_array(self.inner)
    }
}
