use crate::render::buffer::{VBO, VAO};
use glow::Context;
use obj::Obj;
use std::path::{Path, PathBuf};
use cgmath::{Vector2, Vector3};
use crate::render::Deletable;
use crate::Renderable;

pub struct OBJModel {
    vao : VAO,
    verts : VBO,
    norms : VBO,
    uv: VBO,
}

impl OBJModel {
    pub fn new(gl : &Context, file_name : &str) -> Obj {
        let path = Path::new("")
            .join("assets")
            .join("models")
            .join(file_name);

        let obj = Obj::load(path.as_path());
        obj.expect(&*format!("Object at path {} did NOT rendered correctly", file_name))
        //load in this order: verticies -> uv -> normal
        // let mut vert_vbo = VBO::new(&gl).unwrap();
        // let converted_vecs: Vec<Vector3<f32>> =
        //     convert_vec_array3_to_vec_vector3(&obj.unwrap().data.position);
        // vert_vbo.load_vec3s(&gl, converted_vecs);
        //
        // let mut uv_vbo = VBO::new(&gl).unwrap();
        // let converted_uvs: Vec<Vector2<f32>> =
        //     convert_vec_array2_to_vec_vector2(&obj.unwrap().data.texture);
        // uv_vbo.load_vec2s(&gl, converted_uvs);
        //
        // let mut norm_vbo = VBO::new(&gl).unwrap();
        // let converted_norms: Vec<Vector3<f32>> =
        //     convert_vec_array3_to_vec_vector3(&obj.unwrap().data.normal);
        // norm_vbo.load_vec3s(&gl, converted_norms);
        //
        //
        // let mut vao = VAO::new(&gl).unwrap();
        // vao.add_vbo(&gl, 0, &vert_vbo);
        // vao.add_vbo(&gl, 1, &uv_vbo);
        // vao.add_vbo(&gl, 2, &norm_vbo);
        //
        // OBJModel {vao, verts: vert_vbo, norms: norm_vbo, uv: uv_vbo }


        // let converted_vecs: Vec<Vector3<f32>> =
        //     obj.unwrap()
        //     .data.position.iter()
        //     .map(|a|
        //         Vector3::new(a[0], a[1], a[2]))
        //     .collect();
        // vec2 converter + vec3 converter

    }
}

// fn convert_vec_array3_to_vec_vector3<T>(vec_of_arrays: &Vec<[T;3]>) -> Vec<Vector3<T>>{
//     let converted_array: Vec<Vector3<T>> = *vec_of_arrays.iter()
//         .map(|a| Vector3::new(, &a[1], &a[2]))
//         .collect::<Vector3<T>>();
//     converted_array
// }
//
// fn convert_vec_array2_to_vec_vector2<T>(vec_of_2Darrays: &Vec<[T;2]>) -> Vec<Vector2<T>> {
//     let converted_array: Vec<Vector2<T>> = *vec_of_2Darrays.iter()
//         .map(|a| Vector2::new(&a[0], &a[1]))
//         .collect::<Vector2<T>>();
//     converted_array
// }

// impl Deletable for ObjModel {
//     unsafe fn delete(&self, gl: &Context) {
//
//     }
// }

// impl Renderable for ObjModel {
//     unsafe fn render(&self, gl: &Context) {

        // let mut vert_vbo = VBO::new(&gl).unwrap();
        // vert_vbo.load_vec3s(&gl, verts);
        //
        // let mut uv_vbo = VBO::new(&gl)?;
        // uv_vbo.load_vec2s(&gl, uvs);
        //
        // let mut norm_vbo = VBO::new(&gl)?;
        // norm_vbo.load_vec3s(&gl, norm);
        //
        // let mut vao = VAO::new(&gl).unwrap();
        // vao.addIndexBuffer(&gl, vec![0, 2, 1, 1, 2, 3]);
        // vao.add_vbo(&gl ,0, &vert_vbo);
        // vao.add_vbo(&gl, 1, &uv_vbo);
    // }
// }
