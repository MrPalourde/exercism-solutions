#[derive(Debug)]
pub struct HighScores<'a> {
    all_scores: &'a [u32],
    latest_score: Option<u32>,
    personal_best: Option<u32>,
    personal_top_three: Vec<u32>
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let mut scores_to_vec: Vec<u32> = scores.to_vec(); 
        scores_to_vec.sort();
        scores_to_vec.reverse();
        HighScores {
            all_scores: scores,
            latest_score: scores.last().copied(),
            personal_best: scores_to_vec.first().copied(),
            personal_top_three: scores_to_vec.iter().take(3).copied().collect::<Vec<u32>>(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.all_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.latest_score
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.personal_best
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.personal_top_three.clone()
    }
}
