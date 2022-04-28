use cgmath::{Matrix4, Vector2, Vector3, Vector4};
use egui::{DragValue, Ui, Vec2};
use egui::emath::Numeric;

#[derive(Copy, Clone)]
pub enum UIRenderType {
    MUTABLE,
    IMMUTABLE,
    HIDDEN
}

pub trait Debugable {
    fn debug(&mut self, ui : &mut Ui, enabled : bool) -> bool;
}

///Vector3 impl.
impl <T> Debugable for Vector3<T> where T : Numeric {
    fn debug(&mut self, ui: &mut Ui, enabled : bool) -> bool {

        let res = ui.add_enabled_ui(enabled, |ui| {
            ui.columns(3, |ui| {
                match ui { _ => {} }

                ui[0].add(DragValue::new(&mut self.x).speed(0.1)).changed() ||
                    ui[1].add(DragValue::new(&mut self.y).speed(0.1)).changed() ||
                    ui[2].add(DragValue::new(&mut self.z).speed(0.1)).changed()
            })
        });

        res.inner
    }
}

pub fn debug_colorRBG(ui : &mut Ui, enabled : bool,  value : &mut Vector3<f32>) -> bool {
    ui.add_enabled_ui(enabled, |ui| {
        let mut color : [f32; 3] = [value.x, value.y, value.z];
        let changed = ui.color_edit_button_rgb(&mut color).changed();
        if changed {value.x = color[0]; value.y = color[1]; value.z = color[2];}
        changed
    }).inner
}

// impl <T> Debugable for Vector4<T> where T : Numeric {
//     fn debug(&mut self, ui: &mut Ui, render_type : &UIRenderType) {
//         let enabled = if let UIRenderType::MUTABLE = render_type {
//             true
//         } else {
//             false
//         };
//
//         if let UIRenderType::HIDDEN = render_type {return;}
//
//         ui.add_enabled_ui(enabled, |ui| {
//             ui.horizontal(|ui| {
//                 ui.add(DragValue::new(&mut self.x));
//                 ui.add(DragValue::new(&mut self.y));
//                 ui.add(DragValue::new(&mut self.z));
//                 ui.add(DragValue::new(&mut self.w));
//             });
//         });
//     }
// }
//
// impl <T> Debugable for Vector3<T> where T : Numeric {
//     fn debug(&mut self, ui: &mut Ui, render_type : &UIRenderType) {
//         let enabled = if let UIRenderType::MUTABLE = render_type {
//             true
//         } else {
//             false
//         };
//
//         if let UIRenderType::HIDDEN = render_type {return;}
//
//         ui.add_enabled_ui(enabled, |ui| {
//             ui.horizontal(|ui| {
//                 ui.add(DragValue::new(&mut self.x).speed(0.1));
//                 ui.add(DragValue::new(&mut self.y).speed(0.1));
//                 ui.add(DragValue::new(&mut self.z).speed(0.1));
//             });
//         });
//     }
// }
//
// impl <T> Debugable for Vector2<T> where T : Numeric {
//     fn debug(&mut self, ui: &mut Ui, render_type : &UIRenderType) {
//         let enabled = if let UIRenderType::MUTABLE = render_type {
//             true
//         } else {
//             false
//         };
//
//         if let UIRenderType::HIDDEN = render_type {return;}
//
//         ui.add_enabled_ui(enabled, |ui| {
//             ui.horizontal(|ui| {
//                 ui.add(DragValue::new(&mut self.x));
//                 ui.add(DragValue::new(&mut self.y));
//             });
//         });
//     }
// }
//
// impl <T> Debugable for Matrix4<T> where T : Numeric {
//     fn debug(&mut self, ui: &mut Ui, render_type: &UIRenderType) {
//         let enabled = if let UIRenderType::MUTABLE = render_type {
//             true
//         } else {
//             false
//         };
//
//         if let UIRenderType::HIDDEN = render_type {return;}
//
//         ui.add_enabled_ui(enabled, |ui|{
//             ui.horizontal(|ui|{
//                 ui.add(DragValue::new(&mut self.x.x));
//                 ui.add(DragValue::new(&mut self.y.x));
//                 ui.add(DragValue::new(&mut self.z.x));
//                 ui.add(DragValue::new(&mut self.w.x));
//             });
//             ui.horizontal(|ui|{
//                 ui.add(DragValue::new(&mut self.x.y));
//                 ui.add(DragValue::new(&mut self.y.y));
//                 ui.add(DragValue::new(&mut self.z.y));
//                 ui.add(DragValue::new(&mut self.w.y));
//             });
//             ui.horizontal(|ui|{
//                 ui.add(DragValue::new(&mut self.x.z));
//                 ui.add(DragValue::new(&mut self.y.z));
//                 ui.add(DragValue::new(&mut self.z.z));
//                 ui.add(DragValue::new(&mut self.w.z));
//             });
//             ui.horizontal(|ui|{
//                 ui.add(DragValue::new(&mut self.x.w));
//                 ui.add(DragValue::new(&mut self.y.w));
//                 ui.add(DragValue::new(&mut self.z.w));
//                 ui.add(DragValue::new(&mut self.w.w));
//             })
//         });
//     }
// }
//
// impl Debugable for i32 {
//     fn debug(&mut self, ui: &mut Ui, render_type: &UIRenderType) {
//         let enabled = if let UIRenderType::MUTABLE = render_type {
//             true
//         } else {
//             false
//         };
//
//         if let UIRenderType::HIDDEN = render_type {return;}
//
//         ui.add_enabled_ui(enabled, |ui| {
//             ui.add(DragValue::new(self))
//         });
//     }
// }
//
// impl Debugable for f32 {
//     fn debug(&mut self, ui: &mut Ui, render_type: &UIRenderType) {
//         let enabled = if let UIRenderType::MUTABLE = render_type {
//             true
//         } else {
//             false
//         };
//
//         if let UIRenderType::HIDDEN = render_type {return;}
//
//         ui.add_enabled_ui(enabled, |ui| {
//             ui.add(DragValue::new(self))
//         });
//     }
// }