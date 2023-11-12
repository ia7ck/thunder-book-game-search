pub trait AlternateGameState: Clone {
    type Action: Clone + Copy;
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn advance(&mut self, action: Self::Action);
    fn done(&self) -> bool;
    fn score(&self) -> i16 {
        unimplemented!("盤面評価を必要とするαβ探索などのために実装する")
    }
    fn winning_status(&self) -> Option<WinningStatus>;
}

#[derive(Clone, Copy)]
pub enum WinningStatus {
    Win,
    Draw,
    Lose,
}
