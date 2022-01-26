use glow::{Context, HasContext, NativeProgram, VERTEX_SHADER, FRAGMENT_SHADER};
use std::ops::Add;
use std::fs;

pub struct ShaderProgram {
    program : NativeProgram,
}

impl ShaderProgram {

    pub fn new(gl : &Context) -> Result<Self, String> {
        unsafe {
            let program = gl.create_program()?;
            Ok(ShaderProgram{ program })
        }
    }

    pub fn load_vertex_shader(&self, gl : &Context, file_name : &str) -> Result<(), String> {
        self.load_shader(gl, file_name, VERTEX_SHADER)
    }

    pub fn load_fragment_shader(&self, gl : &Context, file_name : &str) -> Result<(), String> {
        self.load_shader(gl, file_name, FRAGMENT_SHADER)
    }

    pub fn link(&self, gl : &Context) {
        unsafe {
            self.bind(gl);
            gl.link_program(self.program);
            if !gl.get_program_link_status(self.program) {
                panic!("Unable to link shader program.")
            }
        }
    }

    pub fn bind(&self, gl : &Context) {
        unsafe {
            gl.use_program(Some(self.program))
        }
    }

    fn load_shader(&self, gl : &Context, file_name : &str, shader_type : u32) -> Result<(), String> {
        unsafe {
            self.bind(gl);

            let vs = gl.create_shader(shader_type)?;
            let filepath = String::from("assets/shaders/").add(file_name);
            let data = fs::read_to_string(filepath.as_str()).expect("Could not find file.");
            gl.shader_source(vs, data.as_str());
            gl.compile_shader(vs);

            if !gl.get_shader_compile_status(vs) {
                panic!("{}", gl.get_shader_info_log(vs))
            }

            gl.attach_shader(self.program, vs);
            Ok(())
        }
    }
}

