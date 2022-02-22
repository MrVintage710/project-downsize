use glow::{Context, HasContext, NativeProgram, VERTEX_SHADER, FRAGMENT_SHADER, NativeUniformLocation, NativeShader, UniformLocation};
use std::ops::Add;
use std::fs;
use cgmath::{Vector3, Vector2, Vector4, Matrix4};
use std::collections::HashMap;
use crate::render::shader::UniformValue::{VEC4, VEC3, VEC2, INT, FLOAT};
use crate::render::Debugable;
use egui::{Ui, DragValue};

const VERTEX_SHADER_INDEX : usize = 0;
const FRAGMENT_SHADER_INDEX : usize = 1;
const GEOMETRY_SHADER_INDEX : usize = 2;
const TESSELATION_SHADER_INDEX: usize = 3;

/// This enum represents all of the types that we can turn into a uniform value. To make something
/// have the ability to become a uniform, implement `Into<UniformValue>` for that type.
#[derive(Debug, Clone, Copy, PartialEq)]
enum UniformValue {
    FLOAT(f32),
    INT(i32),
    UNSIGNED_INT(u32),
    VEC4(Vector4<f32>),
    VEC3(Vector3<f32>),
    VEC2(Vector2<f32>),
    MAT4(Matrix4<f32>)
}

/// `Into<UniformValue>` implementation for `Vector4<f32>`. This is so that the type can be used in the
/// `create_uniform()` and `set_uniform()` methods.
impl Into<UniformValue> for Vector4<f32> {
    fn into(self) -> UniformValue {
        VEC4(self)
    }
}

/// `Into<UniformValue>` implementation for `Vector3<f32>`. This is so that the type can be used in the
/// `create_uniform()` and `set_uniform()` methods.
impl Into<UniformValue> for Vector3<f32> {
    fn into(self) -> UniformValue {
        VEC3(self)
    }
}

impl Into<UniformValue> for Vector2<f32> {
    fn into(self) -> UniformValue {
        VEC2(self)
    }
}

impl Into<UniformValue> for i32 {
    fn into(self) -> UniformValue {
        INT(self)
    }
}

impl Into<UniformValue> for f32 {
    fn into(self) -> UniformValue {
        FLOAT(self)
    }
}

/// This represents a Uniform state.
struct UniformState {
    current_value : UniformValue,
    hasChanged: bool,
    location : Option<NativeUniformLocation>
}

impl UniformState {
    pub fn new<T>(value : T, location : Option<UniformLocation>) -> Self where T : Into<UniformValue> {
        let value = value.into();
        UniformState {current_value : value, hasChanged : false, location}
    }

    pub fn set<T>(&mut self, value : T) where T : Into<UniformValue> {
        let value  = value.into();
        if std::mem::discriminant(&self.current_value) == std::mem::discriminant(&value) {
            self.current_value = value;
        } else {
            panic!("The Uniform Type {:?} does not match type {:?}", value, self.current_value)
        }
        self.hasChanged = true;
    }

    pub fn get(&mut self) -> &UniformValue {
        self.hasChanged = false;
        &self.current_value
    }

    pub fn needs_update(&self) -> bool {
        self.hasChanged
    }
}

impl Debugable for UniformState {
    fn debug(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
           match self.current_value {
               VEC4(value) => {
                   let mut proxy = value.clone();
                   ui.add(DragValue::new(&mut proxy.x));
                   ui.add(DragValue::new(&mut proxy.y));
                   ui.add(DragValue::new(&mut proxy.z));
                   ui.add(DragValue::new(&mut proxy.w));
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               VEC3(mut value) => {
                   let mut proxy = value.clone();
                   ui.add(DragValue::new(&mut proxy.x));
                   ui.add(DragValue::new(&mut proxy.y));
                   ui.add(DragValue::new(&mut proxy.z));
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::VEC2(value) => {
                   let mut proxy = value.clone();
                   ui.add(DragValue::new(&mut proxy.x));
                   ui.add(DragValue::new(&mut proxy.y));
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::MAT4(_) => {}
               UniformValue::FLOAT(value) => {
                   let mut proxy = value.clone();
                   ui.add(DragValue::new(&mut proxy));
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::INT(value) => {
                   let mut proxy = value.clone();
                   ui.add(DragValue::new(&mut proxy));
                   if proxy != value {

                   }
               }
               UniformValue::UNSIGNED_INT(_) => {}
           }
        });
    }
}

//fn debug_vec3()

/// This is a shader program struct. It stores all the functionality need for loading shaders and
/// setting uniform values. This struct also stores the current state of uniforms.
pub struct ShaderProgram {
    program : NativeProgram,
    uniforms : HashMap<String, UniformState>,
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

    pub fn uniform<T>(&mut self, name : &str, value : T) where T : Into<UniformValue> {
        if self.uniforms.contains_key(name) {
            let uniform = self.uniforms.get_mut(name).unwrap();
            uniform.set(value)
        } else {
            let uniform = UniformState::new(value, None);
            self.uniforms.insert(name.to_string(), uniform);
        }
    }

    pub fn update_uniforms(&mut self, gl : &Context) {
        unsafe {
            let program = self.program.clone();
            for (name, uniform) in self.uniforms.iter_mut() {
                if uniform.location.is_none() {
                    uniform.location = Some(ShaderProgram::create_uniform_location(program, gl, name))
                }
                if uniform.needs_update() {
                    let location = uniform.location.unwrap().clone();
                    match uniform.get() {
                        VEC4(_) => {}
                        VEC3(vec) => gl.uniform_3_f32(Some(&location), vec.x, vec.y, vec.z),
                        UniformValue::VEC2(_) => {}
                        UniformValue::MAT4(_) => {}
                        UniformValue::FLOAT(_) => {}
                        UniformValue::INT(_) => {}
                        UniformValue::UNSIGNED_INT(_) => {}
                    }
                }
            }
        }
    }

    fn create_uniform_location(program : NativeProgram, gl : &Context, name : &str) -> NativeUniformLocation {
        unsafe {
            let uniform = gl.get_uniform_location(program, name);
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
            value.debug(ui)
        }
    }
}

