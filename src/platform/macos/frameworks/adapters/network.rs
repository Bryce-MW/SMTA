use super::super::network;
use network::NWObject;

pub mod parameters {
    use super::*;
    use network::parameters::*;

    pub type ConfigureProtocolBlock = nw_parameters_configure_protocol_block_t;
    pub type Parameters = NWObject<nw_parameters>;

    pub fn create_secure_tcp(tls_conf: ConfigureProtocolBlock,
                             tcp_conf: ConfigureProtocolBlock)
        -> Parameters {
        unsafe {
            nw_parameters_create_secure_tcp(tls_conf, tcp_conf)
        }
    }

    pub fn default_configuration() -> ConfigureProtocolBlock {
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
}
