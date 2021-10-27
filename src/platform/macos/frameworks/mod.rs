pub mod adapters;
pub mod blocks;
pub mod network;
pub mod system;

pub mod object {
    use core::ffi::c_void;
    use std::marker::PhantomData;
    use std::ptr::NonNull;

    use super::*;
    use network::nw_object::*;

    #[repr(transparent)]
    pub struct Object<T: ?Sized, D: RetainRelease>(Option<NonNull<T>>, PhantomData<*const D>);
    impl<T: ?Sized, D: RetainRelease> Drop for Object<T, D> {
        fn drop(&mut self) {
            match self.0 {
                None => {}
                Some(obj) => unsafe {
                    D::release(obj.cast());
                }
            }
        }
    }

    pub trait RetainRelease {
        unsafe fn retain(obj: NonNull<c_void>);
        unsafe fn release(obj: NonNull<c_void>);
    }
}
