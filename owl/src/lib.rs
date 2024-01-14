#![allow(dead_code)]

use raw_gl_context::GlContext;

mod safe_bindings;
mod traits;
pub use traits::*;
pub use trait_derives::*;
mod oxidised_bindings;
pub(crate) use oxidised_bindings as ox;

pub fn load_proc(context: &GlContext) {
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
}

mod errors;
pub use errors::*;
mod buffers;
pub use buffers::*;
mod vertex_arrays;
pub use vertex_arrays::*;

//
// #[derive(Debug)]
// pub enum Error {
//     MaximumExceeded(String),
//     ObjectCreationError(String),
//     ShaderError(ShaderError),
//     InsufficientMemory,
//     Unspecified,
// }
// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::MaximumExceeded(limit) => write!(f, "maximum number of {limit} exceeded"),
//             Self::ObjectCreationError(object) => write!(f, "could not create {object}"),
//             Self::ShaderError(se) => write!(f, "shader error: {se}"),
//             Self::InsufficientMemory => write!(f, "insufficient memory to complete operation"),
//             Self::Unspecified => write!(f,"unspecified error occured"),
//         }
//     }
// }
// impl std::error::Error for Error {}
//
// pub enum TypedData {
//     Float32(Vec<f32>),
//     U32(Vec<u32>),
// }
// impl TypedData {
//     fn type_size(&self) -> usize {
//         match self {
//             TypedData::Float32(_) => std::mem::size_of::<f32>(),
//             TypedData::U32(_) => std::mem::size_of::<u32>(),
//
//         }
//     }
//     fn gl_type(&self) -> gl::types::GLenum {
//         match self {
//             TypedData::Float32(_) => gl::FLOAT,
//             TypedData::U32(_) => gl::UNSIGNED_INT,
//         }
//     }
//     unsafe fn get_size_and_ptr(&self) -> (usize, *const std::ffi::c_void) {
//         match self {
//             TypedData::Float32(floats) => (floats.len() * self.type_size(), floats.as_ptr() as _),
//             TypedData::U32(uints) => (uints.len() * self.type_size(), uints.as_ptr() as _),
//         }
//     }
// }
//
//
// pub enum BufferUsage {
//     StaticDraw = gl::STATIC_DRAW as isize
// }
// pub struct ArrayBuffer {
//     id: u32,
//     data: TypedData
// }
// impl ArrayBuffer {
//     fn new() -> Self {
//         let mut id = 0;
//         // cf
//         safe_bindings::GenBuffer(&mut id);
//         ArrayBuffer { id, data: TypedData::Float32(Vec::new()) }
//     }
//     // None => Insufficient Memory
//     pub fn new_data(data: TypedData, usage_pattern: BufferUsage) -> Option<Self> {
//         let new_buffer = ArrayBuffer { data, ..Self::new() };
//         new_buffer.bind();
//         unsafe {
//         let (size, ptr) = new_buffer.data.get_size_and_ptr();
//             gl::BufferData(gl::ARRAY_BUFFER,
//                 size as isize,
//                 ptr,
//                 usage_pattern as u32);
//         }
//         // safe_bindings::BufferData(safe_bindings::BufferTypes::Array, new_buffer.data, usage)
//         if let Some(_) = safe_bindings::GetError() {
//             None
//         } else {
//             Some(new_buffer)
//         }
//     }
//     fn type_size(&self) -> usize {
//         self.data.type_size()
//     }
//     fn bind(&self) {
//         safe_bindings::BindBuffer(safe_bindings::BufferType::Array, self.id)
//     }
//     // pub fn unbind(&self) {
//     //  safe_bindings::BindBuffer(safe_bindings::BufferTypes::Array, 0)
//     // }
// }
//
// struct ElementBuffer {
//     id: u32,
//     data: TypedData
// }
// impl ElementBuffer {
//     fn new() -> Self {
//         let mut id = 0;
//         safe_bindings::GenBuffer(&mut id);
//         Self { id, data: TypedData::U32(Vec::new()) }
//     }
//     // None => Insufficient Memory
//     // TODO: transform into Result
//     fn new_data (data: TypedData, usage_pattern: BufferUsage) -> Option<Self> {
//         let new_buffer = Self { data, ..Self::new() };
//         new_buffer.bind();
//         unsafe {
//     let (size, ptr) = new_buffer.data.get_size_and_ptr();
//             gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
//                 size as isize,
//                 ptr,
//                 usage_pattern as u32);
//         }
//         // safe_bindings::BufferData(safe_bindings::BufferTypes::Array, new_buffer.data, usage)
//         if let Some(_) = safe_bindings::GetError() {
//             None
//         } else {
//             Some(new_buffer)
//         }
//     }
//     fn gl_type(&self) -> gl::types::GLenum {
//         self.data.gl_type()
//     }
//     // fn type_size(&self) -> usize {
//     //     self.data.type_size()
//     // }
//     fn bind(&self) {
//         safe_bindings::BindBuffer(safe_bindings::BufferType::ElementArray, self.id)
//     }
//     // pub fn unbind(&self) {
//     //    safe_bindings::BindBuffer(safe_bindings::BufferType::ElementArray, 0)
//     // }
// }
//
// pub struct VertexArray<'a> {
//     id: u32,
//     vertex_attributes: Vec<Input<'a>>,
//     element_buffer: Option<ElementBuffer>,
// }
// impl<'a> VertexArray<'a> {
//     pub fn new() -> Self {
//         let mut id = 0;
//         safe_bindings::GenVertexArray(&mut id);
//         VertexArray {
//             id,
//             vertex_attributes: Vec::new(),
//             element_buffer: None,
//         }
//     }
//     pub fn add_input_from_buffer<'b: 'a>(&mut self, buffer: &'b ArrayBuffer, attribute: Attribute,
//         normalised: bool, stride: u32, offset: usize) -> Result<(),Error> {
//         self.bind();
//         buffer.bind();
//         // buffer bound ==> safe
//         unsafe {
//             // usize -> u8: never more than ~16 vertex attributes
//             Input::new(buffer, self.vertex_attributes.len().try_into().unwrap(),
//                 attribute, normalised, stride, offset)
//                 .map(|input| {
//                     // unsafe
//                     safe_bindings::EnableVertexAttribArray(input.index);
//                     self.vertex_attributes.push(input)
//                 })
//                 .ok_or(Error::MaximumExceeded("vertex attributes".to_owned()))
//         }
//     }
//     pub fn add_element_buffer_data(&mut self, data: TypedData, usage_pattern: BufferUsage) -> Result<(),Error>{
//         self.bind();
//         self.element_buffer = match ElementBuffer::new_data(data, usage_pattern) {
//             Some(eb) => {
//                 eb.bind();
//                 Some(eb)
//             },
//             None => return Err(Error::InsufficientMemory),
//         };
//         Ok(())
//     }
//     fn bind(&self) {
//         safe_bindings::BindVertexArray(self.id)
//     }
//     // fn unbind(&self) {
//     //      safe_bindings::BindVertexArray(0)
//     // }
// }
//
// #[derive(Clone)]
// pub enum Attribute {
//     Bool(String),
//     Int(String),
//     Float(String),
//     Vec2(String),
//     Vec3(String),
//     Vec4(String),
//     BVec2(String),
//     BVec3(String),
//     BVec4(String),
//     IVec2(String),
//     IVec3(String),
//     IVec4(String),
//     Mat2(String),
//     Mat3(String),
//     Mat4(String),
// }
// impl Attribute {
//     fn get_size(&self) -> u32 {
//         match self {
//             Self::Bool(_) => 1,
//             Self::Int(_) => 1,
//             Self::Float(_) => 1,
//             Self::Vec2(_) => 2,
//             Self::Vec3(_) => 3,
//             Self::Vec4(_) => 4,
//             Self::BVec2(_) => 2,
//             Self::BVec3(_) => 3,
//             Self::BVec4(_) => 4,
//             Self::IVec2(_) => 2,
//             Self::IVec3(_) => 3,
//             Self::IVec4(_) => 4,
//             Self::Mat2(_) => 4,
//             Self::Mat3(_) => 9,
//             Self::Mat4(_) => 16,
//         }
//     }
//     fn get_gl_type(&self) -> gl::types::GLenum {
//         match self {
//             Self::Bool(_) => gl::BOOL,
//             Self::Int(_) => gl::INT,
//             Self::Float(_) => gl::FLOAT,
//             Self::Vec2(_) => gl::FLOAT,
//             Self::Vec3(_) => gl::FLOAT,
//             Self::Vec4(_) => gl::FLOAT,
//             Self::BVec2(_) => gl::BOOL,
//             Self::BVec3(_) => gl::BOOL,
//             Self::BVec4(_) => gl::BOOL,
//             Self::IVec2(_) => gl::INT,
//             Self::IVec3(_) => gl::INT,
//             Self::IVec4(_) => gl::INT,
//             Self::Mat2(_) => gl::FLOAT,
//             Self::Mat3(_) => gl::FLOAT,
//             Self::Mat4(_) => gl::FLOAT,
//         }
//     }
//     fn to_string(&self) -> String {
//         self.clone().into()
//     }
// }
// impl From<Attribute> for String {
//     fn from(value: Attribute) -> Self {
//         match value {
//             Attribute::Bool(s) => format!("bool {}",s),
//             Attribute::Int(s) => format!("int {}",s),
//             Attribute::Float(s) => format!("float {}",s),
//             Attribute::Vec2(s) => format!("vec2 {}",s),
//             Attribute::Vec3(s) => format!("vec3 {}",s),
//             Attribute::Vec4(s) => format!("vec3 {}",s),
//             Attribute::BVec2(s) => format!("bvec2 {}",s),
//             Attribute::BVec3(s) => format!("bvec2 {}",s),
//             Attribute::BVec4(s) => format!("bvec2 {}",s),
//             Attribute::IVec2(s) => format!("bvec2 {}",s),
//             Attribute::IVec3(s) => format!("bvec2 {}",s),
//             Attribute::IVec4(s) => format!("bvec2 {}",s),
//             Attribute::Mat2(s) => format!("bvec2 {}",s),
//             Attribute::Mat3(s) => format!("bvec2 {}",s),
//             Attribute::Mat4(s) => format!("bvec2 {}",s),
//         }
//     }
// }
// struct Input<'b> {
//     buffer: &'b ArrayBuffer,
//     index: u8,
//     attribute: Attribute,
//     // enabled: bool,
// }
// impl<'b> Input<'b> {
//     // ensure that a buffer is bound before calling
//     // None ==> reached max vertex attributes
//     unsafe fn new(buffer: &'b ArrayBuffer, index: u8, attribute: Attribute, normalised: bool, stride: u32, offset: usize)
//         -> Option<Input<'b>> {
//         let mut max_vertex_attributes = 0;
//         unsafe {
//             safe_bindings::GetIntegerv(safe_bindings::Parameter::MaxVertexAttribs,
//             std::slice::from_mut(&mut max_vertex_attributes));
//         }
//         if index as i32 >= max_vertex_attributes {
//             None
//         } else {
//             unsafe {
//                 // As: index <= max <= max_u32, size < 5 < max_i32, stride*primitive_size < max i32
//                 gl::VertexAttribPointer(index as u32,
//                     attribute.get_size() as i32, attribute.get_gl_type(), normalised.as_gl(),
//                     (stride as usize * buffer.type_size()) as i32, offset as *const _);
//             }
//             // safe_bindings::VertexAttribPointer(index, attribute.get_size().into(), attribute.get_type(), normalised, stride * buffer.type_size(), offset);
//             Some(Self { buffer, index, attribute, })
//         }
//     }
// }
//
// pub enum Pipe {
//     VertexToFragment(Attribute),
// }
//
// #[derive(Debug)]
// pub enum ShaderError {
//     IncorrectVersion,
//     SourceContainsNullBytes(usize),
//     CompilationError(String),
// }
// impl std::fmt::Display for ShaderError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::IncorrectVersion => write!(f,
//                 r#"incorrect glsl version supplied, currently supported versions:
//                 430"#
//             ),
//             Self::SourceContainsNullBytes(n) => write!(f,
//                 "source code cannot contain null bytes, null byte(s) at index: {n}"),
//             Self::CompilationError(e) => write!(f,
//                 "failed to compile with the following error(s):\n{}", e)
//         }
//     }
// }
// impl std::error::Error for ShaderError {}
//
// use std::ffi::CString;
// struct VertexShader {
//     id: u32,
//     body: CString,
// }
// struct FragmentShader {
//     id: u32,
//     body: CString,
//     out_colour: String,
// }
// pub struct ShaderPipeline {
//     version: u32,
//     vertex_shader: VertexShader,
//     fragment_shader: FragmentShader,
//     inputs: Vec<(usize, Attribute)>
// }
// impl ShaderPipeline {
//     pub fn new(version: u32) -> Result<Self, Error> {
//         if ![430].contains(&version) {
//             return Err(Error::ShaderError(ShaderError::IncorrectVersion))
//         }
//         let vertex_id = safe_bindings::CreateShader(safe_bindings::ShaderType::Vertex);
//         let fragment_id = safe_bindings::CreateShader(safe_bindings::ShaderType::Fragment);
//         if vertex_id == 0 || fragment_id == 0 {
//             return Err(Error::ObjectCreationError("vertex or fragment shader".to_owned()))
//         }
//         Ok(Self {
//             version,
//             // empty strings: no null bytes
//             vertex_shader: VertexShader { id: vertex_id, body: CString::new("").unwrap() },
//             fragment_shader: FragmentShader {
//                 id: fragment_id, body: CString::new("").unwrap(), out_colour: String::new() },
//             inputs: Vec::new()
//         })
//     }
//     pub fn with_vertex_body(self, body: &str) -> Result<Self,ShaderError> {
//         match CString::new(body) {
//             Ok(body) => Ok(Self {vertex_shader: VertexShader { body, ..self.vertex_shader }, ..self} ),
//             Err(e) => Err(ShaderError::SourceContainsNullBytes(e.nul_position()))
//         }
//     }
//     pub fn with_fragment_body(self, body: &str, out_colour_name: &str) -> Result<Self,ShaderError> {
//         match CString::new(body) {
//             Ok(body) => Ok(Self {fragment_shader: FragmentShader {
//                 body, out_colour: out_colour_name.to_owned(), ..self.fragment_shader },
//                 ..self} ),
//             Err(e) => Err(ShaderError::SourceContainsNullBytes(e.nul_position()))
//         }
//     }
//     pub fn with_inputs_from_vertex_array(self, vao: &VertexArray) -> Self {
//         // let input_string: String = vao.vertex_attributes.iter().enumerate()
//         //     .fold("".to_owned(),
//         //         | accumulator, (index, attribute)| accumulator + &format!("layout (location = {index}) in {};\n",
//         //         attribute.glsl()));
//         Self { inputs: vao.vertex_attributes.iter().enumerate()
//             .map(|(index, input)| (index, input.attribute.clone()))
//             .collect(), ..self }
//     }
//     pub fn compile(self) -> Result<ShaderProgram,Error> {
//         // vertex shader
//         let vertex_source = CString::new([
//             format!("#version {} core\n", self.version).as_bytes(),
//             self.inputs.iter().fold("".to_owned(),
//                 |acc,(index,attribute)| acc + &format!("layout (location = {index}) in {};\n",
//                     attribute.to_string())).as_bytes(),
//             self.vertex_shader.body.as_bytes()
//         // formatted string literals: no null, vertex_shader.body: CString ==> no nulls
//         ].concat()).unwrap();
//         safe_bindings::ShaderSource(self.vertex_shader.id, &[vertex_source]);
//         safe_bindings::CompileShader(self.vertex_shader.id);
//         if !Self::shader_succeeded(self.vertex_shader.id) {
//             return Err(Error::ShaderError(ShaderError::CompilationError(
//                 Self::shader_compile_log(self.vertex_shader.id)
//             )));
//         }
//         let fragment_source = CString::new([
//             format!("#version {} core\n", self.version).as_bytes(),
//             format!("out vec4 {};\n", self.fragment_shader.out_colour).as_bytes(),
//             self.fragment_shader.body.as_bytes(),
//         // formatted string literals: no null, fragment_shader.body: CString ==> no nulls
//         ].concat()).unwrap();
//         safe_bindings::ShaderSource(self.fragment_shader.id, &[fragment_source]);
//         safe_bindings::CompileShader(self.fragment_shader.id);
//         if !Self::shader_succeeded(self.fragment_shader.id) {
//             return Err(Error::ShaderError(ShaderError::CompilationError(
//                 Self::shader_compile_log(self.fragment_shader.id)
//             )));
//         }
//         let program_id = safe_bindings::CreateProgram();
//         if program_id == 0 {
//             return Err(Error::ObjectCreationError("shader program".to_owned()))
//         }
//         safe_bindings::AttachShader(program_id, self.vertex_shader.id);
//         safe_bindings::AttachShader(program_id, self.fragment_shader.id);
//         safe_bindings::LinkProgram(program_id);
//         safe_bindings::DetachShader(program_id, self.vertex_shader.id);
//         safe_bindings::DetachShader(program_id, self.fragment_shader.id);
//         safe_bindings::DeleteShader(self.vertex_shader.id);
//         safe_bindings::DeleteShader(self.fragment_shader.id);
//         Ok(ShaderProgram { id: program_id })
//     }
//     // ensure valid shader id
//     fn shader_succeeded(id: u32) -> bool {
//         let mut shader_success = 0;
//         safe_bindings::GetShaderiv(id, safe_bindings::ShaderParameter::CompileStatus, &mut shader_success);
//         (shader_success as gl::types::GLboolean).to_bool()
//     }
//     // ensure valid id
//     fn shader_compile_log(id: u32) -> String {
//         let mut log_length = 0;
//         safe_bindings::GetShaderiv(id, safe_bindings::ShaderParameter::InfoLogLength, &mut log_length);
//         let mut log = vec![0; log_length as usize];
//         safe_bindings::GetShaderInfoLog(id, &mut log, None);
//         // last character always \0, because of length check above
//         String::from_utf8(log.iter().take(log.len()-1).map(|c| *c as u8).collect())
//             .expect("ascii should always be valid UTF-8")
//     }
// }
//
// pub struct ShaderProgram {
//     pub id: u32
// }
// impl ShaderProgram {
//     fn use_me(&self) {
//         safe_bindings::UseProgram(self.id)
//     }
// }
// impl Drop for ShaderProgram {
//     fn drop(&mut self) {
//         safe_bindings::DeleteProgram(self.id)
//     }
// }
//
// enum MeshDrawMode {
//     Triangles,
//     TriangleStrip,
// }
// impl Into<gl::types::GLenum> for MeshDrawMode {
//     fn into(self) -> gl::types::GLenum {
//         match self {
//             Self::Triangles => gl::TRIANGLES,
//             Self::TriangleStrip => gl::TRIANGLE_STRIP,
//         }
//     }
// }
// pub struct Mesh {
//     pub start: usize,
//     pub count: usize,
// }
// impl Mesh {
//     fn draw(&self, mode: MeshDrawMode, vertex_array_object: &VertexArray, shader_program: &ShaderProgram) {
//         shader_program.use_me();
//         vertex_array_object.bind();
//         match vertex_array_object.element_buffer {
//             Some(ref eb) => unsafe { gl::DrawElements(mode.into(), self.count as i32, eb.gl_type(),
//                 self.start as *const _) },
//             // safe_bindings::DrawElements(safe_bindings::DrawMode::.., self.count,
//             //     safe_bindings::IndexType::.., self.start);
//             None => unsafe { gl::DrawArrays(mode.into(), self.start as i32, self.count as i32) }
//             // safe_bindings::DrawArrays(safe_bindings::DrawMode::.., self.count, self.start)
//         }
//     }
//     pub fn draw_triangles(&self, vertex_array_object: &VertexArray, shader_program: &ShaderProgram) {
//         self.draw(MeshDrawMode::Triangles, vertex_array_object, shader_program)
//     }
//     pub fn draw_triangle_strip(&self, vertex_array_object: &VertexArray, shader_program: &ShaderProgram) {
//         self.draw(MeshDrawMode::TriangleStrip, vertex_array_object, shader_program)
//     }
// }
//
// pub struct Colour(pub f32, pub f32, pub f32, pub f32);
// pub fn colour_clear(colour: Colour) {
//     oxgl::clear_colour(colour.0, colour.1, colour.2, colour.3);
//     safe_bindings::Clear(safe_bindings::ClearFlags::ColourBuffer);
// }
//
// trait ToGLBool {
//     fn as_gl(&self) -> gl::types::GLboolean;
// }
// impl ToGLBool for bool {
//     fn as_gl(&self) -> gl::types::GLboolean {
//         match self {
//             true => gl::TRUE,
//             false => gl::FALSE
//         }
//     }
// }
// trait ToBool {
//     fn to_bool(self) -> bool;
// }
// // FIXME: bad fucking design. any enum other than `GL_TRUE` defaults to false
// impl ToBool for gl::types::GLboolean {
//     fn to_bool(self) -> bool {
//         self == gl::TRUE
//     }
// }
