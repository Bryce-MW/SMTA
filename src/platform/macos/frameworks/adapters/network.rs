use super::super::network;
use network::NWObject;

pub mod parameters {
    use super::*;
    use network::parameters::*;

    pub type ConfigureProtocolBlock<'a> = nw_parameters_configure_protocol_block_t<'a>;
    pub type Parameters = NWObject<nw_parameters>;

    pub fn create_secure_tcp(tls_conf: ConfigureProtocolBlock,
                             tcp_conf: ConfigureProtocolBlock)
        -> Parameters {
        unsafe {
            nw_parameters_create_secure_tcp(tls_conf, tcp_conf)
        }
    }

    pub fn default_configuration() -> ConfigureProtocolBlock<'static> {
        unsafe {
            return _nw_parameters_configure_protocol_default_configuration;
        }
    }
}

pub mod listener {
    use std::ffi::CStr;

    use super::*;
    use network::listener::*;
    use parameters::Parameters;

    use super::super::system::dispatch::queue::Queue;

    pub type Listener = NWObject<nw_listener>;
    pub type State = nw_listener_state;
    pub type StateChangedHandler<'a> = nw_listener_state_changed_handler_t<'a>;

    // IMPORTANT(bryce): We always concat "\0" so from_bytes_with_nul_unchecked is ok
    #[macro_export]
    macro_rules! port {
        ($i:literal) => {unsafe {::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($i, "\0").as_bytes()) }};
    }

    pub fn create_with_port(port: &CStr, parameters: Parameters) -> Listener {
        unsafe {
            nw_listener_create_with_port(port.as_ptr(), parameters)
        }
    }

    pub fn set_queue(listener: Listener, queue: Queue) {
        unsafe {
            nw_listener_set_queue(listener, queue)
        }
    }

    pub fn set_state_changed_handler(listener: Listener, handler: StateChangedHandler) {
        unsafe {
            nw_listener_set_state_changed_handler(listener, handler)
        }
    }
}

pub mod error {
    use super::*;
    use network::error::nw_error;

    pub type Error = NWObject<nw_error>;
}
