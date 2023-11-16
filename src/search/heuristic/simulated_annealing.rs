use ::rand::thread_rng;
use rand::Rng;

use crate::{game::heuristic::HeuristicGameState, search::heuristic::ChooseState};

pub struct SimulatedAnnealing {
    transitions: usize,
    start_temperature: f64,
    end_temperature: f64,
}

impl SimulatedAnnealing {
    pub fn new(transitions: usize, start_temperature: f64, end_temperature: f64) -> Self {
        Self {
            transitions,
            start_temperature,
            end_temperature,
        }
    }
}

impl<S> ChooseState<S> for SimulatedAnnealing
where
    S: HeuristicGameState,
{
    fn choose(&self, initial_state: &S) -> S {
        let mut rng = thread_rng();
        let mut state = initial_state.clone();
        let mut score = state.start();
        let mut best_state = state.clone();
        let mut best_score = score;
        for i in 0..self.transitions {
            let mut next_state = state.clone();
            next_state.transition();
            let next_score = next_state.start();
            let t = self.start_temperature
                + (self.end_temperature - self.start_temperature)
                    * (i as f64 / self.transitions as f64);
            if score < next_score
                || rng.gen_bool(f64::exp((f64::from(next_score) - f64::from(score)) / t))
            {
                score = next_score;
                state = next_state;
                if best_score < score {
                    best_score = score;
                    best_state = state.clone();
                }
            }
        }
        best_state
    }
}
