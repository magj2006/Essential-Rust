use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    assert!(candidate.is_ascii());

    let mut set = HashSet::new();

    candidate.to_lowercase().chars().filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))
}
