pub trait OnePlayerGameState: Clone {
    type Action: Clone + Copy;
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn advance(&mut self, action: Self::Action);
    fn done(&self) -> bool;
    fn evaluate_score(&self) -> u32;
}
