#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if dna.chars().all(|c| "CGTA".contains(c)) {
            Ok(Dna {
                dna: String::from(dna),
            })
        } else {
            Err(dna.chars().position(|c| !"CGTA".contains(c)).unwrap_or(0))
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(
            self.dna
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => 'X',
                })
                .collect::<String>()
                .as_str(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if rna.chars().all(|c| "CGAU".contains(c)) {
            Ok(Rna {
                rna: String::from(rna),
            })
        } else {
            Err(rna.chars().position(|c| !"CGAU".contains(c)).unwrap_or(0))
        }
    }
}
