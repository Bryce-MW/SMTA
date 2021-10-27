pub mod spout;

use std::thread;
use std::thread::JoinHandle;

pub fn create_named_thread<F, T>(name: String, func: F) -> std::io::Result<JoinHandle<T>>
where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static {
    thread::Builder::new().name(name).spawn(|| func())
}
