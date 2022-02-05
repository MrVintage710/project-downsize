use std::any::TypeId;

pub mod bitflag;

pub fn is_type<'l, A: 'static, B: 'l>(a : A) -> bool {
    TypeId::of::<A>() == TypeId::of::<B>()
}