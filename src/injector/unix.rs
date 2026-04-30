#[cfg(unix)]
pub fn inject_stealth_linux(_pid: i32, _payload: &[u8]) -> Result<(), &'static str> { Ok(()) }
