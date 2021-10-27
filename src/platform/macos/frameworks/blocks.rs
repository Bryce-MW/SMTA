use core::ffi::c_void;
use std::{
    mem::MaybeUninit,
    os::raw::{c_int, c_ulong, c_char}
};

use super::system::Class;

// Bitflags enum
const BLOCK_REFCOUNT_MASK:    c_int = (0xffff);
const BLOCK_NEEDS_FREE:       c_int = (1 << 24);
const BLOCK_HAS_COPY_DISPOSE: c_int = (1 << 25);
const BLOCK_HAS_CTOR:         c_int = (1 << 26); /* Helpers have C++ code. */
const BLOCK_IS_GC:            c_int = (1 << 27);
const BLOCK_IS_GLOBAL:        c_int = (1 << 28);
const BLOCK_HAS_DESCRIPTOR:   c_int = (1 << 29);

// NOTE(bryce): This is a type which is not Sized but pointers to it
//  are Thin. This may not be actually what we want but it should work for now
extern { pub type Unknown; }

#[macro_export]
macro_rules! ext_block {
    (($($t:ty),*), $r:ty) => {
        &'static $crate::platform::macos::frameworks::blocks::Block<
            extern "C" fn(
                &$crate::platform::macos::frameworks::blocks::Block<extern "C" fn(),
                $crate::platform::macos::frameworks::blocks::Unknown>,
                $($t),+
            ) ->$r,
            $crate::platform::macos::frameworks::blocks::Unknown
        >
    };
}

#[repr(C)]
pub struct Block<F, D: ?Sized> {
    isa: &'static Class, // initialized to &_NSConcreteStackBlock or &_NSConcreteGlobalBlock
    flags: c_int,
    reserved: MaybeUninit<c_int>,
    // TODO(bryce): Should this be extern function. I can't really "spread" P so IDK how to do this.
    invoke: F,
    // TODO(bryce): Am I allowed to have a reference to a potentially invalid type?
    descriptor: &'static BlockDescriptor,
    _imports: D
}

// TODO(bryce): Is size always a ulong or is it usize?
#[repr(C)]
struct BlockDescriptor {
    reserved: c_ulong, // NULL
    size: c_ulong, // sizeof(struct Block)
    // TODO(bryce): How do we properly specify that these may not even be allocated?
    // optional helper functions
    copy_helper: fn(dst: *mut c_void, src: *mut c_void), // IFF (1<<25)
    dispose_helper: fn(src: *mut c_void), // IFF (1<<25)
    // required ABI.2010.3.16
    signature: *const c_char // IFF (1<<30)
}
