use glow::*;
use image::io::Reader as ImageReader;
use egui_glow::glow::UNSIGNED_BYTE;
use std::borrow::Borrow;
use image::{EncodableLayout, GenericImageView};
use std::fs::File;
use png::ColorType;

pub struct Texture {
    texture : NativeTexture
}

impl Texture {
    pub fn new(gl : &Context, filename : &str) -> Self {
        unsafe {
            let image = ImageReader::open(format!("assets/textures/{}", filename))
                .expect("Unable to find texture file.");

            let image  = image.decode().unwrap();
            let image = image.into_rgba32f();
            println!("{:?}", image.get_pixel(0, 1));

            // let decoder = png::Decoder::new(File::open(format!("assets/textures/{}", filename)).unwrap());
            // let mut reader = decoder.read_info().unwrap();
            //
            // let mut buf = vec![0; reader.output_buffer_size()];
            // let info = reader.next_frame(&mut buf).unwrap();
            //
            // println!("{}, {}, {}", buf[(info.width * 4) as usize], buf[(info.width * 4 + 1) as usize], buf[(info.width * 4 + 2) as usize]);

            let texture = gl.create_texture().expect("Can not create texture.");
            gl.bind_texture(TEXTURE_2D, Some(texture));

            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);

            // let color_format = match info.color_type {
            //     ColorType::Grayscale => {LUMINANCE}
            //     ColorType::Rgb => {println!("RGB"); RGB}
            //     ColorType::Indexed => {ALPHA}
            //     ColorType::GrayscaleAlpha => {LUMINANCE_ALPHA}
            //     ColorType::Rgba => {println!("RGBA"); RGBA}
            // };

            gl.tex_image_2d(TEXTURE_2D, 0, SRGB as i32, image.width() as i32, image.height() as i32, 0, RGBA, FLOAT, Some(&image.as_bytes()));
            gl.generate_mipmap(TEXTURE_2D);

            Texture{texture}
        }
    }

    pub fn bind(&self, gl : &Context) {
        unsafe { gl.bind_texture(TEXTURE_2D, Some(self.texture)); }
    }
}