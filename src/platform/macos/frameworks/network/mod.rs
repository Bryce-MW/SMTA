pub mod parameters;
pub mod nw_object;
pub mod listener;
pub mod protocol_options;

use super::object::{RetainRelease, Object};
use nw_object::{nw_retain, nw_release};

use std::ffi::c_void;
use std::ptr::NonNull;

pub type NWObject<T> = Object<T, RetainerReleaser>;

pub struct RetainerReleaser;
impl RetainRelease for RetainerReleaser {
    unsafe fn retain(obj: NonNull<c_void>) {
        nw_retain(obj);
    }

    unsafe fn release(obj: NonNull<c_void>) {
        nw_release(obj)
    }
}
