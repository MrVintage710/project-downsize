use glow::{Context, HasContext, NativeProgram, VERTEX_SHADER, FRAGMENT_SHADER, NativeUniformLocation, NativeShader, UniformLocation};
use std::ops::Add;
use std::fs;
use cgmath::{Vector3, Vector2, Vector4, Matrix4};
use std::collections::HashMap;
use crate::render::shader::UniformType::{VEC4, VEC3};
use crate::render::Debugable;
use egui::{Ui, DragValue};

const VERTEX_SHADER_INDEX : usize = 0;
const FRAGMENT_SHADER_INDEX : usize = 1;
const GEOMETRY_SHADER_INDEX : usize = 2;
const TESSELATION_SHADER_INDEX: usize = 3;

/// This enum represents all of the types that we can turn into a uniform value. To make something
/// have the ability to become a uniform, implement `Into<UniformValue>` for that type.
#[derive(Debug, Clone, Copy)]
enum UniformType {
    VEC4(Vector4<f32>),
    VEC3(Vector3<f32>),
    VEC2(Vector2<f32>),
    MAT4(Matrix4<f32>)
}

/// `Into<UniformType>` implementation for `Vector4<f32>`. This is so that the type can be used in the
/// `create_uniform()` and `set_uniform()` methods.
impl Into<UniformType> for Vector4<f32> {
    fn into(self) -> UniformType {
        VEC4(self)
    }
}

/// `Into<UniformType>` implementation for `Vector3<f32>`. This is so that the type can be used in the
/// `create_uniform()` and `set_uniform()` methods.
impl Into<UniformType> for Vector3<f32> {
    fn into(self) -> UniformType {
        VEC3(self)
    }
}

/// This represents a Uniform Variable. Consists of a Uniform Type and a Uniform location.
type Uniform = (UniformType, UniformLocation);

/// This is a shader program struct. It stores all the functionality need for loading shaders and
/// setting uniform values. This struct also stores the current state of uniforms.
pub struct ShaderProgram {
    program : NativeProgram,
    uniforms : HashMap<String, Uniform>,
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

    pub fn destroy(&self, gl : &Context) {
        unsafe {
            for shader in self.shaders {
                if shader.is_some() { gl.delete_shader(shader.unwrap())}
            }

            gl.delete_program(self.program)
        }
    }

    pub fn uniform<T>(&mut self, gl : &Context, name : &str, value : T) where T : Into<UniformType>{
        unsafe {
            self.bind(gl);
            let (uniform_type, uniform_location) = if !self.uniforms.contains_key(name) {
                (value.into(), self.create_uniform(gl, name))
            } else {
                let uniform = self.uniforms.get(name).unwrap();
                let t = value.into();
                if std::mem::discriminant(&uniform.0) == std::mem::discriminant(&t) {
                    (t, uniform.1)
                } else {
                    panic!("The Uniform Type {:?} does not match type {:?}", t, uniform.0)
                }
            };

            match uniform_type {
                VEC4(vec) => {todo!()}
                VEC3(vec) => gl.uniform_3_f32(Some(&uniform_location), vec.x, vec.y, vec.z),
                UniformType::VEC2(vec) => {todo!()}
                UniformType::MAT4(vec) => {todo!()}
            }
            self.uniforms.insert(name.to_string(), (uniform_type, uniform_location));
        }
    }

    fn create_uniform(&mut self, gl : &Context, name : &str) -> NativeUniformLocation {
        unsafe {
            let uniform = gl.get_uniform_location(self.program, name);
            if uniform.is_none() { panic!("Unable to create shader location '{}'", name)}
            uniform.unwrap()
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

impl Debugable for ShaderProgram {
    fn debug(&mut self, ui: &mut Ui) {
        for (name, value) in self.uniforms.iter_mut() {
            ui.label(name);
            match value.0 {
                VEC4(vec) => {todo!()}
                VEC3(mut vec) => {
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut vec.x));
                    });
                }
                UniformType::VEC2(vec) => {todo!()}
                UniformType::MAT4(mat) => {todo!()}
            }
        }
    }
}

