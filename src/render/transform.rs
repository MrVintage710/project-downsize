use std::borrow::BorrowMut;
use cgmath::{Matrix4, Quaternion, Vector3, SquareMatrix, Zero, Deg, Angle, Rad, BaseFloat, Transform as TransformMatrix};
use egui::emath::Numeric;
use crate::render::debug::{Debugable, UIRenderType};
use egui::Ui;
use cgmath::Rotation3;
use crate::render::shader::{ShaderUniformHandler, Uniform};
use crate::util::math::wrap_vec3;
use crate::util::variable::UpdateVariable;

#[derive(Clone)]
pub struct Transform {
    pos : Vector3<f32>,
    scale: Vector3<f32>,
    rotation : Vector3<f32>,
    origin : Vector3<f32>,
    mat : Matrix4<f32>,
    mat_has_changed: bool,
    uniform_handler: Option<ShaderUniformHandler>
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0),
            mat : Matrix4::identity(),
            mat_has_changed: true,
            uniform_handler: None
        }
    }

    pub fn get_mat(&mut self) -> Matrix4<f32> {
        if self.mat_has_changed {
            self.mat = self.calc_mat();
            self.mat_has_changed = false;
            if self.uniform_handler.is_some() {
                self.uniform_handler.unwrap().update_uniform(Matrix4(self.mat))
            }
        }

        self.mat
    }

    pub fn calc_mat(&self) -> Matrix4<f32> {
        let origin_mat = Matrix4::from_translation(self.origin);
        let transform_mat = Matrix4::from_translation(self.pos);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation_x_mat = Matrix4::from_angle_x(Deg(self.rotation.x));
        let rotation_y_mat = Matrix4::from_angle_y(Deg(self.rotation.y));
        let rotation_z_mat = Matrix4::from_angle_z(Deg(self.rotation.z));
        let mat =  transform_mat * rotation_x_mat * rotation_y_mat * rotation_z_mat  * scale_mat * origin_mat;
        mat
    }

    pub fn calc_cam_mat(&self) -> Matrix4<f32> {
        let origin_mat = Matrix4::from_translation(-self.origin);
        let transform_mat = Matrix4::from_translation(-self.pos);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation_x_mat = Matrix4::from_angle_x(Deg(self.rotation.x));
        let rotation_y_mat = Matrix4::from_angle_y(Deg(self.rotation.y));
        let rotation_z_mat = Matrix4::from_angle_z(Deg(self.rotation.z));
        let mat =  transform_mat * rotation_x_mat * rotation_y_mat * rotation_z_mat  * scale_mat * origin_mat;
        mat
    }

    pub fn set_pos<T>(&mut self, value : T) -> &mut Self where T: Into<Vector3<f32>> {
        self.pos = value.into();
        self.mat_has_changed = true;
        self
    }

    pub fn set_rot<T>(&mut self, value : T) -> &mut Self where T: Into<Vector3<f32>> {
        self.rotation = value.into();
        wrap_vec3(self.rotation.borrow_mut(), 0.0, 360.0);
        self.mat_has_changed = true;
        self
    }

    pub fn set_scale<T>(&mut self, value : T) -> &mut Self where T: Into<Vector3<f32>> {
        self.scale = value.into();
        self.mat_has_changed = true;
        self
    }

    pub fn add_pos<T>(&mut self, value : T) -> &mut Self where T : Into<Vector3<f32>> {
        self.pos += value.into();
        self.mat_has_changed = true;
        self
    }

    pub fn add_rot<T>(&mut self, value : T) -> &mut Self where T : Into<Vector3<f32>> {
        self.rotation += value.into();
        wrap_vec3(self.rotation.borrow_mut(), 0.0, 360.0);
        self.mat_has_changed = true;
        self
    }
}

impl Into<Matrix4<f32>> for Transform {
    fn into(mut self) -> Matrix4<f32> {
        self.get_mat()
    }
}

impl Uniform for Transform {
    fn provide_handle(&mut self, handle: ShaderUniformHandler) {
        unsafe { self.uniform_handler = Some(handle) }
    }

    fn get_id(&self) -> String {
        "transform".to_owned()
    }
}

impl Debugable for Transform {
    fn debug(&mut self, ui: &mut Ui, render_type: &UIRenderType) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.label("Position");
                self.pos.debug(ui, render_type)
            });
            ui.horizontal(|ui| {
                ui.label("Scale");
                self.scale.debug(ui, render_type);
            });
            ui.horizontal(|ui| {
                ui.label("Rotation");
                self.rotation.debug(ui, render_type);
                wrap_vec3(self.rotation.borrow_mut(), 0.0, 360.0)
            });
            ui.horizontal(|ui| {
                ui.label("Origin");
                self.origin.debug(ui, render_type)
            });
        });
    }
}

///This a transform meant to control a camera. It wraps transform and inverts all incoming changes.
pub struct CameraTransform {

}