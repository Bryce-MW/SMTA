use crate::{port, block, threads::{self, spout::Spout}};
use super::frameworks::adapters::{network::*, system::{*, dispatch::*}};
use super::frameworks::blocks::Block;

pub struct SocketType {

}

pub fn create_listener_thread() -> &'static Spout<SocketType> {
    let connection_spout: &'static Spout<_> = Box::leak(Box::new(Spout::new()));
    // TODO(bryce): Errors
    threads::create_named_thread("Listener".to_string(), || {
        let listener_params = parameters::create_secure_tcp(
            parameters::default_configuration(),
            parameters::default_configuration());
        let listener = listener::create_with_port(port!("993"), listener_params);
        listener::set_queue(listener.clone(),
                            queue::get_global_queue(queue::Priority::LOW, 0));
        listener::set_state_changed_handler(listener,
                                            &<block!((listener::State, error::Error), ())>::new(
            |state: listener::State, error: error::Error| {
            // Something
        }).unsize());

        Some(())
    }).unwrap();

    connection_spout
}
