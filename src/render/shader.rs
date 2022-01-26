use glow::{Context, HasContext, NativeProgram, VERTEX_SHADER, FRAGMENT_SHADER, NativeUniformLocation};
use std::ops::Add;
use std::fs;
use cgmath::Vector3;
use std::collections::HashMap;

pub struct ShaderProgram {
    program : NativeProgram,
    uniforms : HashMap<String, NativeUniformLocation>
}

impl ShaderProgram {

    pub fn new(gl : &Context) -> Result<Self, String> {
        unsafe {
            let program = gl.create_program()?;
            Ok(ShaderProgram{ program, uniforms : HashMap::new() })
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

    pub fn create_uniform_vec3(&mut self, gl : &Context, name : &str, vec : Vector3<f32>) {
        unsafe {
            self.bind(gl);
            let uniform = gl.get_uniform_location(self.program, name);
            if uniform.is_none() { panic!("Unable to create shader location '{}'", name)}
            let uniform = uniform.unwrap();
            gl.uniform_3_f32(Some(&uniform), vec.x, vec.y, vec.z);

            self.uniforms.insert(name.to_string(), uniform);
        }
    }

    pub fn uniform_vec3(&self, gl : &Context, name : &str, vec : Vector3<f32>) {
        unsafe {
            self.bind(gl);
            if self.uniforms.contains_key(name) {
                gl.uniform_3_f32(self.uniforms.get(name), vec.x, vec.y, vec.z)
            } else {
                panic!("There is no shader uniform with name '{}'", name)
            }
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

