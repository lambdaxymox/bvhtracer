use crate::gl;
use crate::gl::types::{
    GLboolean, 
    GLchar, 
    GLenum, 
    GLfloat, 
    GLint, 
    GLubyte, 
    GLuint,
};
use glfw;
use glfw::{
    Context, 
    Glfw,
};

use std::ffi::{
    CStr, 
    CString,
};
use std::fs::{
    File,
};
use std::io::{
    Read, 
    BufReader,
};
use std::collections::{
    HashMap,
};
use std::sync::mpsc::Receiver;
use std::ptr;
use std::fmt;
use std::mem;
use std::path::Path;

// use log::{info, error};


// OpenGL extension constants.
pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;

// 256 Kilobytes.
const MAX_SHADER_LENGTH: usize = 262144;


#[inline]
pub fn glubyte_ptr_to_string(cstr: *const GLubyte) -> String {
    unsafe {
        CStr::from_ptr(cstr as *const i8).to_string_lossy().into_owned()
    }
}

#[inline]
pub fn gl_str(st: &str) -> CString {
    CString::new(st).unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GlRuntimeParameter {
    MaxCombinedTextureImageUnits = gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS as isize,
    MaxCubeMapTextureSize        = gl::MAX_CUBE_MAP_TEXTURE_SIZE as isize, 
    MaxDrawBuffers               = gl::MAX_DRAW_BUFFERS as isize,
    MaxFragmentUniformComponents = gl::MAX_FRAGMENT_UNIFORM_COMPONENTS as isize,
    MaxTextureImageUnits         = gl::MAX_TEXTURE_IMAGE_UNITS as isize,
    MaxTextureSize               = gl::MAX_TEXTURE_SIZE as isize,
    MaxVaryingFloats             = gl::MAX_VARYING_FLOATS as isize,
    MaxVertexAttributes          = gl::MAX_VERTEX_ATTRIBS as isize,
    MaxVertexTextureImageUnits   = gl::MAX_VERTEX_TEXTURE_IMAGE_UNITS as isize,
    MaxVertexUniformComponents   = gl::MAX_VERTEX_UNIFORM_COMPONENTS as isize,
    MaxViewportDimensions        = gl::MAX_VIEWPORT_DIMS as isize,
    Stereo                       = gl::STEREO as isize,
}

impl GlRuntimeParameter {
    pub const fn as_isize(self) -> isize {
        self as isize
    }
}

impl fmt::Display for GlRuntimeParameter {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let st = match *self {
            GlRuntimeParameter::MaxCombinedTextureImageUnits => "GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS",
            GlRuntimeParameter::MaxCubeMapTextureSize        => "GL_MAX_CUBE_MAP_TEXTURE_SIZE",
            GlRuntimeParameter::MaxDrawBuffers               => "GL_MAX_DRAW_BUFFERS",
            GlRuntimeParameter::MaxFragmentUniformComponents => "GL_MAX_FRAGMENT_UNIFORM_COMPONENTS",
            GlRuntimeParameter::MaxTextureImageUnits         => "GL_MAX_TEXTURE_IMAGE_UNITS",
            GlRuntimeParameter::MaxTextureSize               => "GL_MAX_TEXTURE_SIZE",
            GlRuntimeParameter::MaxVaryingFloats             => "GL_MAX_VARYING_FLOATS",
            GlRuntimeParameter::MaxVertexAttributes          => "GL_MAX_VERTEX_ATTRIBS",
            GlRuntimeParameter::MaxVertexTextureImageUnits   => "GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS",
            GlRuntimeParameter::MaxVertexUniformComponents   => "GL_MAX_VERTEX_UNIFORM_COMPONENTS",
            GlRuntimeParameter::MaxViewportDimensions        => "GL_MAX_VIEWPORT_DIMS",
            GlRuntimeParameter::Stereo                       => "GL_STEREO"
        };

        write!(formatter, "{}", st)
    }
}

/// A record containing a description of the GL capabilities on a local machine.
/// The contents of this record can be used for debugging OpenGL problems on
/// different machines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlRuntimeParameterSet {
    data: HashMap<GlRuntimeParameter, String>,
}

impl fmt::Display for GlRuntimeParameterSet {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(formatter, "GL Context Params:").unwrap();
        for (param, value) in self.data.iter() {
            writeln!(formatter, "{} = {}", param, value).unwrap();
        }
        writeln!(formatter)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GlslType {
    GlFloat           = gl::FLOAT as isize,
    GlFloatVec2       = gl::FLOAT_VEC2 as isize,
    GlFloatVec3       = gl::FLOAT_VEC3 as isize,
    GlFloatVec4       = gl::FLOAT_VEC4 as isize,
    GlInt             = gl::INT as isize,
    GlIntVec2         = gl::INT_VEC2 as isize,
    GlIntVec3         = gl::INT_VEC3 as isize,
    GlIntVec4         = gl::INT_VEC4 as isize,
    GlUnsignedInt     = gl::UNSIGNED_INT as isize,
    GlUnsignedIntVec2 = gl::UNSIGNED_INT_VEC2 as isize,
    GlUnsignedIntVec3 = gl::UNSIGNED_INT_VEC3 as isize,
    GlUnsignedIntVec4 = gl::UNSIGNED_INT_VEC4 as isize,
    GlBool            = gl::BOOL as isize,
    GlBoolVec2        = gl::BOOL_VEC2 as isize,
    GlBoolVec3        = gl::BOOL_VEC3 as isize,
    GlBoolVec4        = gl::BOOL_VEC4 as isize,
    GlFloatMat2       = gl::FLOAT_MAT2 as isize,
    GlFloatMat2x3     = gl::FLOAT_MAT2x3 as isize,
    GlFloatMat2x4     = gl::FLOAT_MAT2x4 as isize,
    GlFloatMat3       = gl::FLOAT_MAT3 as isize,
    GlFloatMat3x2     = gl::FLOAT_MAT3x2 as isize,
    GlFloatMat3x4     = gl::FLOAT_MAT3x4 as isize,
    GlFloatMat4       = gl::FLOAT_MAT4 as isize,
    GlFloatMat4x2     = gl::FLOAT_MAT4x2 as isize,
    GlFloatMat4x3     = gl::FLOAT_MAT4x3 as isize,
}

impl GlslType {
    /// Calculate the storage size in bytes of a GLSL data type.
    pub const fn size_of(self) -> usize {
        match self {
            GlslType::GlFloat           => 1 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatVec2       => 2 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatVec3       => 3 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatVec4       => 4 * mem::size_of::<GLfloat>(),
            GlslType::GlInt             => 1 * mem::size_of::<GLint>(),
            GlslType::GlIntVec2         => 2 * mem::size_of::<GLint>(),
            GlslType::GlIntVec3         => 3 * mem::size_of::<GLint>(),
            GlslType::GlIntVec4         => 4 * mem::size_of::<GLint>(),
            GlslType::GlUnsignedInt     => 1 * mem::size_of::<GLuint>(),
            GlslType::GlUnsignedIntVec2 => 2 * mem::size_of::<GLuint>(),
            GlslType::GlUnsignedIntVec3 => 3 * mem::size_of::<GLuint>(),
            GlslType::GlUnsignedIntVec4 => 4 * mem::size_of::<GLuint>(),
            GlslType::GlBool            => 1 * mem::size_of::<GLboolean>(),
            GlslType::GlBoolVec2        => 2 * mem::size_of::<GLboolean>(),
            GlslType::GlBoolVec3        => 3 * mem::size_of::<GLboolean>(),
            GlslType::GlBoolVec4        => 4 * mem::size_of::<GLboolean>(),
            GlslType::GlFloatMat2       => 4 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat2x3     => 6 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat2x4     => 8 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat3       => 9 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat3x2     => 6 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat3x4     => 12 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat4       => 16 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat4x2     => 8 * mem::size_of::<GLfloat>(),
            GlslType::GlFloatMat4x3     => 12 * mem::size_of::<GLfloat>(),
        }
    }
    
    pub const fn as_isize(self) -> isize {
        self as isize
    }
}

#[derive(Clone, Debug)]
pub enum ShaderCompilationError {
    ShaderNotFound(String),
    CouldNotParseShader(String),
    CouldNotCompileShader(String),
    CouldNotLinkShader,
    ShaderValidationFailed,
}

impl fmt::Display for ShaderCompilationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderCompilationError::ShaderNotFound(ref file_name) => {
                write!(formatter, "Could not open the shader file for reading: {}", file_name)
            }
            ShaderCompilationError::CouldNotParseShader(ref file_name) => {
                write!(formatter, "The shader file exists, but there was an error in reading it: {}", file_name)
            }
            ShaderCompilationError::CouldNotCompileShader(ref file_name) => {
                write!(formatter, "The shader could not be compiled: {}", file_name)
            }
            ShaderCompilationError::CouldNotLinkShader => {
                write!(formatter, "The shader program could not be linked.")
            }
            ShaderCompilationError::ShaderValidationFailed => {
                write!(formatter, "Shader validation failed.")
            }
        }
    }
}

/// A record containing all the relevant compilation log information for a
/// given GLSL shader compiled at run time.
pub struct ShaderLog {
    index: GLuint,
    log: String,
}

impl fmt::Display for ShaderLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Shader info log for GL index {}:", self.index).unwrap();
        writeln!(f, "{}", self.log)
    }
}

/// A record containing all the relevant compilation log information for a
/// given GLSL shader program compiled at run time.
pub struct ProgramLog {
    index: GLuint,
    log: String,
}

impl fmt::Display for ProgramLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Program info log for GL index {}:", self.index).unwrap();
        writeln!(f, "{}", self.log)
    }
}

/// A record for storing all the OpenGL state needed on the application side
/// of the graphics application in order to manage OpenGL and GLFW.
pub struct GlContext {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width: u32,
    pub height: u32,
    pub channel_depth: u32,
    pub running_time_seconds: f64,
    pub framerate_time_seconds: f64,
    pub frame_count: u32,
}

#[cfg(target_os = "macos")]
fn __init_glfw() -> Glfw {
    // Start a GL context and OS window using the GLFW helper library.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // We must place the window hints before creating the window because
    // glfw cannot change the properties of a window after it has been created.
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    glfw
}

#[cfg(target_os = "windows")]
fn __init_glfw() -> Glfw {
    // Start a GL context and OS window using the GLFW helper library.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // We must place the window hints before creating the window because
    // glfw cannot change the properties of a window after it has been created.
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    glfw
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn __init_glfw() -> Glfw {
    // Start a GL context and OS window using the GLFW helper library.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // We must place the window hints before creating the window because
    // glfw cannot change the properties of a window after it has been created.
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    glfw
}

/// Initialize a new OpenGL context and start a new GLFW window. 
pub fn init_gl(width: u32, height: u32) -> Result<GlContext, String> {
    // Start GL context and O/S window using the GLFW helper library.
    // info!("Starting GLFW");
    // info!("Using GLFW version {}", glfw::get_version_string());

    // Start a GL context and OS window using the GLFW helper library.
    let glfw = __init_glfw();

    // info!("Started GLFW successfully");
    let maybe_glfw_window = glfw.create_window(
        width, height, "Googly Blocks", glfw::WindowMode::Windowed
    );
    let (mut window, events) = match maybe_glfw_window {
        Some(tuple) => tuple,
        None => {
            // error!("Failed to create GLFW window");
            return Err(String::new());
        }
    };

    window.make_current();
    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_refresh_polling(true);
    window.set_size_polling(true);
    window.set_sticky_keys(true);

    // Load the OpenGl function pointers.
    gl::load_with(|symbol| { window.get_proc_address(symbol) as *const _ });

    // Get renderer and version information.
    let renderer = glubyte_ptr_to_string(unsafe { gl::GetString(gl::RENDERER) });
    // info!("Renderer: {}", renderer);

    let version = glubyte_ptr_to_string(unsafe { gl::GetString(gl::VERSION) });
    // info!("OpenGL version supported: {}", version);
    // info!("{}", gl_params());

    Ok(GlContext {
        glfw: glfw, 
        window: window, 
        events: events,
        width: width,
        height: height,
        channel_depth: 3,
        running_time_seconds: 0.0,
        framerate_time_seconds: 0.0,
        frame_count: 0,
    })
}

impl GlContext {
    /// Updates the timers in a GL context. It returns the elapsed time since the last call to
    /// `update_timers`.
    pub fn update_timers(&mut self) -> f64 {
        let current_seconds = self.glfw.get_time();
        let elapsed_seconds = current_seconds - self.running_time_seconds;
        self.running_time_seconds = current_seconds;

        elapsed_seconds
    }

    /// Update the framerate and display in the window titlebar.
    pub fn update_fps_counter(&mut self) {
        let current_time_seconds = self.glfw.get_time();
        let elapsed_seconds = current_time_seconds - self.framerate_time_seconds;
        if elapsed_seconds > 0.5 {
            self.framerate_time_seconds = current_time_seconds;
            let fps = self.frame_count as f64 / elapsed_seconds;
            self.window.set_title(&format!("OpenGL Window @ {:.2}", fps));
            self.frame_count = 0;
        }

        self.frame_count += 1;
    }

    /// Print out the GL capabilities on a local machine. This is handy for debugging
    /// OpenGL program problems on other people's machines.
    pub fn gl_params(&self) -> GlRuntimeParameterSet {
        let max_combined_texture_image_units = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxCombinedTextureImageUnits as u32, &mut raw_value);

            format!("{}", raw_value)
        };
        let max_cube_map_texture_size = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxCubeMapTextureSize as u32, &mut raw_value);
        
            format!("{}", raw_value)
        };
        let max_draw_buffers = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxDrawBuffers as u32, &mut raw_value);
      
            format!("{}", raw_value)
        };
        let max_fragment_uniform_components = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxFragmentUniformComponents as u32, &mut raw_value);
      
            format!("{}", raw_value)
        };
        let max_texture_image_units = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxTextureImageUnits as u32, &mut raw_value);
      
            format!("{}", raw_value)
        };
        let max_texture_size = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxTextureSize as u32, &mut raw_value);
    
            format!("{}", raw_value)
        };
        let max_varying_floats = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxVaryingFloats as u32, &mut raw_value);
      
            format!("{}", raw_value)
        };
        let max_vertex_attributes = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxVertexAttributes as u32, &mut raw_value);
      
            format!("{}", raw_value)
        };
        let max_vertex_texture_image_units = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxVertexTextureImageUnits as u32, &mut raw_value);
      
            format!("{}", raw_value)
        };
        let max_vertex_uniform_components = unsafe {
            let mut raw_value = 0;
            gl::GetIntegerv(GlRuntimeParameter::MaxVertexUniformComponents as u32, &mut raw_value);
    
            format!("{}", raw_value)
        };
        let max_viewport_dimensions = unsafe {
            let mut raw_value: [GLint; 2] = [0; 2];    
            gl::GetIntegerv(GlRuntimeParameter::MaxViewportDimensions as u32, &mut raw_value[0]);

            format!("(width: {}, height: {})", raw_value[0], raw_value[1])
        };
        let stereo = unsafe {
            let mut raw_value = 0;
            gl::GetBooleanv(GlRuntimeParameter::Stereo as u32, &mut raw_value);

            format!("{}", raw_value)
        };

        let mut data = HashMap::new();
        data.insert(GlRuntimeParameter::MaxCombinedTextureImageUnits, max_combined_texture_image_units);
        data.insert(GlRuntimeParameter::MaxCubeMapTextureSize, max_cube_map_texture_size);
        data.insert(GlRuntimeParameter::MaxDrawBuffers, max_draw_buffers);
        data.insert(GlRuntimeParameter::MaxFragmentUniformComponents, max_fragment_uniform_components);
        data.insert(GlRuntimeParameter::MaxTextureImageUnits, max_texture_image_units);
        data.insert(GlRuntimeParameter::MaxTextureSize, max_texture_size);
        data.insert(GlRuntimeParameter::MaxVaryingFloats, max_varying_floats);
        data.insert(GlRuntimeParameter::MaxVertexAttributes, max_vertex_attributes);
        data.insert(GlRuntimeParameter::MaxVertexTextureImageUnits, max_vertex_texture_image_units);
        data.insert(GlRuntimeParameter::MaxVertexUniformComponents, max_vertex_uniform_components);
        data.insert(GlRuntimeParameter::MaxViewportDimensions, max_viewport_dimensions);
        data.insert(GlRuntimeParameter::Stereo, stereo);

        GlRuntimeParameterSet { data, }
    }

    /// Load a shader source file.
    fn parse_shader_source<P: AsRef<Path>, R: Read>(&self,
        reader: &mut R, file_name: P, shader_str: &mut [u8]) -> Result<usize, ShaderCompilationError> {

        shader_str[0] = 0;
        let bytes_read = match reader.read(shader_str) {
            Ok(val) => val,
            Err(_) => {
                let disp = file_name.as_ref().display().to_string();
                return Err(ShaderCompilationError::CouldNotParseShader(disp));
            }
        };

        // Append \0 character to end of the shader string to mark the end of a C string.
        shader_str[bytes_read] = 0;

        Ok(bytes_read)
    }

    /// Query the shader information log generated during shader compilation from
    /// OpenGL.
    pub fn get_shader_fragment_log(&self, shader_index: GLuint) -> ShaderLog {
        let mut actual_length = 0;
        unsafe {
            gl::GetShaderiv(shader_index, gl::INFO_LOG_LENGTH, &mut actual_length);
        }
        let mut raw_log = vec![0_i8; actual_length as usize];
        unsafe {
            gl::GetShaderInfoLog(shader_index, raw_log.len() as i32, &mut actual_length, &mut raw_log[0]);
        }
    
        let mut log = String::new();
        for raw_log_i in raw_log.iter().take(actual_length as usize) {
            log.push(*raw_log_i as u8 as char);
        }

        ShaderLog { index: shader_index, log }
    }

    /// Create a shader from source files.
    pub fn compile_shader_fragment<P: AsRef<Path>, R: Read>(
        &self,
        reader: &mut R, file_name: P, kind: GLenum) -> Result<GLuint, ShaderCompilationError> {

        let disp = file_name.as_ref().display();
        // info!("Creating shader from {}.\n", disp);

        let mut shader_string = vec![0; MAX_SHADER_LENGTH];

        let bytes_read = match self.parse_shader_source(reader, &file_name, &mut shader_string) {
            Ok(val) => val,
            Err(e) => {
                // error!("{}", e);
                return Err(e);
            }
        };

        if bytes_read >= (MAX_SHADER_LENGTH - 1) {
            // info!(
            //    "WARNING: The shader was truncated because the shader code 
            //    was longer than MAX_SHADER_LENGTH {} bytes.", MAX_SHADER_LENGTH
            //);
        }

        let shader = unsafe { gl::CreateShader(kind) };
        let p = shader_string.as_ptr() as *const GLchar;
        unsafe {
            gl::ShaderSource(shader, 1, &p, ptr::null());
            gl::CompileShader(shader);
        }

        // Check for shader compile errors.
        let mut params = -1;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut params);
        }

        if params != gl::TRUE as i32 {
            let log = self.get_shader_fragment_log(shader);
            // error!("ERROR: GL shader index {} did not compile\n{}", shader, log);
            return Err(
                ShaderCompilationError::CouldNotCompileShader(format!("{}", disp))
            );
        }
        // info!("Shader compiled with index {}.\n", shader);
    
        Ok(shader)
    }

    /// Query the shader program information log generated during shader compilation
    /// from OpenGL.
    pub fn get_shader_log(&self, program_index: GLuint) -> ProgramLog {
        let mut actual_length = 0;
        unsafe {
            gl::GetProgramiv(program_index, gl::INFO_LOG_LENGTH, &mut actual_length);
        }
        let mut raw_log = vec![0_i8; actual_length as usize];
        unsafe {
            gl::GetProgramInfoLog(program_index, raw_log.len() as i32, &mut actual_length, &mut raw_log[0]);
        }

        let mut log = String::new();
        for raw_log_i in raw_log.iter().take(actual_length as usize) {
            log.push(*raw_log_i as u8 as char);
        }

        ProgramLog { index: program_index, log }
    }

    /// Validate that the shader program `sp` can execute with the current OpenGL program state.
    /// Use this for information purposes in application development. Return `true` if the program and
    /// OpenGL state contain no errors.
    pub fn validate_shader_program(&self, sp: GLuint) -> bool {
        let mut params = -1;
        unsafe {
            gl::ValidateProgram(sp);
            gl::GetProgramiv(sp, gl::VALIDATE_STATUS, &mut params);
        }

        if params != gl::TRUE as i32 {
            // error!("Program {} GL_VALIDATE_STATUS = GL_FALSE\n", sp);
            // error!("{}", program_info_log(sp));
        
            return false;
        }

        // info!("Program {} GL_VALIDATE_STATUS = {}\n", sp, params);
    
        true
    }

    /// Compile and link a shader program.
    pub fn link_shader_program(
        &self,
        vertex_shader: GLuint, fragment_shader: GLuint) -> Result<GLuint, ShaderCompilationError> {

        let program = unsafe { gl::CreateProgram() };
        // info!("Created program {}. Attaching shaders {} and {}.\n",
        //     program, vertex_shader, fragment_shader
        // );

        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            // Link the shader program. If binding input attributes, do that before linking.
            gl::LinkProgram(program);
        }

        let mut params = -1;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut params);
        }
        if params != gl::TRUE as i32 {
            // error!("ERROR: could not link shader program GL index {}\n", program);
            // error!("{}", program_info_log(program));
            return Err(ShaderCompilationError::CouldNotLinkShader);
        }

        unsafe {
            // Delete shaders here to free memory.
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(program)
    }

    /// Compile and link a shader program directly from any readable sources.
    pub fn compile_shader<R1: Read, P1: AsRef<Path>, R2: Read, P2: AsRef<Path>>(
        &self,
        vert_reader: &mut R1, vert_file_name: P1,
        frag_reader: &mut R2, frag_file_name: P2) -> Result<GLuint, ShaderCompilationError> {

        let vertex_shader = self.compile_shader_fragment(
            vert_reader, 
            vert_file_name, 
            gl::VERTEX_SHADER
        )?;
        let fragment_shader = self.compile_shader_fragment(
            frag_reader, 
            frag_file_name, 
            gl::FRAGMENT_SHADER
        )?;
        let program = self.link_shader_program(vertex_shader, fragment_shader)?;

        Ok(program)
    }
}

