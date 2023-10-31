use ::std::{collections::BinaryHeap, time::Duration};

use crate::{game::one_player::OnePlayerGameState, search::one_player::ChooseAction, TimeKeeper};

pub struct Beam {
    beam_width: usize,
    threshold: Duration,
}

impl Beam {
    pub fn new(beam_width: usize, threshold: Duration) -> Self {
        Self {
            beam_width,
            threshold,
        }
    }
}

impl<S> ChooseAction<S> for Beam
where
    S: OnePlayerGameState + Ord,
    S::Action: Ord,
{
    fn choose(&self, state: &S) -> S::Action {
        let time_keeper = TimeKeeper::new(self.threshold);
        let mut best_action = None;
        let mut heap = BinaryHeap::new();
        heap.push((state.clone(), best_action));
        loop {
            let mut new_heap = BinaryHeap::new();
            for _ in 0..self.beam_width {
                if time_keeper.time_over() {
                    return best_action.unwrap();
                }
                let Some((now_state, first_action)) = heap.pop() else {
                    break;
                };
                let legal_actions = now_state.legal_actions();
                for action in legal_actions {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    new_heap.push((next_state, first_action.or(Some(action))));
                }
            }
            heap = new_heap;
            if let Some((state, action)) = heap.peek() {
                best_action = *action;
                if state.done() {
                    break;
                }
            }
        }
        best_action.unwrap()
    }
}
