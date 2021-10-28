pub mod dispatch {
    use super::super::super::system::dispatch;
    use dispatch::DispatchObject;
    pub mod queue {
        use super::*;
        use dispatch::queue::*;

        pub type Queue = DispatchObject<dispatch_queue>;
        pub type Priority = dispatch::queue::Priority;

        // TODO(bryce): See what changes need to be done to Priority
        pub fn get_global_queue(identifier: Priority, flags: isize) -> Queue {
            unsafe {
                dispatch_get_global_queue(identifier, flags)
            }
        }
    }
}
