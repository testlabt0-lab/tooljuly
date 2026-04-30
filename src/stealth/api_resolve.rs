pub const fn hash_djb2(data: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    let mut i = 0;
    while i < data.len() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(data[i] as u32);
        i += 1;
    }
    hash
}
