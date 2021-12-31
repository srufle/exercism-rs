#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(x) => Some(*x),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        let mut best = 0;
        for score in self.scores {
            if score >= &best {
                best = *score
            }
        }

        Some(best)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three: Vec<u32> = Vec::new();
        for score in self.scores {
            top_three.push(*score);
        }
        // Sort descending
        top_three.sort_by(|a, b| b.cmp(a));
        top_three.truncate(3);
        top_three
    }
}
