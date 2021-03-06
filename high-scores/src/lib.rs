#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

// Insired by
// https://exercism.org/tracks/rust/exercises/high-scores/solutions/menb111
impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // replaced match with *cloned*, I had previously tried clone
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // Was using a for loop to do what I think max does
        // in the iterator
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = self.scores.to_vec();

        // Sort descending
        top_three.sort_by(|a, b| b.cmp(a));
        top_three.truncate(3);
        top_three
    }
}
