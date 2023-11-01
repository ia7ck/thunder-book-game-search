mod state;

use ::std::time::Duration;

use ::rand::{rngs::SmallRng, SeedableRng};

use ::thunder_book_game_search::{
    game::alternate::{AlternateGameState, WinningStatus},
    search::alternate::{
        alpha_beta::AlphaBeta, iterative_deepening_alpha_beta::IterativeDeepeningAlphaBeta,
        mcts::MCTS, mini_max::MiniMax, primitive_montecarlo::PrimitiveMontecarlo, random::Random,
        ChooseAction,
    },
};

use crate::state::AlternateMazeState;

fn first_player_winning_status<A, B>(
    mut state: AlternateMazeState,
    first: &A,
    second: &B,
) -> WinningStatus
where
    A: ChooseAction<AlternateMazeState>,
    B: ChooseAction<AlternateMazeState>,
{
    for turn in 0.. {
        if let Some(status) = state.winning_status() {
            return match status {
                WinningStatus::Win => {
                    if turn % 2 == 0 {
                        status
                    } else {
                        WinningStatus::Lose
                    }
                }
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

fn play<A, B>(alice: &A, bob: &B, games: u32, h: usize, w: usize, end_turn: u32, seed: u64)
where
    A: ChooseAction<AlternateMazeState>,
    B: ChooseAction<AlternateMazeState>,
{
    let mut rng = SmallRng::seed_from_u64(seed);
    let (mut win_alice, mut win_bob) = (0, 0);
    for _ in 0..games {
        let state = AlternateMazeState::new(h, w, end_turn, &mut rng);
        match first_player_winning_status(state.clone(), alice, bob) {
            WinningStatus::Win => {
                win_alice += 1;
            }
            WinningStatus::Lose => {
                win_bob += 1;
            }
            WinningStatus::Draw => {}
        }
        // 手番による有利不利を均すために同じ盤面で先手・後手両方プレイする
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
    let (games, h, w, end_turn) = (100, 3, 3, 4);

    let random = Random {};
    let mini_max = MiniMax::new(end_turn);
    let alpha_beta = AlphaBeta::new(end_turn, Duration::from_millis(10));
    let short_iterative_deepening = IterativeDeepeningAlphaBeta::new(Duration::from_micros(10));
    let long_iterative_deepening = IterativeDeepeningAlphaBeta::new(Duration::from_micros(1000));
    let short_primitive_montecarlo = PrimitiveMontecarlo::new(Duration::from_micros(10));
    let long_primitive_montecarlo = PrimitiveMontecarlo::new(Duration::from_micros(1000));
    let short_mcts = MCTS::new(Duration::from_micros(10));
    let long_mcts = MCTS::new(Duration::from_micros(1000));

    println!("random vs. mini_max");
    play(&random, &mini_max, games, h, w, end_turn, 12345);

    println!("mini_max vs. alpha_beta");
    play(&mini_max, &alpha_beta, games, h, w, end_turn, 67);

    let (games, h, w, end_turn) = (10, 5, 5, 10);
    println!("[iterative deepening] short vs. long");
    play(
        &short_iterative_deepening,
        &long_iterative_deepening,
        games,
        h,
        w,
        end_turn,
        89,
    );

    let (games, h, w, end_turn) = (100, 3, 3, 4);
    println!("random vs. primitive montecarlo");
    play(
        &random,
        &short_primitive_montecarlo,
        games,
        h,
        w,
        end_turn,
        9,
    );

    println!("[primitive montecarlo] short vs. long");
    play(
        &short_primitive_montecarlo,
        &long_primitive_montecarlo,
        games,
        h,
        w,
        end_turn,
        876,
    );

    println!("primitive montecarlo vs. mcts");
    play(
        &long_primitive_montecarlo,
        &long_mcts,
        games,
        h,
        w,
        end_turn,
        54,
    );

    println!("[mcts] short vs. long");
    play(&short_mcts, &long_mcts, games, h, w, end_turn, 3);
}
