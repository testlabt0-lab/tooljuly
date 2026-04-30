#[cfg(target_os = "windows")]
pub unsafe fn set_hardware_hook(_target: usize, _detour: usize) -> Result<(), &'static str> { Ok(()) }
