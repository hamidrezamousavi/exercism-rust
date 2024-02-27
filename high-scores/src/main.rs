use high_scores::HighScores;
fn main(){
    let s = HighScores::new(&[]);
    println!("{:?}", s.personal_top_three());
    println!("{:?}", s);

}