use cgmath::{BaseFloat, Vector3};

// pub struct GlobalLighting<F> where F : BaseFloat{
//     color : Vector3<F>,
//     direction : Vector3<F>,
//     ambient_min : f32
// }
//
// impl <F : BaseFloat> GlobalLighting<F> {
//     pub fn new<T>(color : T, direction : T, ambient_min : F) -> Self where T : Into<Vector3<F>> {
//         GlobalLighting {
//             color : color.into(),
//             direction : direction.into(),
//             ambient_min
//         }
//     }
//
//
// }
//
// impl <F : BaseFloat> Default for GlobalLighting<F> {
//     fn default() -> Self {
//         GlobalLighting {
//             color: Vector3::new(1.0, 1.0, 1.0),
//             direction: Vector3::new(0.0, -1.0, 1.0),
//             ambient_min: 0.2
//         }
//     }
// }