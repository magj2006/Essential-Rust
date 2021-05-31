pub fn factors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut m = n;
    let mut prime = 2;
    while m > 1 {
        if m % prime == 0 {
            m /= prime;
            v.push(prime);
        } else if prime < 3 {
                prime += 1;
        } else {
                prime += 2;
        }
    }

    v
}
