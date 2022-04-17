use glow::*;
use glutin::dpi::PhysicalSize;

struct Downsize {
    width : u32,
    height : u32,
    fbo : NativeFramebuffer,
    color_attachment : NativeTexture,
    depth_attachment : NativeTexture
}

impl Downsize {

    pub fn new(gl : &Context, width : u32, height : u32) -> Self {
        unsafe {
            let fbo = gl.create_framebuffer().expect("Can't create fbo.");
            gl.bind_framebuffer(FRAMEBUFFER, Some(fbo));

            //Texture Attachment
            let color_attachment = gl.create_texture().expect("Could not create texture.");
            gl.bind_texture(TEXTURE_2D, Some(color_attachment));
            gl.tex_image_2d(TEXTURE_2D, 0, SRGB as i32, width as i32, height as i32, 0, RGB, UNSIGNED_BYTE, None);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl.bind_texture(TEXTURE_2D, None);
            gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(color_attachment), 0);

            //Depth Stencil Attachment
            let depth_attachment = gl.create_texture().expect("Could not create depth texture");
            gl.bind_texture(TEXTURE_2D, Some(depth_attachment));
            gl.tex_image_2d(TEXTURE_2D, 0, DEPTH24_STENCIL8 as i32, width as i32, height as i32, 0, DEPTH_STENCIL, DEPTH24_STENCIL8, None);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl.bind_texture(TEXTURE_2D, None);
            gl.framebuffer_texture_2d(FRAMEBUFFER, DEPTH_STENCIL_ATTACHMENT, TEXTURE_2D, Some(depth_attachment), 0);

            Downsize{
                width,
                height,
                fbo,
                color_attachment,
                depth_attachment
            }
        }
    }

    pub fn render<'a>(&self, gl : &'a Context, size : PhysicalSize<u32>, renderCallback: impl FnOnce(&'a Context)) {
        unsafe {
            gl.bind_framebuffer(FRAMEBUFFER, Some(self.fbo));
            gl.viewport(0, 0, self.width as i32, self.height as i32);
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            renderCallback(gl);

            gl.bind_framebuffer(FRAMEBUFFER, None);
            gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.fbo));
            gl.bind_framebuffer(DRAW_FRAMEBUFFER, None);
            gl.blit_framebuffer(0, 0, self.width as i32, self.height as i32, 0, 0, size.width as i32, size.height as i32, COLOR_BUFFER_BIT, NEAREST)
        }
    }

    pub fn delete(self, gl : &Context) {
        unsafe { gl.delete_framebuffer(self.fbo) }
    }
}