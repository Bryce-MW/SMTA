#[cfg(any(target_os = "macos", target_os = "ios"))]
mod macos;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use macos as platform_impl;

// type SocketType
pub use platform_impl::listener::SocketType;
// fn create_listener_thread() -> &'static Spout<SocketType>
pub use platform_impl::listener::create_listener_thread;

#[macro_export]
macro_rules! opaque {
    ($i:ident) => {
        extern {
            pub type $i;
        }
        // pub struct $i {
        //     _data: [u8; 0],
        //     _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
        // }
    };
}
