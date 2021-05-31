pub fn raindrops(num: u32) -> String {
    let output: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|&&(n, _)| num % n == 0) // whether n is num's factor
        .map(|&(_, s)| s)
        .collect();

    if output.len() == 0 {
        num.to_string()
    } else {
        output
    }
}
