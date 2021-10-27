use crate::{port, threads::{self, spout::Spout}};
use super::frameworks::adapters::{network::*, system::{*, dispatch::*}};
use super::frameworks::system::dispatch::queue::Priority;

pub struct SocketType {

}

pub fn create_listener_thread() -> &'static Spout<SocketType> {
    let connection_spout: &'static Spout<_> = Box::leak(Box::new(Spout::new()));
//NW_PARAMETERS_DEFAULT_CONFIGURATION
    // TODO(bryce): Errors
    threads::create_named_thread("Listener".to_string(), || -> Option<()> {
        let listener_params = parameters::create_secure_tcp(
            parameters::default_configuration(),
            parameters::default_configuration());
        let listener = listener::create_with_port(port!("993"), listener_params);
        listener::set_queue(listener,
                            queue::get_global_queue(Priority::LOW, 0));

        Some(())
    }).unwrap();

    connection_spout
}
