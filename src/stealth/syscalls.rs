#[cfg(target_os = "windows")]
pub unsafe fn direct_syscall_read_memory() -> i32 { 0 }
