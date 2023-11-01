use ::std::{fmt, mem};

use ::thunder_book_game_search::game::alternate::{AlternateGameState, WinningStatus};

#[derive(Clone, Copy)]
pub struct DropPiece {
    x: usize,
}

impl DropPiece {
    fn new(x: usize) -> Self {
        Self { x }
    }
}

#[derive(Clone)]
pub struct ConnectFourState {
    h: usize,
    w: usize,
    // my_board[y][x]: 下からy番目、左からx番目に自分の駒があるか
    // my_board[y][x], enemy_board[y][x] 両方trueは起こらないようにする
    my_board: Vec<Vec<bool>>,
    enemy_board: Vec<Vec<bool>>,
    winning_status_cache: Option<WinningStatus>,
}

impl ConnectFourState {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            my_board: vec![vec![false; w]; h],
            enemy_board: vec![vec![false; w]; h],
            winning_status_cache: None,
        }
    }
}

impl AlternateGameState for ConnectFourState {
    type Action = DropPiece;

    fn legal_actions(&self) -> Vec<Self::Action> {
        let mut actions = Vec::new();
        for x in 0..self.w {
            // 一番上が空いていればx列目に駒を落とせる
            if !self.my_board[self.h - 1][x] && !self.enemy_board[self.h - 1][x] {
                actions.push(DropPiece::new(x));
            }
        }
        actions
    }

    fn advance(&mut self, action: Self::Action) {
        assert!(!self.done());

        let piece_y = (0..self.h)
            .find(|&y| !self.my_board[y][action.x] && !self.enemy_board[y][action.x])
            .unwrap_or_else(|| panic!("{}列目が埋まっている", action.x));
        self.my_board[piece_y][action.x] = true;

        let left = || (0..action.x).rev();
        let right = || action.x..self.w;
        let up = || piece_y..self.h;
        let down = || (0..piece_y).rev();

        // 横
        let yoko = {
            let left = left().take_while(|&x| self.my_board[piece_y][x]).count();
            let right = right().take_while(|&x| self.my_board[piece_y][x]).count();
            left + right
        };
        // 左上から右下
        let naname = {
            let upper_left = up()
                .clone()
                .zip(left())
                .take_while(|&(y, x)| self.my_board[y][x])
                .count();
            let lower_right = down()
                .zip(right())
                .take_while(|&(y, x)| self.my_board[y][x])
                .count();
            upper_left + lower_right
        };
        // 右上から左下
        let menana = {
            let upper_right = up()
                .zip(right())
                .take_while(|&(y, x)| self.my_board[y][x])
                .count();
            let lower_left = down()
                .zip(left())
                .take_while(|&(y, x)| self.my_board[y][x])
                .count();
            upper_right + lower_left
        };
        // 縦
        let tate = 1 + down().take_while(|&y| self.my_board[y][action.x]).count();

        mem::swap(&mut self.my_board, &mut self.enemy_board);
        if yoko >= 4 || naname >= 4 || menana >= 4 || tate >= 4 {
            // 今回駒が揃ったので次に打つ側の負け
            self.winning_status_cache = Some(WinningStatus::Lose);
        } else if self.legal_actions().is_empty() {
            self.winning_status_cache = Some(WinningStatus::Draw);
        }
    }

    fn done(&self) -> bool {
        self.winning_status_cache.is_some()
    }

    fn score(&self) -> i16 {
        unimplemented!()
    }

    fn winning_status(&self) -> Option<WinningStatus> {
        self.winning_status_cache
    }
}

impl ConnectFourState {
    // デバッグ用
    fn first(&self) -> bool {
        let (mut my, mut enemy) = (0, 0);
        for y in 0..self.h {
            for x in 0..self.w {
                if self.my_board[y][x] {
                    my += 1;
                }
                if self.enemy_board[y][x] {
                    enemy += 1;
                }
            }
        }
        assert!(my <= enemy);
        my == enemy // my < enemy のときは後手番
    }
}

// 先手を x, 後手を o で表示する
impl fmt::Debug for ConnectFourState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first = self.first();
        for y in (0..self.h).rev() {
            for x in 0..self.w {
                let c = match (self.my_board[y][x], self.enemy_board[y][x]) {
                    (true, true) => unreachable!(),
                    (true, false) => {
                        if first {
                            'x'
                        } else {
                            'o'
                        }
                    }
                    (false, true) => {
                        if first {
                            'o'
                        } else {
                            'x'
                        }
                    }
                    (false, false) => '.',
                };
                write!(f, "{}", c)?;
            }
            if y > 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
