// 数字集め迷路

use ::std::{
    cmp,
    fmt::{self, Formatter},
};

use ::rand::Rng;

use ::thunder_book_game_search::game::one_player::OnePlayerGameState;

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
pub enum Action {
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
pub struct MazeState {
    character: Coord,
    point: Vec<Vec<u8>>,
    turn: u32,
    end_turn: u32,
    game_score: u32,
}

impl MazeState {
    pub fn new(h: usize, w: usize, end_turn: u32, rng: &mut impl Rng) -> Self {
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
        }
    }
}

impl OnePlayerGameState for MazeState {
    type Action = Action;

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

    fn evaluate_score(&self) -> u32 {
        self.game_score
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
        self.evaluate_score().cmp(&other.evaluate_score())
    }
}

impl cmp::PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
