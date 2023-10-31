mod state;

use ::std::time::Duration;

use ::rand::{rngs::SmallRng, SeedableRng};

use ::thunder_book_game_search::{
    game::one_player::OnePlayerGameState,
    search::one_player::{
        beam::Beam, chokudai::Chokudai, greedy::Greedy, random::Random, ChooseAction,
    },
};

use crate::state::MazeState;

fn average_score<T>(next_action: T, games: u32, h: usize, w: usize, end_turn: u32, seed: u64) -> f64
where
    T: ChooseAction<MazeState>,
{
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut total = 0;
    for _ in 0..games {
        let mut state = MazeState::new(h, w, end_turn, &mut rng);
        while !state.done() {
            let action = next_action.choose(&state);
            state.advance(action);
        }
        total += state.evaluate_score();
    }
    f64::from(total) / f64::from(games)
}

fn main() {
    let (games, h, w, end_turn, seed) = (20, 30, 30, 100, 9876543210);

    println!(
        "random: {}",
        average_score(Random {}, games, h, w, end_turn, seed)
    );
    println!(
        "greedy: {}",
        average_score(Greedy {}, games, h, w, end_turn, seed)
    );
    println!(
        "beam: {}",
        average_score(
            Beam::new(5, Duration::from_millis(10)),
            games,
            h,
            w,
            end_turn,
            seed
        )
    );
    println!(
        "chokudai: {}",
        average_score(
            Chokudai::new(1, end_turn as usize, Duration::from_millis(10)),
            games,
            h,
            w,
            end_turn,
            seed
        )
    );
}
