use crate::{game::one_player::OnePlayerGameState, search::one_player::ChooseAction};

pub struct Greedy {}

impl<S> ChooseAction<S> for Greedy
where
    S: OnePlayerGameState,
{
    fn choose(&self, state: &S) -> S::Action {
        let legal_actions = state.legal_actions();
        let mut best_score = 0;
        let mut best_action = None;
        for action in legal_actions {
            let mut next_state = state.clone();
            next_state.advance(action);
            let score = next_state.evaluate_score();
            if best_score <= score {
                best_score = score;
                best_action = Some(action);
            }
        }
        best_action.unwrap()
    }
}
