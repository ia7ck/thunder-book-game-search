mod state;

use ::rand::{rngs::SmallRng, SeedableRng};

use ::thunder_book_game_search::{
    game::heuristic::HeuristicGameState,
    search::heuristic::{random::Random, ChooseState},
};

use crate::state::AutoMoveMazeState;

fn average_score<T>(
    best_state: T,
    games: u32,
    h: usize,
    w: usize,
    end_turn: u32,
    character_num: usize,
    seed: u64,
) -> f64
where
    T: ChooseState<AutoMoveMazeState>,
{
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut total = 0;
    for _ in 0..games {
        let state = AutoMoveMazeState::new(h, w, end_turn, character_num, &mut rng);
        let score = best_state.choose(&state).start();
        total += score;
    }
    f64::from(total) / f64::from(games)
}

fn main() {
    let (games, h, w, end_turn, character_num, seed) = (1, 5, 5, 5, 3, 31415);

    let random = Random {};

    println!(
        "random: {}",
        average_score(random, games, h, w, end_turn, character_num, seed)
    );
}
