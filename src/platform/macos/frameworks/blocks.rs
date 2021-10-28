use core::ffi::c_void;
use core::mem;
use std::{
    mem::MaybeUninit,
    os::raw::{c_int, c_ulong, c_char}
};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::CoerceUnsized;
use std::ptr;
use super::system::block::R_NSConcreteGlobalBlock;

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
        $crate::platform::macos::frameworks::blocks::Block<
            ($($t),*),
            $r,
            $crate::platform::macos::frameworks::blocks::Unknown
        >
    };
}

#[macro_export]
macro_rules! block {
    (($($t:ty),*), $r:ty) => {
        $crate::platform::macos::frameworks::blocks::Block<
            ($($t),*),
            $r,
            (_, $crate::platform::macos::frameworks::blocks::BlockDescriptor)
        >
    };
}

#[repr(C)]
pub struct Block<A, R, D: ?Sized> {
    isa: &'static Class, // initialized to &_NSConcreteStackBlock or &_NSConcreteGlobalBlock
    flags: c_int,
    reserved: MaybeUninit<c_int>,
    invoke: extern "C" fn(&Block<A, R, Unknown>, ...) -> R,
    // TODO(bryce): Am I allowed to have a reference to a potentially invalid type?
    descriptor: *const BlockDescriptor,
    _args: PhantomData<*const A>,
    _data: D
}
impl<A, R, D> Block<A, R, D> {
    pub fn unsize(&self) -> &Block<A, R, Unknown>{
        // IMPORTANT(bryce): The only thing we are doing is making the data from known to unknown.
        //  This struct is repr C so the layout is always known.
        unsafe {
            mem::transmute(&self)
        }
    }
}

macro_rules! block_impl {
    ($n:ident, ($($t:ident),*)) => {
        extern "C" fn $n<Closure: Fn($($t),*) -> R, R, $($t),*>(block: &Block<($($t),*), R, (Closure, BlockDescriptor)>, $($t: $t),*) -> R {
            block._data.0($($t),*)
        }
        impl<Closure: Fn($($t),*) -> R, R, $($t),*> Block<($($t),*), R, (Closure, BlockDescriptor)> {
            pub fn new(func: Closure) -> Self {
                let mut block = Block {
                    isa: unsafe { R_NSConcreteGlobalBlock() },
                    flags: BLOCK_HAS_DESCRIPTOR,
                    reserved: MaybeUninit::uninit(),
                    invoke: unsafe { mem::transmute($n::<Closure, R, $($t),*> as extern "C" fn(block: &Block<($($t),*), R, (Closure, BlockDescriptor)>, $($t: $t),*) -> R) },
                    descriptor: ptr::null(),
                    _args: PhantomData,
                    _data: (func, BlockDescriptor {
                        reserved: 0,
                        size: size_of::<Self>() as c_ulong,
                        copy_helper: None,
                        dispose_helper: None,
                        signature: ptr::null()
                    })
                };
                block.descriptor = &block._data.1;
                block
            }
        }
    };
}

block_impl!(block_invoke_, ());
block_impl!(block_invoke_A0A1, (A0, A1));

// TODO(bryce): Is size always a ulong or is it usize?
#[repr(C)]
pub struct BlockDescriptor {
    reserved: c_ulong, // NULL
    size: c_ulong, // sizeof(struct Block)
    // TODO(bryce): How do we properly specify that these may not even be allocated?
    // optional helper functions
    copy_helper: Option<extern fn(dst: *mut c_void, src: *mut c_void)>, // IFF (1<<25)
    dispose_helper: Option<extern fn(src: *mut c_void)>, // IFF (1<<25)
    // required ABI.2010.3.16
    signature: *const c_char // IFF (1<<30)
}
