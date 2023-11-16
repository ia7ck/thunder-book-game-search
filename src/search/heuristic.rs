pub mod hill_climb;
pub mod random;

use crate::game::heuristic::HeuristicGameState;

pub trait ChooseState<S>
where
    S: HeuristicGameState,
{
    fn choose(&self, initial_state: &S) -> S;
}
