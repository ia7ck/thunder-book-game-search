pub mod beam;
pub mod chokudai;
pub mod greedy;
pub mod random;

use crate::game::one_player::OnePlayerGameState;

pub trait ChooseAction<S>
where
    S: OnePlayerGameState,
{
    fn choose(&self, state: &S) -> S::Action;
}
