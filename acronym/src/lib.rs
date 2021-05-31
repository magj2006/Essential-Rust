pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .inspect(|c| println!("1: {}", c))
                    .skip_while(|c| c.is_uppercase())
                    .inspect(|c| println!("2: {}", c))
                    .filter(|c| c.is_uppercase())
                    .inspect(|c| println!("3: {}", c)),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
