use crate::{game::heuristic::HeuristicGameState, search::heuristic::ChooseState};

pub struct HillClimb {
    transitions: usize,
}

impl HillClimb {
    pub fn new(transitions: usize) -> Self {
        Self { transitions }
    }
}

impl<S> ChooseState<S> for HillClimb
where
    S: HeuristicGameState,
{
    fn choose(&self, initial_state: &S) -> S {
        let mut state = initial_state.clone();
        let mut best_score = state.start();
        for _ in 0..self.transitions {
            let mut next_state = state.clone();
            next_state.transition();
            let next_score = next_state.start();
            if best_score < next_score {
                best_score = next_score;
                state = next_state;
            }
        }
        state
    }
}
