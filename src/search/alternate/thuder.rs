use ::std::time::Duration;

use crate::{
    game::alternate::{AlternateGameState, WinningStatus},
    search::alternate::ChooseAction,
    TimeKeeper,
};

pub struct Thunder {
    threshold: Duration,
}

impl Thunder {
    pub fn new(threshold: Duration) -> Self {
        Self { threshold }
    }
}

// MCTS からコピー
impl<S> ChooseAction<S> for Thunder
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> S::Action {
        let time_keeper = TimeKeeper::new(self.threshold);
        let mut root = Node::new(state.clone());
        let legal_actions = state.legal_actions();
        root.expand(&legal_actions);
        for _ in 0.. {
            if time_keeper.time_over() {
                break;
            }
            root.evaluate();
        }
        assert_eq!(legal_actions.len(), root.child_nodes.len());
        let (action, _) = legal_actions
            .into_iter()
            .zip(root.child_nodes)
            .max_by_key(|(_, node)| node.attempt)
            .unwrap();
        action
    }
}

// MCTS からコピー
// Node::evaluate() が少し違う
struct Node<S> {
    state: S,
    attempt: u32,
    win: f64,
    child_nodes: Vec<Node<S>>,
}

impl<S> Node<S>
where
    S: AlternateGameState,
{
    fn new(state: S) -> Self {
        Self {
            state,
            attempt: 0,
            win: 0.0,
            child_nodes: Vec::new(),
        }
    }

    fn expand(&mut self, legal_actions: &Vec<S::Action>) {
        assert!(self.child_nodes.is_empty());
        for &action in legal_actions {
            let mut next_state = self.state.clone();
            next_state.advance(action);
            self.child_nodes.push(Node::new(next_state));
        }
    }

    fn evaluate(&mut self) -> f64 {
        if let Some(status) = self.state.winning_status() {
            let value = match status {
                WinningStatus::Win => 1.0,
                WinningStatus::Draw => 0.5,
                WinningStatus::Lose => 0.0,
            };
            self.win += value;
            self.attempt += 1;
            value
        } else if self.child_nodes.is_empty() {
            let value = self.state.score_rate();
            self.win += value;
            self.expand(&self.state.legal_actions());
            value
        } else {
            let index = 'next_child_node_index: {
                for (i, node) in self.child_nodes.iter().enumerate() {
                    if node.attempt == 0 {
                        break 'next_child_node_index i;
                    }
                }
                (0..self.child_nodes.len())
                    .max_by(|&i, &j| {
                        assert_ne!(self.child_nodes[i].attempt, 0);
                        assert_ne!(self.child_nodes[j].attempt, 0);
                        let left =
                            1.0 - self.child_nodes[i].win / f64::from(self.child_nodes[i].attempt);
                        let right =
                            1.0 - self.child_nodes[j].win / f64::from(self.child_nodes[j].attempt);
                        left.total_cmp(&right)
                    })
                    .unwrap()
            };
            let value = 1.0 - self.child_nodes[index].evaluate();
            self.win += value;
            self.attempt += 1;
            value
        }
    }
}
