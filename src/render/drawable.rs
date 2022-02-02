use glow::Context;
use egui::Ui;
use std::error::Error;

/// This trait is all a struct needs to be rendered to the screen. If they
/// implement this trait, they can be put into the render context. Once in the
/// render context, it can be drawn using the RenderContext's `render()` method.

pub trait Drawable {
    ///This describes what happens when the object is rendered.
    unsafe fn render(&self, gl : &Context);

    unsafe fn destroy(&self, gl : &Context);

    unsafe fn pre_render(&self, gl : &Context) {}

    unsafe fn post_render(&self, gl : &Context) {}

    fn debug(&mut self, ui : &mut Ui) {}
}
