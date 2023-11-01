mod state;

use ::thunder_book_game_search::{
    game::alternate::{AlternateGameState, WinningStatus},
    search::alternate::{mcts::MCTS, random::Random, ChooseAction},
};

use crate::state::ConnectFourState;

// src/bin/alternate/main.rs からコピー
fn first_player_winning_status<A, B>(
    mut state: ConnectFourState,
    first: &A,
    second: &B,
) -> WinningStatus
where
    A: ChooseAction<ConnectFourState>,
    B: ChooseAction<ConnectFourState>,
{
    for turn in 0.. {
        if let Some(status) = state.winning_status() {
            return match status {
                WinningStatus::Win => unreachable!(),
                WinningStatus::Lose => {
                    if turn % 2 == 0 {
                        status
                    } else {
                        WinningStatus::Win
                    }
                }
                WinningStatus::Draw => status,
            };
        }
        let action = if turn % 2 == 0 {
            first.choose(&state)
        } else {
            second.choose(&state)
        };
        state.advance(action);
    }
    unimplemented!()
}

fn play<A, B>(alice: &A, bob: &B, games: u32, h: usize, w: usize)
where
    A: ChooseAction<ConnectFourState>,
    B: ChooseAction<ConnectFourState>,
{
    let (mut win_alice, mut win_bob) = (0, 0);
    for _ in 0..games {
        let state = ConnectFourState::new(h, w);
        match first_player_winning_status(state.clone(), alice, bob) {
            WinningStatus::Win => {
                win_alice += 1;
            }
            WinningStatus::Lose => {
                win_bob += 1;
            }
            WinningStatus::Draw => {}
        }
        match first_player_winning_status(state, bob, alice) {
            WinningStatus::Win => {
                win_bob += 1;
            }
            WinningStatus::Lose => {
                win_alice += 1;
            }
            WinningStatus::Draw => {}
        }
    }

    println!(
        "alice = {}, bob = {}, even = {}",
        f64::from(win_alice) / f64::from(games * 2),
        f64::from(win_bob) / f64::from(games * 2),
        f64::from(games * 2 - win_alice - win_bob) / f64::from(games * 2),
    );
}

fn main() {
    let (games, h, w) = (100, 6, 7);

    let random = Random {};
    let mcts = MCTS::new(50);

    println!("random vs. random");
    play(&random, &random, games, h, w);

    println!("random vs. mcts");
    play(&random, &mcts, games, h, w);
}
