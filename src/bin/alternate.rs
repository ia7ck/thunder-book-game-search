use std::fmt;

use rand::{rngs::SmallRng, seq::IteratorRandom, thread_rng, Rng, SeedableRng};

#[derive(Clone)]
struct Character {
    y: usize,
    x: usize,
    game_score: i16,
}

impl Character {
    fn new(y: usize, x: usize) -> Self {
        Self {
            y,
            x,
            game_score: 0,
        }
    }
}

#[derive(Clone, Copy)]
enum Action {
    Right,
    Left,
    Down,
    Up,
}

impl Action {
    fn dydx(&self) -> (isize, isize) {
        match self {
            Action::Right => (0, 1),
            Action::Left => (0, -1),
            Action::Down => (1, 0),
            Action::Up => (-1, 0),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Action::Right => '>',
            Action::Left => '<',
            Action::Down => 'v',
            Action::Up => '^',
        };
        write!(f, "{}", c)
    }
}

enum WinningStatus {
    Win,
    Draw,
    Lose,
}

#[derive(Clone)]
struct AlternateMazeState {
    point: Vec<Vec<u8>>,
    turn: u32,
    end_turn: u32,
    characters: [Character; 2],
}

impl AlternateMazeState {
    fn new(h: usize, w: usize, end_turn: u32, rng: &mut impl Rng) -> Self {
        let characters = [
            Character::new(h / 2, w / 2 - 1),
            Character::new(h / 2, w / 2 + 1),
        ];
        let mut point = vec![vec![0; w]; h];
        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            for x in 0..w {
                if characters.iter().any(|c| c.y == y && c.x == x) {
                    continue;
                }
                point[y][x] = rng.gen_range(0..10);
            }
        }
        Self {
            point,
            turn: 0,
            end_turn,
            characters,
        }
    }

    fn legal_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        for action in [Action::Right, Action::Left, Action::Down, Action::Up] {
            let (dy, dx) = action.dydx();
            match (
                self.characters[0].y.checked_add_signed(dy),
                self.characters[0].x.checked_add_signed(dx),
            ) {
                (Some(ty), Some(tx)) if ty < self.point.len() && tx < self.point[ty].len() => {
                    actions.push(action);
                }
                _ => {}
            }
        }
        actions
    }

    fn advance(&mut self, action: Action) {
        let (dy, dx) = action.dydx();
        let character = &mut self.characters[0];
        character.y = character.y.checked_add_signed(dy).unwrap();
        character.x = character.x.checked_add_signed(dx).unwrap();
        character.game_score += i16::from(self.point[character.y][character.x]);
        self.point[character.y][character.x] = 0;
        self.turn += 1;
        self.characters.swap(0, 1); // characters[0]が次の手番のキャラクターになるように
    }

    fn done(&self) -> bool {
        self.turn == self.end_turn
    }

    fn winning_status(&self) -> Option<WinningStatus> {
        use std::cmp::Ordering::*;
        if self.done() {
            match self.characters[0]
                .game_score
                .cmp(&self.characters[1].game_score)
            {
                Less => Some(WinningStatus::Lose),
                Equal => Some(WinningStatus::Draw),
                Greater => Some(WinningStatus::Win),
            }
        } else {
            None
        }
    }

    fn score(&self) -> i16 {
        self.characters[0].game_score - self.characters[1].game_score
    }
}

impl fmt::Debug for AlternateMazeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "turn:  {}", self.turn)?;
        let characters = if self.turn % 2 == 0 {
            [&self.characters[0], &self.characters[1]]
        } else {
            [&self.characters[1], &self.characters[0]]
        };
        writeln!(
            f,
            "score: {} vs. {}",
            characters[0].game_score, characters[1].game_score
        )?;
        let a = (characters[0].y, characters[0].x);
        let b = (characters[1].y, characters[1].x);
        for y in 0..self.point.len() {
            for x in 0..self.point[y].len() {
                let c = if (y, x) == a && (y, x) == b {
                    '@' // 重なり
                } else if (y, x) == a {
                    'A'
                } else if (y, x) == b {
                    'B'
                } else if self.point[y][x] == 0 {
                    '.'
                } else {
                    char::from(self.point[y][x] + b'0')
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn random_action(state: &AlternateMazeState) -> Action {
    let mut legal_actions = state.legal_actions();
    let mut rng = thread_rng();
    legal_actions.drain(..).choose(&mut rng).unwrap()
}

fn mini_max_action(state: &AlternateMazeState, depth: u32) -> Action {
    enum SearchResult {
        Score(i16),                  // leaf node
        ScoreAndAction(i16, Action), // internal
    }
    fn search(s: &AlternateMazeState, d: u32) -> SearchResult {
        if s.done() || d == 0 {
            return SearchResult::Score(s.score());
        }
        s.legal_actions()
            .into_iter()
            .fold(SearchResult::Score(s.score()), |mut best, action| {
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
            })
    }
    match search(state, depth) {
        SearchResult::Score(_) => unimplemented!("stateから遷移できる状態がない"),
        SearchResult::ScoreAndAction(_, action) => action,
    }
}

fn alpha_beta_action(state: &AlternateMazeState, depth: u32) -> Action {
    #[derive(Clone, Copy)]
    enum SearchResult {
        Score(i16),
        ScoreAndAction(i16, Action),
    }
    impl SearchResult {
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
    fn search(
        s: &AlternateMazeState,
        alpha: SearchResult,
        beta: SearchResult,
        d: u32,
    ) -> SearchResult {
        if s.done() || d == 0 {
            return SearchResult::Score(s.score());
        }
        let legal_actions = s.legal_actions();
        if legal_actions.is_empty() {
            return SearchResult::Score(s.score());
        }
        let mut alpha = alpha;
        for action in legal_actions {
            let mut next_s = s.clone();
            next_s.advance(action);
            let score = search(&next_s, beta.neg(), alpha.neg(), d - 1)
                .neg()
                .score();
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
                return alpha;
            }
        }
        alpha
    }
    let alpha = SearchResult::Score(i16::MIN + 1); // .neg()をしてもオーバーフローしないように+1
    let beta = SearchResult::Score(i16::MAX);
    match search(state, alpha, beta, depth) {
        SearchResult::Score(_) => unimplemented!("stateから遷移できる状態がない"),
        SearchResult::ScoreAndAction(_, action) => action,
    }
}

trait ChooseAction {
    fn choose(&self, state: &AlternateMazeState) -> Action;
}

fn play(
    alice: Box<dyn ChooseAction>,
    bob: Box<dyn ChooseAction>,
    games: u32,
    h: usize,
    w: usize,
    end_turn: u32,
    seed: u64,
) {
    let players = [alice, bob];
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut win = [0, 0];
    for _ in 0..games {
        let state = AlternateMazeState::new(h, w, end_turn, &mut rng);
        // alice先手、後手を両方プレイ
        for i in [0, 1] {
            let mut state = state.clone();
            loop {
                if let Some(status) = state.winning_status() {
                    match status {
                        WinningStatus::Win => {
                            win[i] += 1;
                        }
                        WinningStatus::Lose => {
                            win[i ^ 1] += 1;
                        }
                        _ => {}
                    }
                    break;
                }
                let action = if state.turn % 2 == 0 {
                    players[i].choose(&state)
                } else {
                    players[i ^ 1].choose(&state)
                };
                state.advance(action);
            }
        }
    }

    println!(
        "alice = {}, bob = {}, even = {}",
        f64::from(win[0]) / f64::from(games * 2),
        f64::from(win[1]) / f64::from(games * 2),
        f64::from(games * 2 - win[0] - win[1]) / f64::from(games * 2),
    );
}

fn main() {
    let (h, w, end_turn) = (3, 3, 4);
    struct Random {}
    impl ChooseAction for Random {
        fn choose(&self, state: &AlternateMazeState) -> Action {
            random_action(state)
        }
    }
    #[derive(Clone)]
    struct MiniMax {
        depth: u32,
    }
    impl ChooseAction for MiniMax {
        fn choose(&self, state: &AlternateMazeState) -> Action {
            mini_max_action(state, self.depth)
        }
    }
    struct AlphaBeta {
        depth: u32,
    }
    impl ChooseAction for AlphaBeta {
        fn choose(&self, state: &AlternateMazeState) -> Action {
            alpha_beta_action(state, self.depth)
        }
    }
    let random = Box::new(Random {});
    let mini_max = Box::new(MiniMax { depth: end_turn });
    let alpha_beta = Box::new(AlphaBeta { depth: end_turn });

    println!("random vs. mini_max");
    play(random, mini_max.clone(), 100, h, w, end_turn, 12345);

    println!("mini_max vs. alpha_beta");
    play(mini_max, alpha_beta, 100, h, w, end_turn, 67);
}
