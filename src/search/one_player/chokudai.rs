use ::std::{collections::BinaryHeap, time::Duration};

use crate::{game::one_player::OnePlayerGameState, search::one_player::ChooseAction, TimeKeeper};

pub struct Chokudai {
    beam_width: usize,
    beam_depth: usize,
    threshold: Duration,
}

impl Chokudai {
    pub fn new(beam_width: usize, beam_depth: usize, threshold: Duration) -> Self {
        Self {
            beam_width,
            beam_depth,
            threshold,
        }
    }
}

impl<S> ChooseAction<S> for Chokudai
where
    S: OnePlayerGameState + Ord,
    S::Action: Ord,
{
    fn choose(&self, state: &S) -> S::Action {
        let time_keeper = TimeKeeper::new(self.threshold);
        let mut heaps = vec![BinaryHeap::new(); self.beam_depth + 1];
        heaps[0].push((state.clone(), None));
        'outer: loop {
            for t in 0..self.beam_depth {
                for _ in 0..self.beam_width {
                    if time_keeper.time_over() {
                        break 'outer;
                    }
                    let Some((now_state, first_action)) = heaps[t].pop() else {
                        break;
                    };
                    if now_state.done() {
                        // 戻す
                        heaps[t].push((now_state, first_action));
                        break;
                    }
                    let legal_actions = now_state.legal_actions();
                    for action in legal_actions {
                        let mut next_state = now_state.clone();
                        next_state.advance(action);
                        next_state.evaluate_score();
                        heaps[t + 1].push((next_state, first_action.or(Some(action))));
                    }
                }
            }
        }
        for t in (0..=self.beam_depth).rev() {
            if let Some((_, action)) = heaps[t].peek() {
                return action.unwrap();
            }
        }
        unimplemented!("no action")
    }
}
