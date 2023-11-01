use ::std::time::Duration;

use crate::{
    game::alternate::{AlternateGameState, WinningStatus},
    search::alternate::{random::Random, ChooseAction},
    TimeKeeper,
};

pub struct PrimitiveMontecarlo {
    threshold: Duration,
}

impl PrimitiveMontecarlo {
    pub fn new(threshold: Duration) -> Self {
        Self { threshold }
    }
}

impl<S> ChooseAction<S> for PrimitiveMontecarlo
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> S::Action {
        let time_keeper = TimeKeeper::new(self.threshold);
        let legal_actions = state.legal_actions();
        let mut values = vec![0.0; legal_actions.len()];
        let mut counts = vec![0; legal_actions.len()];
        for i in 0.. {
            if time_keeper.time_over() {
                break;
            }
            let i = i % legal_actions.len();
            let mut next_state = state.clone();
            next_state.advance(legal_actions[i]);
            values[i] += 1.0 - playout(&mut next_state);
            counts[i] += 1;
        }
        let arg_max = (0..legal_actions.len())
            .max_by(|&i, &j| {
                let left = values[i] / f64::from(counts[i]);
                let right = values[j] / f64::from(counts[j]);
                left.total_cmp(&right)
            })
            .unwrap();
        legal_actions[arg_max]
    }
}

pub fn playout<S>(state: &mut S) -> f64
where
    S: AlternateGameState,
{
    match state.winning_status() {
        Some(status) => match status {
            WinningStatus::Win => 1.0,
            WinningStatus::Draw => 0.5,
            WinningStatus::Lose => 0.0,
        },
        None => {
            state.advance((Random {}).choose(state));
            1.0 - playout(state)
        }
    }
}
