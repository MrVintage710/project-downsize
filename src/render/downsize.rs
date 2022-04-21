use cgmath::Vector2;
use egui::{DragValue, Ui};
use glow::*;
use glutin::dpi::PhysicalSize;
use crate::render::debug::{Debugable, UIRenderType};
use crate::util::variable::UpdateVariable;

const STARTING_DIM : u32 = 500;

pub struct Downsize {
    pixel_density : u32,
    last_width : u32,
    last_height : u32,
    fbo : NativeFramebuffer,
    color_attachment : NativeTexture,
    depth_attachment : NativeTexture,
    should_recalc : bool
}

impl Downsize {

    pub fn new(gl : &Context, pixel_density : u32) -> Self {
        unsafe {
            let fbo = gl.create_framebuffer().expect("Can't create fbo.");
            gl.bind_framebuffer(FRAMEBUFFER, Some(fbo));

            //Texture Attachment
            let color_attachment = gl.create_texture().expect("Could not create texture.");
            gl.bind_texture(TEXTURE_2D, Some(color_attachment));
            gl.tex_image_2d(TEXTURE_2D, 0, SRGB as i32, STARTING_DIM as i32, STARTING_DIM as i32, 0, RGB, UNSIGNED_BYTE, None);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl.bind_texture(TEXTURE_2D, None);
            gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(color_attachment), 0);

            //Depth Stencil Attachment
            let depth_attachment = gl.create_texture().expect("Could not create depth texture");
            gl.bind_texture(TEXTURE_2D, Some(depth_attachment));
            gl.tex_image_2d(TEXTURE_2D, 0, DEPTH24_STENCIL8 as i32, STARTING_DIM as i32, STARTING_DIM as i32, 0, DEPTH_STENCIL, UNSIGNED_INT_24_8, None);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl.bind_texture(TEXTURE_2D, None);
            gl.framebuffer_texture_2d(FRAMEBUFFER, DEPTH_STENCIL_ATTACHMENT, TEXTURE_2D, Some(depth_attachment), 0);

            Downsize {
                pixel_density,
                last_width : 0,
                last_height : 0,
                fbo,
                color_attachment,
                depth_attachment,
                should_recalc : false
            }
        }
    }

    pub fn render<'a>(&mut self, gl : &'a Context, size : PhysicalSize<u32>, renderCallback: impl FnOnce(&'a Context, f32)) {
        unsafe {
            if self.pixel_density < 1 {self.pixel_density = 1}
            if self.pixel_density > size.height { self.pixel_density = size.height}
            let (width, height, aspect_ratio) = self.calc_texture_size(gl, size);

            gl.bind_framebuffer(FRAMEBUFFER, Some(self.fbo));
            gl.viewport(0, 0, width as i32, height as i32);
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            renderCallback(gl, aspect_ratio);

            gl.bind_framebuffer(FRAMEBUFFER, None);
            gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.fbo));
            gl.bind_framebuffer(DRAW_FRAMEBUFFER, None);
            gl.blit_framebuffer(0, 0, width as i32, height as i32, 0, 0, size.width as i32, size.height as i32, COLOR_BUFFER_BIT, NEAREST);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
        }
    }

    fn calc_texture_size(&mut self, gl : &Context, new_size : PhysicalSize<u32>) -> (u32, u32, f32) {
        let aspect_ratio = new_size.width as f32 / new_size.height as f32;
        let new_height = self.pixel_density;
        let new_width = (self.pixel_density as f32 * aspect_ratio).floor() as u32;

        if new_size.width != self.last_width || new_size.height != self.last_height || self.should_recalc {
            unsafe {
                //println!("RECALCULATING TEXTURE! {}", new_height);
                gl.bind_texture(TEXTURE_2D, Some(self.color_attachment));
                gl.tex_image_2d(TEXTURE_2D, 0, SRGB as i32, new_width as i32, new_height as i32, 0, RGB, UNSIGNED_BYTE, None);
                gl.bind_texture(TEXTURE_2D, Some(self.depth_attachment));
                gl.tex_image_2d(TEXTURE_2D, 0, DEPTH24_STENCIL8 as i32, new_width as i32, new_height as i32, 0, DEPTH_STENCIL, UNSIGNED_INT_24_8, None);
            }
            self.last_width = new_size.width;
            self.last_height = new_size.height;
            self.should_recalc = false;
        }

        (new_width, new_height, aspect_ratio)
    }

    pub fn destroy(&self, gl : &Context) {
        unsafe { gl.delete_framebuffer(self.fbo) }
    }
}

impl Debugable for Downsize {
    fn debug(&mut self, ui: &mut Ui, render_type: &UIRenderType) {
        let beginning = self.pixel_density;
        let responce = ui.add(DragValue::new(&mut self.pixel_density));
        if beginning != self.pixel_density {self.should_recalc = true}
    }
}