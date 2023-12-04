use raw_gl_context::GlContext;
use gl;

pub fn load_proc(context: &GlContext) {
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
}

#[derive(Debug)]
pub enum Error {
    MaximumExceeded(String),
    ObjectCreationError(String),
    ShaderError(ShaderError),
    InsufficientMemory,
    Unspecified,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaximumExceeded(limit) => write!(f, "maximum number of {limit} exceeded"),
            Self::ObjectCreationError(object) => write!(f, "could not create {object}"),
            Self::ShaderError(se) => write!(f, "shader error: {se}"),
            Self::InsufficientMemory => write!(f, "insufficient memory to complete operation"),
            Self::Unspecified => write!(f,"unspecified error occured"),
        }
    }
}
impl std::error::Error for Error {}

pub enum TypedData {
    Float32(Vec<f32>),
    U32(Vec<u32>),
}
impl TypedData {
    fn type_size(&self) -> usize {
        match self {
            TypedData::Float32(_) => std::mem::size_of::<f32>(),
            TypedData::U32(_) => std::mem::size_of::<u32>(),

        }
    }
    fn gl_type(&self) -> gl::types::GLenum {
        match self {
            TypedData::Float32(_) => gl::FLOAT,
            TypedData::U32(_) => gl::UNSIGNED_INT,
        }
    }
    unsafe fn get_size_and_ptr(&self) -> (usize, *const std::ffi::c_void) {
        match self {
            TypedData::Float32(floats) => (floats.len() * self.type_size(), floats.as_ptr() as _),
            TypedData::U32(uints) => (uints.len() * self.type_size(), uints.as_ptr() as _),
        }
    }
}


pub enum BufferUsage {
    StaticDraw = gl::STATIC_DRAW as isize
}
pub struct ArrayBuffer {
    id: u32,
    data: TypedData
}
impl ArrayBuffer {
    fn new() -> Self {
        let mut id = 0;
        // cf
        unsafe { gl::GenBuffers(1, &mut id) };
        assert_ne!(id, 0, "failed to generate buffer");
        ArrayBuffer { id, data: TypedData::Float32(Vec::new()) }
    }
    // None => Insufficient Memory
    pub fn new_data(data: TypedData, usage_pattern: BufferUsage) -> Option<Self> {
        let new_buffer = ArrayBuffer { data, ..Self::new() };
        new_buffer.bind();
        unsafe {
        let (size, ptr) = new_buffer.data.get_size_and_ptr();
            gl::BufferData(gl::ARRAY_BUFFER,
                size as isize,
                ptr,
                usage_pattern as u32);
        }
        if unsafe { gl::GetError() } != 0 {
            None
        } else {
            Some(new_buffer)
        }
    }
    fn type_size(&self) -> usize {
        self.data.type_size()
    }
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) };
    }
    // pub fn unbind(&self) {
    //     unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    // }
}

struct ElementBuffer {
    id: u32,
    data: TypedData
}
impl ElementBuffer {
    fn new() -> Self {
        let mut id = 0;
        // cf
        unsafe { gl::GenBuffers(1, &mut id) };
        assert_ne!(id, 0, "failed to generate buffer");
        Self { id, data: TypedData::U32(Vec::new()) }
    }
    // None => Insufficient Memory
    fn new_data (data: TypedData, usage_pattern: BufferUsage) -> Option<Self> {
        let new_buffer = Self { data, ..Self::new() };
        new_buffer.bind();
        unsafe {
        let (size, ptr) = new_buffer.data.get_size_and_ptr();
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                size as isize,
                ptr,
                usage_pattern as u32);
        }
        if unsafe { gl::GetError() } != 0 {
            None
        } else {
            Some(new_buffer)
        }
    }
    fn gl_type(&self) -> gl::types::GLenum {
        self.data.gl_type()
    }
    // fn type_size(&self) -> usize {
    //     self.data.type_size()
    // }
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id) };
    }
    // pub fn unbind(&self) {
    //     unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
    // }
}

pub struct VertexArray<'a> {
    id: u32,
    vertex_attributes: Vec<Input<'a>>,
    element_buffer: Option<ElementBuffer>,
}
impl<'a> VertexArray<'a> {
    pub fn new() -> Self {
        let mut id = 0;
        // cf
        unsafe { gl::GenVertexArrays(1, &mut id) };
        // assert_ne!(id, 0, "failed to create vertex array");
        VertexArray {
            id,
            vertex_attributes: Vec::new(),
            element_buffer: None,
        }
    }
    pub fn add_input_from_buffer<'b: 'a>(&mut self, buffer: &'b ArrayBuffer, attribute: Attribute,
        normalised: bool, stride: u32, offset: usize) -> Result<(),Error> {
        self.bind();
        buffer.bind();
        // buffer bound ==> safe
        unsafe {
            // usize -> u8: never more than ~16 vertex attributes
            Input::new(buffer, self.vertex_attributes.len().try_into().unwrap(),
                attribute, normalised, stride, offset)
                .map(|input| {
                    // unsafe
                    gl::EnableVertexAttribArray(input.index as u32);
                    self.vertex_attributes.push(input)
                })
                .ok_or(Error::MaximumExceeded("vertex attributes".to_owned()))
        }
    }
    pub fn add_element_buffer_data(&mut self, data: TypedData, usage_pattern: BufferUsage) -> Result<(),Error>{
        self.bind();
        self.element_buffer = match ElementBuffer::new_data(data, usage_pattern) {
            Some(eb) => {
                eb.bind();
                Some(eb)
            },
            None => return Err(Error::InsufficientMemory),
        };
        Ok(())
    }
    fn bind(&self) {
        // cf
        unsafe { gl::BindVertexArray(self.id) };
    }
    // fn unbind(&self) {
    //     // cf
    //     unsafe { gl::BindVertexArray(0) };
    // }
}

#[derive(Clone)]
pub enum Attribute {
    Bool(String),
    Int(String),
    Float(String),
    Vec2(String),
    Vec3(String),
    Vec4(String),
    BVec2(String),
    BVec3(String),
    BVec4(String),
    IVec2(String),
    IVec3(String),
    IVec4(String),
    Mat2(String),
    Mat3(String),
    Mat4(String),
}
impl Attribute {
    fn get_size(&self) -> u32 {
        match self {
            Self::Bool(_) => 1,
            Self::Int(_) => 1,
            Self::Float(_) => 1,
            Self::Vec2(_) => 2,
            Self::Vec3(_) => 3,
            Self::Vec4(_) => 4,
            Self::BVec2(_) => 2,
            Self::BVec3(_) => 3,
            Self::BVec4(_) => 4,
            Self::IVec2(_) => 2,
            Self::IVec3(_) => 3,
            Self::IVec4(_) => 4,
            Self::Mat2(_) => 4,
            Self::Mat3(_) => 9,
            Self::Mat4(_) => 16,
        }
    }
    fn get_gl_type(&self) -> gl::types::GLenum {
        match self {
            Self::Bool(_) => gl::BOOL,
            Self::Int(_) => gl::INT,
            Self::Float(_) => gl::FLOAT,
            Self::Vec2(_) => gl::FLOAT,
            Self::Vec3(_) => gl::FLOAT,
            Self::Vec4(_) => gl::FLOAT,
            Self::BVec2(_) => gl::BOOL,
            Self::BVec3(_) => gl::BOOL,
            Self::BVec4(_) => gl::BOOL,
            Self::IVec2(_) => gl::INT,
            Self::IVec3(_) => gl::INT,
            Self::IVec4(_) => gl::INT,
            Self::Mat2(_) => gl::FLOAT,
            Self::Mat3(_) => gl::FLOAT,
            Self::Mat4(_) => gl::FLOAT,
        }
    }
    fn to_string(&self) -> String {
        self.clone().into()
    }
}
impl From<Attribute> for String {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Bool(s) => format!("bool {}",s),
            Attribute::Int(s) => format!("int {}",s),
            Attribute::Float(s) => format!("float {}",s),
            Attribute::Vec2(s) => format!("vec2 {}",s),
            Attribute::Vec3(s) => format!("vec3 {}",s),
            Attribute::Vec4(s) => format!("vec3 {}",s),
            Attribute::BVec2(s) => format!("bvec2 {}",s),
            Attribute::BVec3(s) => format!("bvec2 {}",s),
            Attribute::BVec4(s) => format!("bvec2 {}",s),
            Attribute::IVec2(s) => format!("bvec2 {}",s),
            Attribute::IVec3(s) => format!("bvec2 {}",s),
            Attribute::IVec4(s) => format!("bvec2 {}",s),
            Attribute::Mat2(s) => format!("bvec2 {}",s),
            Attribute::Mat3(s) => format!("bvec2 {}",s),
            Attribute::Mat4(s) => format!("bvec2 {}",s),
        }
    }
}
struct Input<'b> {
    buffer: &'b ArrayBuffer,
    index: u8,
    attribute: Attribute,
    // enabled: bool,
}
impl<'b> Input<'b> {
    // ensure that a buffer is bound before calling
    // None ==> reached max vertex attributes
    unsafe fn new(buffer: &'b ArrayBuffer, index: u8, attribute: Attribute, normalised: bool, stride: u32, offset: usize)
        -> Option<Input<'b>> {
        let mut max_vertex_attributes = 0;
        unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attributes) };
        if index as i32 >= max_vertex_attributes {
            None
        } else {
            unsafe {
                // As: index <= max <= max_u32, size < 5 < max_i32, stride*primitive_size < max i32
                gl::VertexAttribPointer(index as u32,
                    attribute.get_size() as i32, attribute.get_gl_type(), normalised.as_gl(),
                    (stride as usize * buffer.type_size()) as i32, offset as *const _);
            }
            Some(Self { buffer, index, attribute, })
        }
    }
}

pub enum Pipe {
    VertexToFragment(Attribute),
}

#[derive(Debug)]
pub enum ShaderError {
    IncorrectVersion,
    SourceContainsNullBytes(usize),
    CompilationError(String),
}
impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectVersion => write!(f,
                r#"incorrect glsl version supplied, currently supported versions:
                430"#
            ),
            Self::SourceContainsNullBytes(n) => write!(f,
                "source code cannot contain null bytes, null byte(s) at index: {n}"),
            Self::CompilationError(e) => write!(f,
                "failed to compile with the following error(s):\n{}", e)
        }
    }
}
impl std::error::Error for ShaderError {}

use std::ffi::CString;
struct VertexShader {
    id: u32,
    body: CString,
}
struct FragmentShader {
    id: u32,
    body: CString,
    out_colour: String,
}
pub struct ShaderPipeline {
    version: u32,
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
    inputs: Vec<(usize, Attribute)>
}
impl ShaderPipeline {
    pub fn new(version: u32) -> Result<Self, Error> {
        if ![430].contains(&version) {
            return Err(Error::ShaderError(ShaderError::IncorrectVersion))
        }
        let vertex_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let fragment_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        if vertex_id == 0 || fragment_id == 0 {
            return Err(Error::ObjectCreationError("vertex or fragment shader".to_owned()))
        }
        Ok(Self {
            version,
            // empty strings: no null bytes
            vertex_shader: VertexShader { id: vertex_id, body: CString::new("").unwrap() },
            fragment_shader: FragmentShader {
                id: fragment_id, body: CString::new("").unwrap(), out_colour: String::new() },
            inputs: Vec::new()
        })
    }
    pub fn with_vertex_body(self, body: &str) -> Result<Self,ShaderError> {
        match CString::new(body) {
            Ok(body) => Ok(Self {vertex_shader: VertexShader { body, ..self.vertex_shader }, ..self} ),
            Err(e) => Err(ShaderError::SourceContainsNullBytes(e.nul_position()))
        }
    }
    pub fn with_fragment_body(self, body: &str, out_colour_name: &str) -> Result<Self,ShaderError> {
        match CString::new(body) {
            Ok(body) => Ok(Self {fragment_shader: FragmentShader {
                body, out_colour: out_colour_name.to_owned(), ..self.fragment_shader },
                ..self} ),
            Err(e) => Err(ShaderError::SourceContainsNullBytes(e.nul_position()))
        }
    }
    pub fn with_inputs_from_vertex_array(self, vao: &VertexArray) -> Self {
        // let input_string: String = vao.vertex_attributes.iter().enumerate()
        //     .fold("".to_owned(),
        //         | accumulator, (index, attribute)| accumulator + &format!("layout (location = {index}) in {};\n",
        //         attribute.glsl()));
        Self { inputs: vao.vertex_attributes.iter().enumerate()
            .map(|(index, input)| (index, input.attribute.clone()))
            .collect(), ..self }
    }
    pub fn compile(self) -> Result<ShaderProgram,Error> {
        // vertex shader
        let vertex_source = CString::new([
            format!("#version {} core\n", self.version).as_bytes(),
            self.inputs.iter().fold("".to_owned(),
                |acc,(index,attribute)| acc + &format!("layout (location = {index}) in {};\n",
                    attribute.to_string())).as_bytes(),
            self.vertex_shader.body.as_bytes()
        // formatted string literals: no null, vertex_shader.body: CString ==> no nulls
        ].concat()).unwrap();
        unsafe {
            gl::ShaderSource(self.vertex_shader.id, 1, &vertex_source.as_ptr(), std::ptr::null());
            gl::CompileShader(self.vertex_shader.id);
        }
        if unsafe { !Self::shader_succeeded(self.vertex_shader.id) } {
            return Err(Error::ShaderError(ShaderError::CompilationError(
                unsafe { Self::shader_compile_log(self.vertex_shader.id) }
            )));
        }
        let fragment_source = CString::new([
            format!("#version {} core\n", self.version).as_bytes(),
            format!("out vec4 {};\n", self.fragment_shader.out_colour).as_bytes(),
            self.fragment_shader.body.as_bytes(),
        // formatted string literals: no null, fragment_shader.body: CString ==> no nulls
        ].concat()).unwrap();
        unsafe {
            gl::ShaderSource(self.fragment_shader.id, 1, &fragment_source.as_ptr(), std::ptr::null());
            gl::CompileShader(self.fragment_shader.id);
        }
        if unsafe { !Self::shader_succeeded(self.fragment_shader.id) } {
            return Err(Error::ShaderError(ShaderError::CompilationError(
                unsafe { Self::shader_compile_log(self.fragment_shader.id) }
            )));
        }
        let program_id = unsafe { gl::CreateProgram() };
        if program_id == 0 {
            return Err(Error::ObjectCreationError("shader program".to_owned()))
        }
        // cf
        unsafe {
            gl::AttachShader(program_id, self.vertex_shader.id);
            gl::AttachShader(program_id, self.fragment_shader.id);
            gl::LinkProgram(program_id);
            gl::DetachShader(program_id, self.vertex_shader.id);
            gl::DetachShader(program_id, self.fragment_shader.id);
            gl::DeleteShader(self.vertex_shader.id);
            gl::DeleteShader(self.fragment_shader.id);
        }
        Ok(ShaderProgram { id: program_id })
    }
    // ensure valid shader id
    unsafe fn shader_succeeded(id: u32) -> bool {
        let mut shader_success = 0;
        unsafe {gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut shader_success)};
        (shader_success as gl::types::GLboolean).to_bool()
    }
    // ensure valid id
    unsafe fn shader_compile_log(id: u32) -> String {
        let mut log_length = 0;
        unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length) };
        let mut log = vec![0; log_length as usize];
        unsafe { gl::GetShaderInfoLog(id, log.len() as i32,
            std::ptr::null_mut(), log.as_mut_ptr()) };
        // cf: will always return valid ascii ==> always valid utf-8
        String::from_utf8(log.iter().take(log.len()-1).map(|c| *c as u8).collect()).unwrap()
    }
}

pub struct ShaderProgram {
    pub id: u32
}
impl ShaderProgram {
    fn use_me(&self) {
        unsafe {
            gl::UseProgram(self.id)
        }
    }
}
impl Drop for ShaderProgram {
    fn drop(&mut self) {
        // cf
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

enum MeshDrawMode {
    Triangles,
    TriangleStrip,
}
impl Into<gl::types::GLenum> for MeshDrawMode {
    fn into(self) -> gl::types::GLenum {
        match self {
            Self::Triangles => gl::TRIANGLES,
            Self::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}
pub struct Mesh {
    pub start: usize,
    pub count: usize,
}
impl Mesh {
    fn draw(&self, mode: MeshDrawMode, vertex_array_object: &VertexArray, shader_program: &ShaderProgram) {
        shader_program.use_me();
        vertex_array_object.bind();
        match vertex_array_object.element_buffer {
            Some(ref eb) => unsafe { gl::DrawElements(mode.into(), self.count as i32, eb.gl_type(),
                self.start as *const _) },
            None => unsafe { gl::DrawArrays(mode.into(), self.start as i32, self.count as i32) }
        }
    }
    pub fn draw_triangles(&self, vertex_array_object: &VertexArray, shader_program: &ShaderProgram) {
        self.draw(MeshDrawMode::Triangles, vertex_array_object, shader_program)
    }
    pub fn draw_triangle_strip(&self, vertex_array_object: &VertexArray, shader_program: &ShaderProgram) {
        self.draw(MeshDrawMode::TriangleStrip, vertex_array_object, shader_program)
    }
}

pub struct Colour(pub f32, pub f32, pub f32, pub f32);
pub fn colour_clear(colour: Colour) {
    unsafe {
        gl::ClearColor(colour.0, colour.1, colour.2, colour.3);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

trait ToGLBool {
    fn as_gl(&self) -> gl::types::GLboolean;
}
impl ToGLBool for bool {
    fn as_gl(&self) -> gl::types::GLboolean {
        match self {
            true => gl::TRUE,
            false => gl::FALSE
        }
    }
}
trait ToBool {
    fn to_bool(self) -> bool;
}
impl ToBool for gl::types::GLboolean {
    fn to_bool(self) -> bool {
        self == gl::TRUE
    }
}
