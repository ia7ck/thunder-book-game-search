use crate::{game::heuristic::HeuristicGameState, search::heuristic::ChooseState};

pub struct Random {}

impl<S> ChooseState<S> for Random
where
    S: HeuristicGameState,
{
    fn choose(&self, initial_state: &S) -> S {
        let mut state = initial_state.clone();
        state.initialize(); // 初期解を生成しなおす
        state
    }
}
