pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3];
    let mut i = 5;
    while primes.len() as u32 <= n {
        if !primes.iter().filter(|&&p| p <= i / 2).any(|p| i % p == 0) {
            primes.push(i);
        }
        i += 2;
    }

    primes[n as usize]
}
