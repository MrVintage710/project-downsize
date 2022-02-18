//Here I will add all the code that will help me in loading object, textures, and shaders into buffers
//like VBO's, VAO's, FBO's and so on.

use glow::*;
use cgmath::{Vector3, Vector2, Vector4};
use std::any::TypeId;
use crate::render::Renderable;
use crate::util::bitflag::BitFlag16;

///This is a functional Wrapping of a vbo. This should have all the functions required to create and manage memory in a vbo
pub struct VBO {
    buffer : NativeBuffer,
    gl_type : Option<u32>,
    amount : u32,
    grouping: u32
}

impl VBO {
    pub fn new(gl : &Context) -> Result<Self, String> {
        unsafe {
            let buffer = gl.create_buffer()?;
            Ok(VBO { buffer, gl_type : None, amount : 0 , grouping: 1})
        }
    }

    pub fn load_vec4<T: 'static>(&mut self, gl : &Context, vec : Vector4<T>) {
        unsafe {
            let vec: [T; 4] = vec.into();
            let data: &[u8] = core::slice::from_raw_parts(
                vec.as_ptr() as *const u8,
                vec.len() * core::mem::size_of::<T>(),
            );

            self.bind(gl);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);
            self.set_type::<T>();
            self.amount = 1;
        }
    }

    pub fn load_vec4s<T: 'static>(&mut self, gl : &Context, vecs : Vec<Vector4<T>>) {
        unsafe {
            let mut data: Vec<T> = Vec::new();
            for vec in vecs {
                data.push(vec.x);
                data.push(vec.y);
                data.push(vec.z);
                data.push(vec.w);
            }
            self.amount = data.len() as u32;

            let data: &[u8] = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<T>()
            );

            self.bind(gl);
            gl.buffer_data_u8_slice(ARRAY_BUFFER, data, STATIC_DRAW);
            self.set_type::<T>();
            self.grouping = 4;
        }
    }

    pub fn load_vec3<T: 'static>(&mut self, gl : &Context, vec : Vector3<T>) {
        unsafe {
            let vec: [T; 3] = vec.into();
            let data: &[u8] = core::slice::from_raw_parts(
                vec.as_ptr() as *const u8,
                vec.len() * core::mem::size_of::<T>(),
            );

            self.bind(gl);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);
            self.set_type::<T>();
            self.amount = 3;
        }
    }

    pub fn load_vec3s<T: 'static>(&mut self, gl : &Context, vecs : Vec<Vector3<T>>) {
        unsafe {
            let mut data: Vec<T> = Vec::new();
            for vec in vecs {
                data.push(vec.x);
                data.push(vec.y);
                data.push(vec.z);
            }
            self.amount = data.len() as u32;

            let data: &[u8] = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<T>()
            );

            self.bind(gl);
            gl.buffer_data_u8_slice(ARRAY_BUFFER, data, STATIC_DRAW);
            self.set_type::<T>();
            self.grouping = 3;
        }
    }

    pub fn load_vec2<T: 'static>(&mut self, gl : &Context, vec : Vector2<T>) {
        unsafe {
            let vec: [T; 2] = vec.into();
            let data: &[u8] = core::slice::from_raw_parts(
                vec.as_ptr() as *const u8,
                vec.len() * core::mem::size_of::<T>(),
            );

            self.bind(gl);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);
            self.set_type::<T>();
            self.grouping = 2;
        }
    }

    pub fn load_vec2s<T: 'static>(&mut self, gl : &Context, vecs : Vec<Vector2<T>>) {
        unsafe {
            let mut data: Vec<T> = Vec::new();
            for vec in vecs {
                data.push(vec.x);
                data.push(vec.y);
            }
            self.amount = data.len() as u32;

            let data: &[u8] = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<T>()
            );

            self.bind(gl);
            gl.buffer_data_u8_slice(ARRAY_BUFFER, data, STATIC_DRAW);
            self.set_type::<T>();
            self.grouping = 2;
        }
    }

    pub fn bind(&self, gl : &Context) {
        unsafe {
            gl.bind_buffer(ARRAY_BUFFER, Some(self.buffer));
        }
    }

    pub fn destroy(&self, gl : &Context) {
        unsafe { gl.delete_buffer(self.buffer) }
    }

    fn set_type<T: 'static>(&mut self) {
        let t = TypeId::of::<T>();
        if t == TypeId::of::<i32>() {
            self.gl_type = Some(INT);
        } else if t == TypeId::of::<u32>() {
            self.gl_type = Some(UNSIGNED_INT)
        } else if t == TypeId::of::<i8>() {
            self.gl_type = Some(BYTE)
        } else if t == TypeId::of::<u8>() {
            self.gl_type = Some(UNSIGNED_BYTE)
        } else if t == TypeId::of::<i16>() {
            self.gl_type = Some(SHORT)
        } else if t == TypeId::of::<u16>() {
            self.gl_type = Some(UNSIGNED_SHORT)
        } else if t == TypeId::of::<f32>() {
            self.gl_type = Some(FLOAT)
        } else {
            panic!("The type given is not an accepted type.")
        }
    }

    pub fn grouping(&self) -> u32 {
        self.grouping
    }

    pub fn data_type(&self) -> u32 {
        if self.gl_type.is_some() {
            return self.gl_type.unwrap();
        }

        return NONE;
    }
}

///VAO implementation. Should handle all of the memory management for me.
pub struct VAO {
    array: NativeVertexArray,
    enabled_attribs : BitFlag16,
    element_array : bool,
    render_count : u32
}

impl VAO {
    pub fn new(gl : &Context) -> Result<Self, String> {
        unsafe {
            let vao = gl.create_vertex_array()?;
            println!("MAX ATTRIBS: {}", gl.get_parameter_i32(MAX_VERTEX_ATTRIBS));
            Ok(VAO { array : vao, enabled_attribs : BitFlag16::new() , element_array : false , render_count : 0 })
        }
    }

    pub fn bind(&self, gl : &Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.array));
        }
    }

    pub fn destroy(&self, gl : &Context) {
        unsafe { gl.delete_vertex_array(self.array) }
    }

    pub fn add_vbo(&mut self, gl : &Context, index : u16, vbo : &VBO) {
        if BitFlag16::max() <= index {panic!("The Max number of VAO attribs is {}, {} was given.", BitFlag16::max(), index)}
        if vbo.gl_type.is_none() {panic!("The VBO has no apparent type.")}

        self.enabled_attribs.mark(index);

        unsafe {
            self.bind(gl);
            vbo.bind(gl);
            gl.vertex_attrib_pointer_f32(index as u32, vbo.grouping() as i32, vbo.data_type(), false, 0, 0)
        }
    }

    pub fn addIndexBuffer(&mut self, gl : &Context, indices : Vec<i32>) {
        self.bind(gl);
        unsafe {
            let index_buffer = gl.create_buffer().expect("Cannot create Index Buffer.");
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(index_buffer));
            let data: &[u8] = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * core::mem::size_of::<i32>()
            );
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, data, STATIC_DRAW);
            self.element_array = true;
            self.render_count = indices.len() as u32;
        }
    }
}

impl Renderable for VAO {
    unsafe fn render(&self, gl: &Context) {
        self.bind(gl);
        for i in 0..BitFlag16::max() {
            if self.enabled_attribs.is_marked(i) {
                gl.enable_vertex_attrib_array(i as u32)
            }
        }

        if self.element_array {
            gl.draw_elements(TRIANGLES, self.render_count as i32, UNSIGNED_INT, 0)
        } else {
            gl.draw_arrays(TRIANGLES, 0, self.render_count as i32)
        }
    }
}

//Tests for vbo's
// #[cfg(test)]
// mod render_tests{
//     use super::*;
//
//     #[test]
//     fn type_test() {
//         let vbo = VBO::new()
//     }
// }