// オート数字集め迷路

use ::std::{
    cmp::Reverse,
    fmt::{self, Formatter},
};

use ::rand::{thread_rng, Rng};

use ::thunder_book_game_search::game::heuristic::HeuristicGameState;

#[derive(Clone)]
struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

#[derive(Clone)]
pub struct AutoMoveMazeState {
    characters: Vec<Coord>,
    point: Vec<Vec<u8>>,
    turn: u32,
    end_turn: u32,
    game_score: u32,
}

impl AutoMoveMazeState {
    pub fn new(
        h: usize,
        w: usize,
        end_turn: u32,
        character_num: usize,
        rng: &mut impl Rng,
    ) -> Self {
        let mut point = vec![vec![0; w]; h];
        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            for x in 0..w {
                point[y][x] = rng.gen_range(0..10);
            }
        }

        let mut state = Self {
            characters: vec![Coord::new(0, 0); character_num],
            point,
            turn: 0,
            end_turn,
            game_score: 0,
        };

        state.initialize();
        state
    }

    fn done(&self) -> bool {
        self.turn == self.end_turn
    }

    fn advance(&mut self) {
        for c in &mut self.characters {
            let mut next = Vec::new();
            // 右
            if c.x + 1 < self.point[c.y].len() {
                next.push(((c.y, c.x + 1), self.point[c.y][c.x + 1]));
            }
            // 左
            if c.x > 0 {
                next.push(((c.y, c.x - 1), self.point[c.y][c.x - 1]));
            }
            // 下
            if c.y + 1 < self.point.len() {
                next.push(((c.y + 1, c.x), self.point[c.y + 1][c.x]));
            }
            // 上
            if c.y > 0 {
                next.push(((c.y - 1, c.x), self.point[c.y - 1][c.x]));
            }
            assert!(!next.is_empty());
            next.sort_by_key(|(_, p)| Reverse(*p)); // stable sort
            let ((y, x), point) = next[0];
            c.y = y;
            c.x = x;
            self.game_score += u32::from(point);
            self.point[y][x] = 0;
        }
        self.turn += 1;
    }
}

impl HeuristicGameState for AutoMoveMazeState {
    fn initialize(&mut self) {
        let mut rng = thread_rng();
        for c in &mut self.characters {
            c.y = rng.gen_range(0..self.point.len());
            c.x = rng.gen_range(0..self.point[c.y].len());
        }
    }

    fn start(&self) -> u32 {
        let mut state = self.clone();
        // キャラクターの初期位置にあるポイントは無効
        for c in &state.characters {
            state.point[c.y][c.x] = 0;
        }
        while !state.done() {
            state.advance();
        }
        state.game_score
    }

    fn transition(&mut self) {
        let mut rng = thread_rng();
        let i = rng.gen_range(0..self.characters.len());
        let new_y = rng.gen_range(0..self.point.len());
        let new_x = rng.gen_range(0..self.point[new_y].len());
        self.characters[i].y = new_y;
        self.characters[i].x = new_x;
    }
}

impl fmt::Debug for AutoMoveMazeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "turn:  {}", self.turn)?;
        writeln!(f, "score: {}", self.game_score)?;
        for y in 0..self.point.len() {
            for x in 0..self.point[y].len() {
                let c = if self.characters.iter().any(|c| c.y == y && c.x == x) {
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
