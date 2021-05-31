pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64, got {}", s)
    }

    1u64 << (s - 1)        
}

pub fn total() -> u64 {
    ((1u128 << 64) - 1) as u64
}
