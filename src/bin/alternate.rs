use std::{
    fmt,
    time::{Duration, Instant},
};

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
            if y + 1 < self.point.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

struct TimeKeeper {
    instant: Instant,
    threshold: Duration,
}

impl TimeKeeper {
    fn new(threshold: Duration) -> Self {
        Self {
            instant: Instant::now(),
            threshold,
        }
    }

    fn time_over(&self) -> bool {
        self.instant.elapsed() >= self.threshold
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

fn alpha_beta_action(
    state: &AlternateMazeState,
    depth: u32,
    time_keeper: &TimeKeeper,
) -> Option<Action> {
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
        t: &TimeKeeper,
    ) -> Option<SearchResult> {
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
    let alpha = SearchResult::Score(i16::MIN + 1); // .neg()をしてもオーバーフローしないように+1
    let beta = SearchResult::Score(i16::MAX);
    match search(state, alpha, beta, depth, time_keeper)? {
        SearchResult::Score(_) => unimplemented!("stateから遷移できる状態がない"),
        SearchResult::ScoreAndAction(_, action) => Some(action),
    }
}

fn iterative_deepening_action(state: &AlternateMazeState, threshold: Duration) -> Action {
    let time_keeper = TimeKeeper::new(threshold);
    let mut best_action = None;
    for depth in 1.. {
        if let Some(action) = alpha_beta_action(state, depth, &time_keeper) {
            best_action = Some(action);
        } else {
            return best_action.unwrap_or_else(|| panic!("深さ1の探索でも時間切れ"));
        }
    }
    unreachable!()
}

// montecarlo
fn playout(state: &mut AlternateMazeState) -> f64 {
    match state.winning_status() {
        Some(status) => match status {
            WinningStatus::Win => 1.0,
            WinningStatus::Draw => 0.5,
            WinningStatus::Lose => 0.0,
        },
        None => {
            state.advance(random_action(state));
            1.0 - playout(state)
        }
    }
}

fn primitive_montecarlo_action(state: &AlternateMazeState, playout_number: usize) -> Action {
    let legal_actions = state.legal_actions();
    let mut values = vec![0.0; legal_actions.len()];
    let mut counts = vec![0; legal_actions.len()];
    for i in 0..playout_number {
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

struct Node {
    state: AlternateMazeState,
    attempt: u32,
    win: f64,
    child_nodes: Vec<Node>,
}

impl Node {
    fn new(state: AlternateMazeState) -> Self {
        Self {
            state,
            attempt: 0,
            win: 0.0,
            child_nodes: Vec::new(),
        }
    }

    fn expand(&mut self, legal_actions: &Vec<Action>) {
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
            fn ucb1(child: &Node, t: u32) -> f64 {
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

fn mcts_action(state: &AlternateMazeState, playout_number: usize) -> Action {
    let mut root = Node::new(state.clone());
    let legal_actions = state.legal_actions();
    root.expand(&legal_actions);
    for _ in 0..playout_number {
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
    let mut win = [0, 0]; // alice, bob
    for _ in 0..games {
        let state = AlternateMazeState::new(h, w, end_turn, &mut rng);
        // alice先手、後手を両方プレイ
        for i in [0, 1] {
            let mut state = state.clone();
            loop {
                let current = if state.turn % 2 == 0 { i } else { i ^ 1 };
                if let Some(status) = state.winning_status() {
                    match status {
                        WinningStatus::Win => {
                            win[current] += 1;
                        }
                        WinningStatus::Lose => {
                            win[current ^ 1] += 1;
                        }
                        _ => {}
                    }
                    break;
                }
                let action = players[current].choose(&state);
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
    #[derive(Clone)]
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
            alpha_beta_action(
                state,
                self.depth,
                &TimeKeeper::new(Duration::from_millis(10)),
            )
            .unwrap_or_else(|| panic!("thresholdを調節する"))
        }
    }
    struct IterativeDeepening {
        threshold: Duration,
    }
    impl ChooseAction for IterativeDeepening {
        fn choose(&self, state: &AlternateMazeState) -> Action {
            iterative_deepening_action(state, self.threshold)
        }
    }
    #[derive(Clone)]
    struct PrimitiveMontecarlo {
        playout_number: usize,
    }
    impl ChooseAction for PrimitiveMontecarlo {
        fn choose(&self, state: &AlternateMazeState) -> Action {
            primitive_montecarlo_action(state, self.playout_number)
        }
    }
    #[allow(clippy::upper_case_acronyms)]
    #[derive(Clone)]
    struct MCTS {
        playout_number: usize,
    }
    impl ChooseAction for MCTS {
        fn choose(&self, state: &AlternateMazeState) -> Action {
            mcts_action(state, self.playout_number)
        }
    }

    let (h, w, end_turn) = (3, 3, 4);

    let random = Box::new(Random {});
    let mini_max = Box::new(MiniMax { depth: end_turn });
    let alpha_beta = Box::new(AlphaBeta { depth: end_turn });
    let short_iterative_deepening = Box::new(IterativeDeepening {
        threshold: Duration::from_millis(1),
    });
    let long_iterative_deepening = Box::new(IterativeDeepening {
        threshold: Duration::from_millis(100),
    });
    let small_primitive_montecarlo = Box::new(PrimitiveMontecarlo { playout_number: 30 });
    let large_primitive_montecarlo = Box::new(PrimitiveMontecarlo {
        playout_number: 3000,
    });
    let small_mcts = Box::new(MCTS { playout_number: 30 });
    let large_mcts = Box::new(MCTS {
        playout_number: 3000,
    });

    println!("random vs. mini_max");
    play(random.clone(), mini_max.clone(), 100, h, w, end_turn, 12345);

    println!("mini_max vs. alpha_beta");
    play(mini_max, alpha_beta, 100, h, w, end_turn, 67);

    let (h, w, end_turn) = (5, 5, 10);
    println!("[iterative deepening] short vs. long");
    play(
        short_iterative_deepening,
        long_iterative_deepening,
        10,
        h,
        w,
        end_turn,
        89,
    );

    let (h, w, end_turn) = (3, 3, 4);
    println!("random vs. primitive montecarlo");
    play(
        random,
        large_primitive_montecarlo.clone(),
        100,
        h,
        w,
        end_turn,
        9,
    );

    println!("[primitive montecarlo] small vs. large");
    play(
        small_primitive_montecarlo.clone(),
        large_primitive_montecarlo.clone(),
        100,
        h,
        w,
        end_turn,
        876,
    );

    println!("primitive montecarlo vs. mcts");
    play(
        large_primitive_montecarlo,
        large_mcts.clone(),
        100,
        h,
        w,
        end_turn,
        54,
    );

    println!("[mcts] small vs. large");
    play(
        small_mcts.clone(),
        large_mcts.clone(),
        100,
        h,
        w,
        end_turn,
        3,
    );
}
