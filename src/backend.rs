use crate::gl;

use gl::types::{GLuint, GLfloat};
use glfw::{Action, Context, Key};

use std::ffi::{CStr, CString, c_void};
use std::ptr;
use std::mem;
use std::fmt;


// OpenGL extension constants.
pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;

pub fn gl_str(st: &str) -> CString {
    CString::new(st).unwrap()
}

/// A record containing all the relevant compilation log information for a
/// given GLSL shader program compiled at run time.
pub struct ShaderProgramLog {
    index: GLuint,
    log: String,
}

impl fmt::Display for ShaderProgramLog {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(formatter, "Program info log for GL index {}:", self.index).unwrap();
        writeln!(formatter, "{}", self.log)
    }
}

/// Query the shader program information log generated during shader compilation
/// from OpenGL.
pub fn program_info_log(index: GLuint) -> ShaderProgramLog {
    let mut actual_length = 0;
    unsafe {
        gl::GetProgramiv(index, gl::INFO_LOG_LENGTH, &mut actual_length);
    }
    let mut raw_log = vec![0 as i8; actual_length as usize];
    unsafe {
        gl::GetProgramInfoLog(index, raw_log.len() as i32, &mut actual_length, &mut raw_log[0]);
    }

    let mut log = String::new();
    for i in 0..actual_length as usize {
        log.push(raw_log[i] as u8 as char);
    }

    ShaderProgramLog { index: index, log: log }
}

/// Validate that the shader program `sp` can execute with the current OpenGL program state.
/// Use this for information purposes in application development. Return `true` if the program and
/// OpenGL state contain no errors.
pub fn validate_shader_program(sp: GLuint) -> bool {
    let mut params = -1;
    unsafe {
        gl::ValidateProgram(sp);
        gl::GetProgramiv(sp, gl::VALIDATE_STATUS, &mut params);
    }

    if params != gl::TRUE as i32 {
        println!("Program {} GL_VALIDATE_STATUS = GL_FALSE\n", sp);
        println!("{}", program_info_log(sp));
        
        return false;
    }

    println!("Program {} GL_VALIDATE_STATUS = {}\n", sp, params);
    
    true
}
