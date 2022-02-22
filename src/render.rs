use glow::{Context, HasContext, FRAMEBUFFER_SRGB};
use glutin::window::Window;
use glutin::event_loop::EventLoop;
use glutin::{ContextWrapper, PossiblyCurrent, WindowedContext};
use egui_glow::EguiGlow;
use egui::Ui;

pub mod frame;
pub mod buffer;
pub mod shader;
pub mod texture;
pub mod debug;
//pub mod model;

pub trait Renderable {
    unsafe fn render(&self, gl : &Context);
}

pub fn createGlutinContext<'a>(title : &str) -> (Context, &'a str, ContextWrapper<PossiblyCurrent, Window>, EventLoop<()>, EguiGlow) {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let window = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        let gl =
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

        gl.enable(FRAMEBUFFER_SRGB);

        let mut egui_glow = egui_glow::EguiGlow::new(&window, &gl);

        (gl, "#version 410", window, event_loop, egui_glow)
    }
}

pub fn createSurfacelessContext() {
    unsafe {
       //todo create a surfaceless context
    }
}

pub fn disable_shader_program(gl : &Context) {
    unsafe {
        gl.use_program(None)
    }
}