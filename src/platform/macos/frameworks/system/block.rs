use core::ffi::c_void;
use core::mem;
use super::Class;

// use super::Availability::*;
// use super::TargetConditionals::*;

// // Create a heap based copy of a Block or simply add a reference to an existing one.
// // This must be paired with Block_release to recover memory, even when running
// // under Objective-C Garbage Collection.
//     BLOCK_EXPORT void *_Block_copy(const void *aBlock)
//     __OSX_AVAILABLE_STARTING(__MAC_10_6, __IPHONE_3_2);
//
// // Lose the reference, and if heap based and last reference, recover the memory
//     BLOCK_EXPORT void _Block_release(const void *aBlock)
//     __OSX_AVAILABLE_STARTING(__MAC_10_6, __IPHONE_3_2);
//
//
// // Used by the compiler. Do not call this function yourself.
//     BLOCK_EXPORT void _Block_object_assign(void *, const void *, const int)
//     __OSX_AVAILABLE_STARTING(__MAC_10_6, __IPHONE_3_2);
//
// // Used by the compiler. Do not call this function yourself.
//     BLOCK_EXPORT void _Block_object_dispose(const void *, const int)
//     __OSX_AVAILABLE_STARTING(__MAC_10_6, __IPHONE_3_2);
//
#[link(name = "System", kind = "framework")]
extern "C" {
    // Used by the compiler. Do not use these variables yourself.
    static _NSConcreteGlobalBlock: [*mut c_void; 32];
    static _NSConcreteStackBlock: [*mut c_void; 32];
}
pub unsafe fn R_NSConcreteGlobalBlock() -> &'static Class {
    mem::transmute(&_NSConcreteGlobalBlock)
}

pub unsafe fn R_NSConcreteStackBlock() -> &'static Class {
    mem::transmute(&_NSConcreteStackBlock)
}

// // Type correct macros
//
// #define Block_copy(...) ((__typeof(__VA_ARGS__))_Block_copy((const void *)(__VA_ARGS__)))
// #define Block_release(...) _Block_release((const void *)(__VA_ARGS__))
