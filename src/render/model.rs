use crate::render::buffer::{VBO, VAO};
use glow::Context;

use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj, ObjResult, TexturedVertex};

use std::path::{Path, PathBuf};
use crate::render::texture::Texture;
use crate::render::shader::ShaderProgram;

pub struct OBJModel {
    texture : Option<Texture>,
    program : ShaderProgram,
    vao : VAO,
    verts : VBO,
    norms : VBO,
}

impl OBJModel {
    pub fn new(gl : &Context, file_name : &str) -> ObjResult<Obj<TexturedVertex, u32>> {
        let path = Path::new("")
            .join("assets")
            .join("models")
            .join(file_name);

        let input = BufReader::new(File::open(path)?);
        let model: Obj<TexturedVertex, u32> = load_obj(input)?;
        Ok(model)
    }
}