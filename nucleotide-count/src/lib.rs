use std::collections::HashMap;

const BASES: [char; 4] = ['A', 'C', 'T', 'G'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        ch if BASES.contains(&ch) => dna
            .chars()
            .find(|c| !BASES.contains(&c))
            .map_or_else(|| Ok(dna.chars().filter(|&c| c == ch).count()), |c| Err(c)),
        ch @ _ => Err(ch),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hm = HashMap::new();

    for t in BASES.iter() {
        let num = count(*t, dna)?;
        *hm.entry(*t).or_insert(0) += num
    }

    Ok(hm)
}
