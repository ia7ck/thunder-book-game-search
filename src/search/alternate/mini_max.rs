use crate::{game::alternate::AlternateGameState, search::alternate::ChooseAction};

pub struct MiniMax {
    depth: u32,
}

impl MiniMax {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }
}

impl<S> ChooseAction<S> for MiniMax
where
    S: AlternateGameState,
{
    fn choose(&self, state: &S) -> S::Action {
        // 気持ちとしては S::Action だが
        // 「can't use generic parameters from outer function」なので S とは別に T を用意する
        enum SearchResult<T>
        where
            T: AlternateGameState,
        {
            Score(i16),                     // leaf node
            ScoreAndAction(i16, T::Action), // internal
        }
        fn search<T>(s: &T, d: u32) -> SearchResult<T>
        where
            T: AlternateGameState,
        {
            if s.done() || d == 0 {
                return SearchResult::Score(s.score());
            }
            s.legal_actions().into_iter().fold(
                SearchResult::Score(s.score()),
                |mut best, action| {
                    let mut next_s = s.clone();
                    next_s.advance(action);
                    // 先手/後手によらずscore最大化で済むように-1倍する
                    let score = -match search(&next_s, d - 1) {
                        SearchResult::Score(score) => score,
                        SearchResult::ScoreAndAction(score, _) => score,
                    };
                    let score_and_action = SearchResult::ScoreAndAction(score, action);
                    best = match best {
                        SearchResult::Score(_) => score_and_action,
                        SearchResult::ScoreAndAction(b, _) if b < score => score_and_action,
                        _ => best,
                    };
                    best
                },
            )
        }
        match search(state, self.depth) {
            SearchResult::Score(_) => unimplemented!("stateから遷移できる状態がない"),
            SearchResult::ScoreAndAction(_, action) => action,
        }
    }
}
