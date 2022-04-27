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

            let mut uniform_map = HashMap::new();
            for i in 0..gl.get_active_uniforms(program) {
                let uniform = gl.get_active_uniform(program, i);
                if uniform.is_some() {
                    let uniform = uniform.unwrap();
                    uniform_map.insert(uniform.name.clone(), gl.get_uniform_location(program, uniform.name.as_str()).unwrap());
                }
                // println!("Active Shader {}: {:?} {:?}", i, uni.name, uni.utype)
            }

            Ok(Shader {
                program,
                vert_shader,
                frag_shader,
                geo_shader,
                tes_shader,
                render_context: Rc::clone(render_context),
                uniform_map
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
    render_context : Rc<RenderContext>,
    uniform_map : HashMap<String, NativeUniformLocation>
}

impl Shader {
    pub fn add_uniform<T>(&mut self, uniform_name : &str, uniform : &mut T) -> ShaderResult<()> where T : Uniform {
        unsafe {
            let uniform_location = self.get_uniform_location(uniform_name)?;

            uniform.provide_handle(ShaderUniformHandler{
                program: self.program.clone(),
                uniform : uniform_location,
                render_context: Rc::clone(&self.render_context)
            })
        }

        Ok(())
    }

    pub fn send_uniform(&mut self, uniform_name : &str, value : impl Into<UniformValue>) -> ShaderResult<()>{
        let uniform_location = self.get_uniform_location(uniform_name)?;
        send_uniforms(&self.render_context.gl, value, self.program, uniform_location);
        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            self.render_context.gl.use_program(Some(self.program))
        }
    }

    pub fn delete(&self) {
        unsafe {
            let gl = &self.render_context.gl;
            gl.delete_shader(self.vert_shader);
            gl.delete_shader(self.frag_shader);
            if self.geo_shader.is_some() { gl.delete_shader(self.geo_shader.unwrap())}
            if self.tes_shader.is_some() { gl.delete_shader(self.tes_shader.unwrap())}
            gl.delete_program(self.program);
        }
    }

    fn get_uniform_location(&mut self, uniform_name : &str) -> ShaderResult<NativeUniformLocation>{
        unsafe {
            if !self.uniform_map.contains_key(uniform_name) {
                return Err(UNIFORM_LOCATION_NOT_FOUND)
            }

            let uniform_location = self.uniform_map.get(uniform_name).unwrap();

            Ok(*uniform_location)
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
        send_uniforms(&self.render_context.gl, value, self.program, self.uniform)
    }
}

pub trait Uniform {
    fn provide_handle(&mut self, handle : ShaderUniformHandler);
}

fn send_uniforms(gl : &Context, uniform_value : impl Into<UniformValue>, program : NativeProgram, location : UniformLocation) {
    let uniform_value = uniform_value.into();
    unsafe {
        gl.use_program(Some(program));
        match uniform_value {
            FLOAT(value) => {
                gl.uniform_1_f32(Some(&location), value);
            }
            INT(_) => {}
            U_INT(_) => {}
            VEC4(_) => {}
            VEC3(_) => {}
            VEC2(_) => {}
            MAT4(value) => {
                let slice : [[f32; 4]; 4] = value.into();
                let result = &slice.concat();
                gl.uniform_matrix_4_f32_slice(Some(&location), false, result)
            }
        }
    }
}