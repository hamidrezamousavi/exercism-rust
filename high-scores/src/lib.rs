#[derive(Debug)]
pub struct HighScores<'a>{
    scores:&'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
       Self{scores}
    }

    pub fn scores(&self) -> &[u32] {
        return self.scores;
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last(){
            Some(&x) => Some(x),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None;
        }
        let mut temp = self.scores[0];
        for item in self.scores{
            if *item > temp{
                temp = *item;
            }
        }
        return Some(temp);
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = Vec::from(self.scores);
        v.sort();
        v.reverse();
        v.truncate(3);
        v
    }
}
