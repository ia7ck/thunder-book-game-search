use std::{
    cmp,
    collections::BinaryHeap,
    fmt::{self, Formatter},
    time::{Duration, Instant},
};

use rand::{rngs::SmallRng, seq::IteratorRandom, thread_rng, Rng, SeedableRng};

#[derive(Clone, PartialEq, Eq)]
struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, PartialEq, Eq)]
struct MazeState {
    character: Coord,
    point: Vec<Vec<u8>>,
    turn: u32,
    end_turn: u32,
    game_score: u32,
    evaluated_score: u32,
}

impl MazeState {
    fn new(h: usize, w: usize, end_turn: u32, rng: &mut impl Rng) -> Self {
        let character = Coord::new(rng.gen_range(0..h), rng.gen_range(0..w));
        let mut point = vec![vec![0; w]; h];
        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            for x in 0..w {
                if y == character.y && x == character.x {
                    point[y][x] = 0;
                } else {
                    point[y][x] = rng.gen_range(0..10);
                }
            }
        }
        Self {
            character,
            point,
            turn: 0,
            end_turn,
            game_score: 0,
            evaluated_score: 0,
        }
    }

    fn legal_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        for action in [Action::Right, Action::Left, Action::Down, Action::Up] {
            let (dy, dx) = action.dydx();
            match (
                self.character.y.checked_add_signed(dy),
                self.character.x.checked_add_signed(dx),
            ) {
                (Some(ty), Some(tx)) if ty < self.point.len() && tx < self.point[ty].len() => {
                    actions.push(action);
                }
                _ => {
                    // 盤面の外に出てしまう
                }
            }
        }
        actions
    }

    fn advance(&mut self, action: Action) {
        let (dy, dx) = action.dydx();
        let character = &mut self.character;
        character.y = character.y.checked_add_signed(dy).unwrap();
        character.x = character.x.checked_add_signed(dx).unwrap();
        self.game_score += u32::from(self.point[character.y][character.x]);
        self.point[character.y][character.x] = 0;
        self.turn += 1;
    }

    fn done(&self) -> bool {
        self.turn == self.end_turn
    }

    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }
}

impl fmt::Debug for MazeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "turn:  {}", self.turn)?;
        writeln!(f, "score: {}", self.game_score)?;
        for y in 0..self.point.len() {
            for x in 0..self.point[y].len() {
                let c = if y == self.character.y && x == self.character.x {
                    '@'
                } else if self.point[y][x] == 0 {
                    '.'
                } else {
                    // '1', '2', ..., '9'
                    char::from(self.point[y][x] + b'0')
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl cmp::Ord for MazeState {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

impl cmp::PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
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

fn random_action(state: &MazeState) -> Action {
    let mut legal_actions = state.legal_actions();
    let mut rng = thread_rng();
    legal_actions.drain(..).choose(&mut rng).unwrap()
}

fn greedy_action(state: &MazeState) -> Action {
    let legal_actions = state.legal_actions();
    let mut best_score = 0;
    let mut best_action = None;
    for action in legal_actions {
        let mut next_state = state.clone();
        next_state.advance(action);
        next_state.evaluate_score();
        if best_score <= next_state.evaluated_score {
            best_score = next_state.evaluated_score;
            best_action = Some(action);
        }
    }
    best_action.unwrap()
}

fn beam_search_action(initial_state: &MazeState, beam_width: usize, threshold: Duration) -> Action {
    let time_keeper = TimeKeeper::new(threshold);
    let mut best_action = None;
    let mut heap = BinaryHeap::new();
    heap.push((initial_state.clone(), best_action));
    loop {
        let mut new_heap = BinaryHeap::new();
        for _ in 0..beam_width {
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

fn chokudai_search_action(
    initial_state: &MazeState,
    beam_width: usize,
    beam_depth: usize,
    threshold: Duration,
) -> Action {
    let time_keeper = TimeKeeper::new(threshold);
    let mut heaps = vec![BinaryHeap::new(); beam_depth + 1];
    heaps[0].push((initial_state.clone(), None));
    'outer: loop {
        for t in 0..beam_depth {
            for _ in 0..beam_width {
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
    for t in (0..=beam_depth).rev() {
        if let Some((_, action)) = heaps[t].peek() {
            return action.unwrap();
        }
    }
    unimplemented!("no action")
}

fn average_score(
    next_action: impl Fn(&MazeState) -> Action,
    games: u32,
    h: usize,
    w: usize,
    end_turn: u32,
    seed: u64,
) -> f64 {
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut total = 0;
    for _ in 0..games {
        let mut state = MazeState::new(h, w, end_turn, &mut rng);
        while !state.done() {
            let action = next_action(&state);
            state.advance(action);
        }
        total += state.game_score;
    }
    f64::from(total) / f64::from(games)
}

fn main() {
    let (h, w, end_turn) = (30, 30, 100);
    let play = |label: &str, next_action: Box<dyn Fn(&MazeState) -> Action>| {
        let average = average_score(next_action, 20, h, w, end_turn as u32, 9876543210);
        println!("{label}: {average}");
    };

    play("random", Box::new(random_action));
    play("greedy", Box::new(greedy_action));
    play(
        "beam",
        Box::new(|s| beam_search_action(s, 5, Duration::from_millis(10))),
    );
    play(
        "chokudai",
        Box::new(|s| chokudai_search_action(s, 1, end_turn, Duration::from_millis(10))),
    );
}
