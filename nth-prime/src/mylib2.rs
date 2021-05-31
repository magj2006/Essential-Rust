use std::ops::RangeFrom;

pub fn nth(n: u32) -> u32 {
    let is_prime = |x| !(2..=x / 2).any(|y| x % y == 0);
    (RangeFrom { start: 3 })
        .skip(2)
        .filter(|&i| is_prime(i))
        .nth((n - 2) as usize)
        .unwrap()
}
