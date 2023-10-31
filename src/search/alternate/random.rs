use ::rand::{seq::SliceRandom, thread_rng};

use crate::{game::alternate::AlternateGameState, search::alternate::ChooseAction};

pub struct Random {}

impl<S> ChooseAction<S> for Random
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> S::Action {
        let mut rng = thread_rng();
        state.legal_actions().choose(&mut rng).copied().unwrap()
    }
}
