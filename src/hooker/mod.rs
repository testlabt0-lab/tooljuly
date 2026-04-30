pub mod hardware;
pub mod ssl_bypass;
pub mod hypervisor;
pub fn setup_engine() {}
pub fn hardware_hook(_target_addr: usize, _detour_addr: usize) -> Result<(), &'static str> { Ok(()) }
