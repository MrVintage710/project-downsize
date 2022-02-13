use glow::{Context, HasContext, NativeTexture, TEXTURE_2D, RGBA, TEXTURE_MAG_FILTER, NEAREST};
use image::io::Reader as ImageReader;
use egui_glow::glow::UNSIGNED_BYTE;
use std::borrow::Borrow;

pub struct Texture {
    texture : NativeTexture
}

impl Texture {
    pub fn new(gl : &Context, filename : &str) -> Self {
        unsafe {
            let image = ImageReader::open(format!("assets/textures/{}", filename))
                .expect("Unable to find texutre file.");

            let image  = image.decode().unwrap();

            let texture = gl.create_texture().expect("Can not create texture.");
            gl.bind_texture(TEXTURE_2D, Some(texture));

            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);

            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, image.width() as i32, image.height() as i32, 0, RGBA, UNSIGNED_BYTE, Some(image.as_bytes()));
            gl.generate_mipmap(TEXTURE_2D);
            Texture{texture}
        }
    }

    pub fn bind(&self, gl : &Context) {
        unsafe { gl.bind_texture(TEXTURE_2D, Some(self.texture)); }
    }
}