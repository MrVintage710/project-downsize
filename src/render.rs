use glow::{Context, HasContext, FRAMEBUFFER_SRGB};
use glutin::window::Window;
use glutin::event_loop::EventLoop;
use glutin::{ContextWrapper, PossiblyCurrent, WindowedContext};
use egui::Ui;
use egui_glow::EguiGlow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};
use egui::epaint::ClippedShape;
use glutin::event::WindowEvent;
use crate::render::drawable::Drawable;
use crate::util::*;
use std::collections::HashMap;
use std::any::TypeId;

pub mod frame;
pub mod buffer;
pub mod shader;
pub mod drawable;
pub mod model;

///----------------------------- Render Context -------------------------------
/// This is the object that holds all of the render specific information.

pub struct RenderContext {
    gl : Context,
    egui : EguiGlow,
    event_loop: EventLoop<()>,
    window: ContextWrapper<PossiblyCurrent, Window>,
    drawables : HashMap<String, Box<dyn Drawable>>,
    render_list : Vec<(u32, String)>
}

impl RenderContext {
    pub fn new(title : &str) -> Self {
        let (gl, glsl_version, window, event_loop) = createGlutinContext(title);
        let mut egui = egui_glow::EguiGlow::new(&window, &gl);

        RenderContext {
            gl,
            egui,
            window,
            event_loop,
            drawables : HashMap::new(),
            render_list : Vec::new()
        }
    }

    pub fn on_event(&mut self, event : &WindowEvent<'_>) {
        self.egui.on_event(event);
    }

    pub fn debug(&mut self) {
        let (should_render, list) = self.egui.run(self.window.window(), |egui_ctx| {
            egui::SidePanel::left("side_panel").show(egui_ctx, |ui|{
                for rle in self.render_list.iter_mut() {
                    let mut drawable = self.drawables.get(&*rle.1).unwrap();
                    ui.collapsing(&*rle.1, |ui| {
                        drawable.debug(ui);
                    });
                }
            });
        });

        if should_render {
            self.egui.paint(&self.window, &self.gl, list);
        }
    }

    pub fn render(&self) {
        unsafe {
            for rle in self.render_list.iter() {
                let drawable = self.drawables.get(&*rle.1).unwrap();

                drawable.pre_render(&self.gl);
                drawable.render(&self.gl);
                drawable.post_render(&self.gl);
            }
        }
    }

    pub fn add_drawable<T: 'static, F>(&mut self, creation_callback : F) -> &Box<dyn Drawable> where T: Drawable, F: FnOnce(&Context) -> (&str, u32, T) {
        let (name, priority, drawable) = creation_callback(&self.gl);

        self.drawables.insert(name.to_string(), Box::new(drawable));
        self.render_list.push((priority, name.to_string()));
        self.render_list.sort_by(|a, b| {a.0.cmp(&b.0)});
        return self.drawables.get(name).unwrap();
    }

    pub fn mutate_drawable<T>(&mut self, name : &str, mutator : impl FnOnce(&mut T)) {
        let b = self.drawables.get_mut(name).unwrap();
        let test = b.as_any_mut().downcast_mut::<T>().unwrap();
        mutator(test)
    }

    pub fn render_drawable<T>(&self, drawable : T) where T: Drawable {
        unsafe {
            drawable.pre_render(&self.gl);
            drawable.render(&self.gl);
            drawable.post_render(&self.gl);
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            for rle in self.render_list.iter() {
                let drawable = self.drawables.get(&*rle.1).unwrap();

                drawable.destroy(&self.gl);
            }

            self.egui.destroy(&self.gl);
        }
    }
}

///----------------------------- Render List Element -------------------------------
/// This struct will allow us to sort the render list so that some things render
/// first.

pub struct RenderListElement {
    pub drawable : Box<dyn Drawable>,
    pub priority : u32,
    pub name : String
}

impl PartialEq for RenderListElement {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for RenderListElement {}

impl Ord for RenderListElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for RenderListElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for RenderListElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.name, self.priority)
    }
}

impl Display for RenderListElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.name, self.priority)
    }
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

pub fn disable_shader_program(gl : &Context) {
    unsafe {
        gl.use_program(None)
    }
}