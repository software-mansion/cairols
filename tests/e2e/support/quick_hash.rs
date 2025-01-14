pub fn quick_hash(input: &str) -> u64 {
    let mut hash = 5381_u64;
    for byte in input.bytes() {
        // Equivalent to `hash * 33 ^ c`
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(u64::from(byte));
    }
    hash
}
