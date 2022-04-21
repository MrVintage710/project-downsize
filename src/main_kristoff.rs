use glow::Context;
use crate::render::model::{OBJModel};

pub fn kristoff_main(gl: &Context) {
    println!("Hello Rust Graphics :)");
    let file_name = "cube.obj";
    let model = OBJModel::new(&gl, file_name)
        .expect("Expected object, no Object named {file_name} found");

    println!("{:?}", model.vertices);
    println!("{:?}", model.indices);
}