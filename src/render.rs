use glow::{Context, HasContext, FRAMEBUFFER_SRGB};
use glutin::window::Window;
use glutin::event_loop::EventLoop;
use glutin::{ContextWrapper, PossiblyCurrent, WindowedContext};

pub mod frame;
pub mod buffer;
pub mod shader;

pub trait Renderable {
    unsafe fn render(&self, gl : &Context);

    unsafe fn pre_render(&self, gl : &Context) {

    }

    unsafe fn post_render(&self, gl : &Context) {

    }
}

pub fn render<T>(gl : &Context, renderable : &T) where T : Renderable {
    unsafe {
        renderable.pre_render(gl);
        renderable.render(gl);
        renderable.post_render(gl);
    }
}

pub trait Destroyable {
    unsafe fn destroy(&self, gl : &Context);
}

pub trait Debugable {
    
}

pub fn createGlutinContext<'a>(title : &str) -> (Context, &'a str, ContextWrapper<PossiblyCurrent, Window>, EventLoop<()> ) {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let window = glutin::ContextBuilder::new()
            .with_srgb(true)
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        let gl =
            glow::Context::from_loader_function(|s| window.get_proc_address(s));

        {
            use glow::HasContext as _;
            gl.enable(glow::FRAMEBUFFER_SRGB);
        }

        (gl, "#version 410", window, event_loop)
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