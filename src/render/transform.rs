use cgmath::{Matrix4, Quaternion, Vector3};
use egui::emath::Numeric;

#[derive(Clone, Copy)]
struct Transform<T> where T : Numeric {
    pos : Vector3<T>,
    scale: Vector3<T>,
    rotation : Quaternion<T>
}

impl <T> Transform<T> where T : Numeric {

    pub fn get_mat(&self) -> Matrix4<T> {

    }

}

impl <T> Into<Matrix4<T>> for Transform<T> where T : Numeric {
    fn into(self) -> Matrix4<T> {
        todo!()
    }
}