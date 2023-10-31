use ::std::time::Duration;

use crate::{game::alternate::AlternateGameState, search::alternate::ChooseAction, TimeKeeper};

pub struct AlphaBeta {
    depth: u32,
    threshold: Duration,
}

impl AlphaBeta {
    pub fn new(depth: u32, threshold: Duration) -> Self {
        Self { depth, threshold }
    }
}

impl<S> ChooseAction<S> for AlphaBeta
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> S::Action {
        alpha_beta_inner(state, self.depth, &TimeKeeper::new(self.threshold))
            .unwrap_or_else(|| panic!("1手にかけられる時間が短い。thresholdを調節する"))
    }
}

#[derive(Clone)]
enum SearchResult<T>
where
    T: AlternateGameState,
{
    Score(i16),
    ScoreAndAction(i16, T::Action),
}

impl<T> Copy for SearchResult<T> where T: AlternateGameState {}

impl<T> SearchResult<T>
where
    T: AlternateGameState,
{
    fn neg(self) -> Self {
        match self {
            SearchResult::Score(s) => SearchResult::Score(-s),
            SearchResult::ScoreAndAction(s, a) => SearchResult::ScoreAndAction(-s, a),
        }
    }
    fn score(&self) -> i16 {
        match self {
            SearchResult::Score(s) => *s,
            SearchResult::ScoreAndAction(s, _) => *s,
        }
    }
}

fn search<T>(
    s: &T,
    alpha: SearchResult<T>,
    beta: SearchResult<T>,
    d: u32,
    t: &TimeKeeper,
) -> Option<SearchResult<T>>
where
    T: AlternateGameState,
{
    if t.time_over() {
        return None;
    }
    if s.done() || d == 0 {
        return Some(SearchResult::Score(s.score()));
    }
    let legal_actions = s.legal_actions();
    if legal_actions.is_empty() {
        return Some(SearchResult::Score(s.score()));
    }
    let mut alpha = alpha;
    for action in legal_actions {
        let mut next_s = s.clone();
        next_s.advance(action);
        let score = search(&next_s, beta.neg(), alpha.neg(), d - 1, t)?
            .neg()
            .score();
        if t.time_over() {
            return None;
        }
        let score_and_action = SearchResult::ScoreAndAction(score, action);
        alpha = match alpha {
            SearchResult::Score(_) => score_and_action,
            SearchResult::ScoreAndAction(b, _) if b < score => score_and_action,
            _ => alpha,
        };
        // βカット
        // 親ノードではbeta.score()を最小化する
        // このノードのスコアはalpha.score()以上が確定している
        // これ以上探索してもbeta.score()を更新できないので枝刈り
        if alpha.score() >= beta.score() {
            return Some(alpha);
        }
    }
    Some(alpha)
}

pub(crate) fn alpha_beta_inner<T>(
    state: &T,
    depth: u32,
    time_keeper: &TimeKeeper,
) -> Option<T::Action>
where
    T: AlternateGameState,
{
    let alpha = SearchResult::Score(i16::MIN + 1); // .neg()をしてもオーバーフローしないように+1
    let beta = SearchResult::Score(i16::MAX);
    match search(state, alpha, beta, depth, time_keeper)? {
        SearchResult::Score(_) => unimplemented!("stateから遷移できる状態がない"),
        SearchResult::ScoreAndAction(_, action) => Some(action),
    }
}
