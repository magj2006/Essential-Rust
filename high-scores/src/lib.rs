#[derive(Debug)]
pub struct HighScores {
    high_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { high_scores: Vec::from(scores) }
    }

    pub fn scores(&self) -> &[u32] {
        &self.high_scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        match self.high_scores.last() {
            Some(&x) => Some(x),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.high_scores.iter().max() {
            Some(&x) => Some(x),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.high_scores.clone();

        v.sort_by(|x, y| y.cmp(x));

        v.iter().take(3).cloned().collect()
    }
}
