// 交互着手数字集め迷路

use ::std::fmt;

use ::rand::Rng;

use ::thunder_book_game_search::game::alternate::{AlternateGameState, WinningStatus};

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

#[derive(Clone)]
pub struct AlternateMazeState {
    point: Vec<Vec<u8>>,
    turn: u32,
    end_turn: u32,
    characters: [Character; 2],
}

impl AlternateMazeState {
    pub fn new(h: usize, w: usize, end_turn: u32, rng: &mut impl Rng) -> Self {
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
}

impl AlternateGameState for AlternateMazeState {
    type Action = Action;

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

    fn score(&self) -> i16 {
        self.characters[0].game_score - self.characters[1].game_score
    }

    fn score_rate(&self) -> f64 {
        let score_0 = self.characters[0].game_score;
        let score_1 = self.characters[1].game_score;
        if score_0 + score_1 == 0 {
            assert_eq!(score_0, 0);
            assert_eq!(score_1, 0);
            0.0 // 0.5 のほうがいい？
        } else {
            f64::from(score_0) / f64::from(score_0 + score_1)
        }
    }

    fn winning_status(&self) -> Option<WinningStatus> {
        use ::std::cmp::Ordering::*;
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
