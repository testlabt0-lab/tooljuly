pub mod unix;
pub mod stomping;
pub fn hijack_thread(_pid: u32, _payload_addr: usize) -> Result<(), &'static str> { Ok(()) }
