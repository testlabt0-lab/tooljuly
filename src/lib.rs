//! Phantom Core: The Ultimate Stealth Instrumentation Engine
pub mod ebpf_tracer;
pub mod scripting;
pub mod stealth;
pub mod hooker;
pub mod injector;

pub fn initialize_phantom() {
    stealth::apply_anti_detection();
    hooker::setup_engine();
    println!("[+] Phantom Core Initialized in Ultimate Stealth Mode");
}
