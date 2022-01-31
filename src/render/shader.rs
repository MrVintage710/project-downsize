use glow::{Context, HasContext, NativeProgram, VERTEX_SHADER, FRAGMENT_SHADER, NativeUniformLocation, NativeShader};
use std::ops::Add;
use std::fs;
use cgmath::Vector3;
use std::collections::HashMap;
use crate::render::Destroyable;

const VERTEX_SHADER_INDEX : usize = 0;
const FRAGMENT_SHADER_INDEX : usize = 1;
const GEOMETRY_SHADER_INDEX : usize = 2;
const TESSELATION_SHADER_INDEX: usize = 3;

pub struct ShaderProgram {
    program : NativeProgram,
    uniforms : HashMap<String, NativeUniformLocation>,
    shaders : [Option<NativeShader>; 4]
}

impl ShaderProgram {
    pub fn new(gl : &Context) -> Result<Self, String> {
        unsafe {
            let program = gl.create_program()?;
            Ok(ShaderProgram{ program, uniforms : HashMap::new(), shaders: [None; 4] })
        }
    }

    pub fn load_vertex_shader(&mut self, gl : &Context, file_name : &str) -> Result<(), String> {
        let shader = self.load_shader(gl, file_name, VERTEX_SHADER)?;
        self.shaders[VERTEX_SHADER_INDEX] = Some(shader);
        Ok(())
    }

    pub fn has_vert_shader(&self) -> bool {
        self.shaders[VERTEX_SHADER_INDEX].is_some()
    }

    pub fn get_vert_shader(&self) -> Option<NativeShader> {
        self.shaders[VERTEX_SHADER_INDEX]
    }

    pub fn load_fragment_shader(&mut self, gl : &Context, file_name : &str) -> Result<(), String> {
        let shader = self.load_shader(gl, file_name, FRAGMENT_SHADER)?;
        self.shaders[FRAGMENT_SHADER_INDEX] = Some(shader);
        Ok(())
    }

    pub fn has_fragment_shader(&self) -> bool {
        self.shaders[FRAGMENT_SHADER_INDEX].is_some()
    }

    pub fn link(&self, gl : &Context) {
        unsafe {
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

    fn load_shader(&self, gl : &Context, file_name : &str, shader_type : u32) -> Result<NativeShader, String> {
        unsafe {
            let shader = gl.create_shader(shader_type)?;
            let filepath = String::from("assets/shaders/").add(file_name);
            let data = fs::read_to_string(filepath.as_str()).expect("Could not find file.");
            gl.shader_source(shader, data.as_str());
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                return Err(gl.get_shader_info_log(shader))
            }

            gl.attach_shader(self.program, shader);
            Ok(shader)
        }
    }
}

impl Destroyable for ShaderProgram {
    unsafe fn destroy(&self, gl: &Context) {
        unsafe {
            for i in 0..4 {
                let shader = self.shaders[i];
                if shader.is_some() {gl.delete_shader(shader.unwrap())}
            }

            gl.delete_program(self.program)
        }
    }
}

