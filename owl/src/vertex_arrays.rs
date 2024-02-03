use crate::{prelude::*, MatInputAttributePointer, ThinInputAttribute};
use crate::{ArrayBuffer,ElementBuffer,Input,OwlError};
use crate::oxidised_bindings as ox;

pub use ox::{ FloatVertexFormat, IntegralVertexFormat, IntegralDataType, DataTypeSize3, DataTypeSize4, DataTypeSizeBgra, DataTypeUnsized };

pub use crate::traits::Bytes;

#[derive(Debug, Clone)]
pub struct AttributePointer<'a, T: ToByteVec> {
    pub buffer: &'a ArrayBuffer<T>,
    pub stride: Bytes,
    pub offset: Bytes,
}

#[derive(Debug, Clone)]
pub(crate) struct InputArray {
    pub(crate) container: Vec<Input>,
    capacity: u8,
    length: u8,
}

impl InputArray {
    fn new(capacity: u8) -> Self {
        Self {
            container: Vec::with_capacity(usize::from(capacity)),
            capacity,
            length: 0
        }
    }
    fn push<T: ToByteVec>(&mut self, attribute: ThinInputAttribute, pointer: AttributePointer<T>) -> Result<(),OwlError> {
        if self.length > self.capacity {
            Err(OwlError::custom("maximum inputs reached"))
        } else {
            self.length += 1;
            self.container.push(Input::new_thin(self.length, attribute, pointer ));
            ox::enable_vertex_attrib_array(self.length)
                .expect("vertex array bound, and next_index <= max_indices");
            Ok(())
        }
    }
    fn push_mat<T: ToByteVec>(&mut self, attribute_pointer: MatInputAttributePointer<T>) -> Result<(),OwlError> {
        let new_length = self.length + attribute_pointer.size();
        if new_length > self.capacity {
            Err(OwlError::custom("maximum inputs reached"))
        } else {
            self.length = new_length;
            self.container.push(Input::new_mat(self.length, attribute_pointer));
            for i in self.length..new_length {
                let index = i - 1;
                ox::enable_vertex_attrib_array(index)
                    .expect("vertex array bound, and next_index <= max_indices");
            }
            Ok(())
        }
    }
}

pub struct VertexArray<E: ToByteVec> {
    inner: ox::VertexArray,
    pub(crate) inputs: InputArray,
    pub(crate) elements: Option<ElementBuffer<E>>,
}
#[allow(clippy::must_use_candidate)]
#[allow(clippy::return_self_not_must_use)]
impl<T: ToByteVec> VertexArray<T> {
    // INVARIANT: will not be deleted until it is dropped
    // fewer calls can fail, reducing error handling, but they now "expect"
    pub fn new() -> Self {
        // CAST: should not exceed 255 (practically, always 16)
        #[allow(clippy::cast_possible_truncation)]
        Self {
            inner: ox::gen_vertex_array(),
            inputs: InputArray::new(u8::try_from(ox::get_uint(ox::UIntParameter::MaxVertexAttribs))
                .expect("practically always 16, should never exceed 255")),
            elements: None
        }
    }
    pub fn with_indices(mut self, buffer: ElementBuffer<T>) -> Self {
        self.bind();
        buffer.bind();
        self.elements = Some(buffer);
        self
    }
    /// # Errors
    ///
    /// This function will return an error if the maximum number of inputs is exceeded.
    pub fn with_input<U: ToByteVec>(mut self, attribute: ThinInputAttribute, pointer: AttributePointer<U>) -> Result<Self,OwlError> {
        self.bind();
        self.inputs.push(attribute, pointer)?;
        Ok(self)
    }
    /// # Errors
    ///
    /// This function will return an error if the maximum number of inputs is exceeded.
    pub fn with_input_mat<U: ToByteVec>(mut self, attribute: MatInputAttributePointer<T>) -> Result<Self, OwlError> {
        self.bind();
        self.inputs.push_mat(attribute)?;
        Ok(self)
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
