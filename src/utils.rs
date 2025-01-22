use std::any::{Any, TypeId};

pub fn typeid_of_val<T: ?Sized + Any>(_t: &T) -> TypeId {
    TypeId::of::<T>()
}
