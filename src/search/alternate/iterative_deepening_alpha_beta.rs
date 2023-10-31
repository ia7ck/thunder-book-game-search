use ::std::time::Duration;

use crate::{
    game::alternate::AlternateGameState,
    search::alternate::{alpha_beta::alpha_beta_inner, ChooseAction},
    TimeKeeper,
};

pub struct IterativeDeepeningAlphaBeta {
    threshold: Duration,
}

impl IterativeDeepeningAlphaBeta {
    pub fn new(threshold: Duration) -> Self {
        Self { threshold }
    }
}

impl<S> ChooseAction<S> for IterativeDeepeningAlphaBeta
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> S::Action {
        let time_keeper = TimeKeeper::new(self.threshold);
        let mut best_action = None;
        for depth in 1.. {
            if let Some(action) = alpha_beta_inner(state, depth, &time_keeper) {
                best_action = Some(action);
            } else {
                return best_action.unwrap_or_else(|| panic!("深さ1の探索でも時間切れ"));
            }
        }
        unreachable!()
    }
}
