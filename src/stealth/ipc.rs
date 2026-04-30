use std::sync::atomic::{AtomicUsize, Ordering};
#[repr(C)]
pub struct CovertRingBuffer {
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
    pub buffer: [u8; 4096],
}
impl CovertRingBuffer {
    pub fn new() -> Self { CovertRingBuffer { head: AtomicUsize::new(0), tail: AtomicUsize::new(0), buffer: [0; 4096] } }
}
