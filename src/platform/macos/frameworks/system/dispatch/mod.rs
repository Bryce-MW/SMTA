pub mod queue;
pub mod object;

use super::super::object::{RetainRelease, Object};
use object::*;

use std::ffi::c_void;
use std::ptr::NonNull;

pub type DispatchObject<T> = Object<T, RetainerReleaser>;

pub struct RetainerReleaser;
impl RetainRelease for RetainerReleaser {
    unsafe fn retain(obj: NonNull<c_void>) {
        dispatch_retain(obj)
    }

    unsafe fn release(obj: NonNull<c_void>) {
        dispatch_release(obj)
    }
}
