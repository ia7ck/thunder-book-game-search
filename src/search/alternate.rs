pub mod alpha_beta;
pub mod iterative_deepening_alpha_beta;
pub mod mcts;
pub mod mini_max;
pub mod primitive_montecarlo;
pub mod random;
pub mod thuder;

use crate::game::alternate::AlternateGameState;

pub trait ChooseAction<S>
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> S::Action;
}
