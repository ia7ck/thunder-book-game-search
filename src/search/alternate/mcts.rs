use ::std::time::Duration;

use crate::{
    game::alternate::{AlternateGameState, WinningStatus},
    search::alternate::{primitive_montecarlo::playout, ChooseAction},
    TimeKeeper,
};

#[allow(clippy::upper_case_acronyms)]
pub struct MCTS {
    threshold: Duration,
}

impl MCTS {
    pub fn new(threshold: Duration) -> Self {
        Self { threshold }
    }
}

impl<S> ChooseAction<S> for MCTS
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> <S as AlternateGameState>::Action {
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
        // legal_actions[i] と root.child_nodes[i] が対応している
        assert_eq!(legal_actions.len(), root.child_nodes.len());
        let (action, _) = legal_actions
            .into_iter()
            .zip(root.child_nodes)
            .max_by_key(|(_, node)| node.attempt)
            .unwrap();
        action
    }
}

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
            const EXPAND_THRESHOLD: u32 = 10;
            let mut state = self.state.clone();
            let value = playout(&mut state);
            self.win += value;
            self.attempt += 1;
            if self.attempt == EXPAND_THRESHOLD {
                self.expand(&self.state.legal_actions());
            }
            value
        } else {
            fn ucb1<T>(child: &Node<T>, t: u32) -> f64
            where
                T: AlternateGameState,
            {
                const C: f64 = 1.0;
                assert_ne!(child.attempt, 0);
                let attempt = f64::from(child.attempt);
                // child.win / attempt は子視点の勝率
                // self 視点の勝率にするために 1.0 から引く
                (1.0 - child.win / attempt) + C * f64::sqrt(2.0 * f64::from(t).ln() / attempt)
            }
            let index = 'next_child_node_index: {
                for (i, node) in self.child_nodes.iter().enumerate() {
                    // 一度も探索していないノードは最優先
                    if node.attempt == 0 {
                        break 'next_child_node_index i;
                    }
                }
                let t = self
                    .child_nodes
                    .iter()
                    .map(|node| node.attempt)
                    .sum::<u32>();
                (0..self.child_nodes.len())
                    .max_by(|&i, &j| {
                        let left = ucb1(&self.child_nodes[i], t);
                        let right = ucb1(&self.child_nodes[j], t);
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
