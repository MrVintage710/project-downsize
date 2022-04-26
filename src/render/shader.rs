use std::borrow::Borrow;
use glow::{Context, HasContext, NativeProgram, VERTEX_SHADER, FRAGMENT_SHADER, NativeUniformLocation, NativeShader, UniformLocation, GEOMETRY_SHADER, TESS_CONTROL_SHADER};
use std::ops::Add;
use std::fs;
use cgmath::{Vector3, Vector2, Vector4, Matrix4};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc;
use crate::render::shader::UniformValue::{VEC4, VEC3, VEC2, INT, FLOAT, MAT4, U_INT};
use egui::{Ui, DragValue};
use crate::render::debug::{Debugable, UIRenderType};
use crate::render::debug::UIRenderType::*;
use crate::render::shader::ShaderError::{GLSL_COMPILE_ERROR, GLSL_LINK_ERROR, GLSL_PARSE_ERROR, MISSING_SHADER, UNIFORM_ALREADY_EXISTS, UNIFORM_LOCATION_NOT_FOUND};
use crate::render::transform::Transform;
use glsl::parser::{Parse as _, ParseError};
use glsl::syntax::{Declaration, ExternalDeclaration, ShaderStage, StorageQualifier, TypeQualifierSpec};
use glsl::syntax::Declaration::InitDeclaratorList;
use crate::render::RenderContext;

const VERTEX_SHADER_INDEX : usize = 0;
const FRAGMENT_SHADER_INDEX : usize = 1;
const GEOMETRY_SHADER_INDEX : usize = 2;
const TESSELATION_SHADER_INDEX: usize = 3;

/// This enum represents all of the types that we can turn into a uniform value. To make something
/// have the ability to become a uniform, implement `Into<UniformValue>` for that type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UniformValue {
    FLOAT(f32),
    INT(i32),
    U_INT(u32),
    VEC4(Vector4<f32>),
    VEC3(Vector3<f32>),
    VEC2(Vector2<f32>),
    MAT4(Matrix4<f32>),
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

impl Into<UniformValue> for Matrix4<f32> {
    fn into(self) -> UniformValue {
        MAT4(self)
    }
}

impl Into<UniformValue> for Transform {
    fn into(self) -> UniformValue {
        MAT4(self.calc_mat())
    }
}

/// This represents a Uniform state.
struct UniformState {
    current_value : UniformValue,
    hasChanged: bool,
    location : Option<NativeUniformLocation>,
    render_type: UIRenderType
}

impl UniformState {
    pub fn new<T>(value : T, location : Option<UniformLocation>) -> Self where T : Into<UniformValue> {
        let value = value.into();
        UniformState {current_value : value, hasChanged : true, location, render_type : UIRenderType::HIDDEN}
    }

    pub fn as_debug_mutable(mut self) -> Self {
        self.render_type = MUTABLE;
        self
    }

    pub fn as_debug_immutable(mut self) -> Self {
        self.render_type = IMMUTABLE;
        self
    }

    pub fn set<T>(&mut self, value : T) where T : Into<UniformValue> {
        let value  = value.into();
        if std::mem::discriminant(&self.current_value) == std::mem::discriminant(&value) {
            if self.current_value != value {
                self.current_value = value;
                self.hasChanged = true;
            }
        } else {
            panic!("The Uniform Type {:?} does not match type {:?}", value, self.current_value)
        }

    }

    pub fn get(&mut self) -> &mut UniformValue {
        self.hasChanged = false;
        &mut self.current_value
    }

    pub fn needs_update(&self) -> bool {
        self.hasChanged
    }
}

impl Debugable for UniformState {
    fn debug(&mut self, ui: &mut Ui, render_type : &UIRenderType) {
        ui.horizontal(|ui| {
           match self.current_value {
               VEC4(value) => {
                   let mut proxy = value.clone();
                   proxy.debug(ui, render_type);
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               VEC3(mut value) => {
                   let mut proxy = value.clone();
                   proxy.debug(ui, render_type);
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::VEC2(value) => {
                   let mut proxy = value.clone();
                   proxy.debug(ui, render_type);
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::MAT4(value) => {
                   let mut proxy = value.clone();
                   proxy.debug(ui, render_type);
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::FLOAT(value) => {
                   let mut proxy = value.clone();
                   proxy.debug(ui, render_type);
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::INT(value) => {
                   let mut proxy = value.clone();
                   proxy.debug(ui, render_type);
                   if proxy != value {
                       self.set(proxy)
                   }
               }
               UniformValue::U_INT(_) => {}
               // UniformValue::TRANSFORM(value) => {
               //     let mut proxy = value.clone();
               //     proxy.debug(ui, render_type);
               //     if proxy != value {
               //         self.set(proxy)
               //     }
               // }
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

    pub fn uniform_debug_type(&mut self, name : &str, debug_type : UIRenderType) {
        if self.uniforms.contains_key(name) {
            self.uniforms.get_mut(name).unwrap().render_type = debug_type;
        }
    }

    pub fn update_uniforms(&mut self, gl : &Context) {
        unsafe {
            self.bind(gl);
            for (name, uniform) in self.uniforms.iter_mut() {
                if uniform.location.is_none() {
                    uniform.location = Some(ShaderProgram::create_uniform_location(self.program.clone(), gl, name))
                }
                if uniform.needs_update() {
                    //println!("Updating uniform: {}", name);
                    ShaderProgram::load_uniform(gl, &uniform.location.unwrap(), uniform.get());
                }
            }
        }
    }

    fn load_uniform(gl : &Context, location : &NativeUniformLocation, value : &mut UniformValue) {
        unsafe {
            match value {
                VEC4(vec) => gl.uniform_4_f32(Some(&location), vec.x, vec.y, vec.z, vec.w),
                VEC3(vec) => gl.uniform_3_f32(Some(&location), vec.x, vec.y, vec.z),
                UniformValue::VEC2(vec) => gl.uniform_2_f32(Some(&location), vec.x, vec.y),
                UniformValue::MAT4(mat) => {
                    let slice : [[f32; 4]; 4] = mat.clone().into();
                    let result = &slice.concat();
                    gl.uniform_matrix_4_f32_slice(Some(&location), false, result)
                },
                UniformValue::FLOAT(_) => {}
                UniformValue::INT(_) => {}
                UniformValue::U_INT(_) => {}
                // UniformValue::TRANSFORM(transform) => {
                //     let mat = transform.calc_mat();
                //     let slice : [[f32; 4]; 4] = mat.clone().into();
                //     let result = &slice.concat();
                //     gl.uniform_matrix_4_f32_slice(Some(&location), false, result)
                // }
            }
        }
    }

    pub fn mutate_uniform(&mut self, uniform_name : &str, callback : impl FnOnce(&mut UniformValue)) {
        if self.uniforms.contains_key(uniform_name) {
            let arg = self.uniforms.get_mut(uniform_name).unwrap();
            callback(&mut arg.current_value);
            arg.hasChanged = true;
        } else {
            panic!("There is no uniform with the name '{}' to mutate.", uniform_name);
        }
    }

    pub fn load_uniform_immediate<T>(&self, gl : &Context, location : &NativeUniformLocation, value : T) where T : Into<UniformValue> {

    }

    pub fn create_uniform_location(program : NativeProgram, gl : &Context, name : &str) -> NativeUniformLocation {
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
    fn debug(&mut self, ui: &mut Ui, render_type : &UIRenderType) {
        if let UIRenderType::HIDDEN = render_type {return;}

        for (name, value) in self.uniforms.iter_mut() {
            if let UIRenderType::HIDDEN = value.render_type {}
            else {
                ui.collapsing(name, |ui|{
                    value.debug(ui, &value.render_type.clone())
                });
            }
        }
    }
}


///Rewrite of the shader
pub struct ShaderBuilder {
    shaders : [Option<String>; 4]
}

impl ShaderBuilder {

    pub fn new() -> Self {
        ShaderBuilder {
            shaders: [None, None, None, None]
        }
    }

    pub fn with_vert_shader(mut self, filename : &str) -> Self {
        self.shaders[VERTEX_SHADER_INDEX] = Some(filename.to_owned());
        self
    }

    pub fn with_frag_shader(mut self, filename : &str) -> Self {
        self.shaders[FRAGMENT_SHADER_INDEX] = Some(filename.to_owned());
        self
    }

    pub fn with_geo_shader(mut self, filename : &str) -> Self {
        self.shaders[GEOMETRY_SHADER_INDEX] = Some(filename.to_owned());
        self
    }

    pub fn with_tes_shader(mut self, filename : &str) -> Self {
        self.shaders[TESSELATION_SHADER_INDEX] = Some(filename.to_owned());
        self
    }

    pub fn build(self, render_context : &Rc<RenderContext>) -> ShaderResult<Shader> {
        if self.shaders[VERTEX_SHADER_INDEX].is_none() || self.shaders[FRAGMENT_SHADER_INDEX].is_none() {
            return Err(MISSING_SHADER)
        }

        let gl = &render_context.gl;

        unsafe {
            let program = gl.create_program().expect("Unable to create shader program.");

            let vert_shader = self.load_shader(gl, self.shaders[VERTEX_SHADER_INDEX].clone().unwrap().as_str(), VERTEX_SHADER)?;
            gl.attach_shader(program, vert_shader);

            let frag_shader = self.load_shader(gl, self.shaders[FRAGMENT_SHADER_INDEX].clone().unwrap().as_str(), FRAGMENT_SHADER)?;
            gl.attach_shader(program, frag_shader);

            let mut geo_shader = None;
            if self.shaders[GEOMETRY_SHADER_INDEX].is_some() {
                geo_shader = Some(self.load_shader(gl, self.shaders[GEOMETRY_SHADER_INDEX].clone().unwrap().as_str(), GEOMETRY_SHADER)?);
                gl.attach_shader(program, geo_shader.unwrap());
            }

            let mut tes_shader = None;
            if self.shaders[TESSELATION_SHADER_INDEX].is_some() {
                tes_shader = Some(self.load_shader(gl, self.shaders[TESSELATION_SHADER_INDEX].clone().unwrap().as_str(), TESS_CONTROL_SHADER)?);
                gl.attach_shader(program, tes_shader.unwrap())
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(GLSL_LINK_ERROR)
            }

            Ok(Shader {
                program,
                vert_shader,
                frag_shader,
                geo_shader,
                tes_shader,
                render_context: Rc::clone(render_context)
            })
        }
    }

    fn load_shader(&self, gl : &Context, file_name : &str, shader_type : u32) -> ShaderResult<NativeShader> {
        unsafe {
            let shader = gl.create_shader(shader_type).expect("Unable to create shader.");
            let filepath = String::from("assets/shaders/").add(file_name);
            let data = fs::read_to_string(filepath.as_str()).expect("Could not find file.");

            //Test parsing stuff

            // let apt = ShaderStage::parse(data.clone());
            // if let Err(parse_error) = apt {return Err(GLSL_PARSE_ERROR(parse_error))}
            // let mut apt = apt.unwrap();
            // for i in apt.0 {
            //     if let ExternalDeclaration::Declaration(mut dec) = i {
            //         if let Declaration::InitDeclaratorList(mut list) = dec {
            //             let type_qualifier = list.head.ty.qualifier.expect("No qualifiers").qualifiers.0;
            //             let type_qualifier_spec = type_qualifier.get(0).unwrap();
            //
            //             if let TypeQualifierSpec::Storage(qualifer) = type_qualifier_spec {
            //                 if let StorageQualifier::Uniform = qualifer {
            //                     println!("{:?} {:?} {:?}", qualifer, list.head.ty.ty.ty, list.head.name.expect("No name").0);
            //                 }
            //             }
            //         }
            //     }
            // }

            gl.shader_source(shader, data.as_str());
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                return Err(GLSL_COMPILE_ERROR(gl.get_shader_info_log(shader)))
            }
            Ok(shader)
        }
    }
}

type ShaderResult<T> = Result<T, ShaderError>;

#[derive(Debug)]
pub enum ShaderError {
    MISSING_SHADER,
    GLSL_LINK_ERROR,
    GLSL_PARSE_ERROR(ParseError),
    GLSL_COMPILE_ERROR(String),
    UNIFORM_ALREADY_EXISTS,
    UNIFORM_LOCATION_NOT_FOUND
}

pub struct Shader{
    program : NativeProgram,
    vert_shader : NativeShader,
    frag_shader : NativeShader,
    geo_shader : Option<NativeShader>,
    tes_shader : Option<NativeShader>,
    render_context : Rc<RenderContext>
    //uniform_map : HashMap<String, Rc<ShaderUniformHandler<'a>>>
}

impl Shader {
    pub fn add_uniform<T>(&self, uniform : &mut T) -> ShaderResult<()> where T : Uniform {
        //if self.uniform_map.contains_key(uniform_name) {return Err(UNIFORM_ALREADY_EXISTS)}

        unsafe {
            let u = self.render_context.gl.get_uniform_location(self.program, uniform_name);
            if u.is_none() { return Err(UNIFORM_LOCATION_NOT_FOUND)}

            uniform.provide_handle(ShaderUniformHandler{
                program: self.program.clone(),
                uniform: u.unwrap(),
                render_context: Rc::clone(&self.render_context)
            })
        }

        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            self.render_context.gl.use_program(Some(self.program))
        }
    }
}

#[derive(Clone)]
pub struct ShaderUniformHandler {
    program : NativeProgram,
    uniform : NativeUniformLocation,
    render_context: Rc<RenderContext>
}

impl ShaderUniformHandler {
    pub fn update_uniform(&self, value : impl Into<UniformValue>) {
        let value = value.into();
        unsafe {
            match value {
                FLOAT(value) => {
                    self.render_context.gl.uniform_1_f32(Some(&self.uniform), value);
                }
                INT(_) => {}
                U_INT(_) => {}
                VEC4(_) => {}
                VEC3(_) => {}
                VEC2(_) => {}
                MAT4(value) => {
                    let slice : [[f32; 4]; 4] = value.into();
                    let result = &slice.concat();
                    self.render_context.gl.uniform_matrix_4_f32_slice(Some(&self.uniform), false, result)
                }
            }
        }
    }
}

pub trait Uniform {
    fn provide_handle(&mut self, handle : ShaderUniformHandler);
}