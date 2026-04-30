#[cfg(target_os = "windows")]
pub unsafe fn execute_module_stomp(_target_process: *mut libc::c_void, _payload: &[u8]) -> Result<(), &'static str> { Ok(()) }
