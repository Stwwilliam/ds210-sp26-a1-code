use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;
use rand::random_range;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let guess = random_range(min..max);

        if player.ask_to_compare(guess) == 0 {
            return guess;
        } else if player.ask_to_compare(guess) == -1 {
            Self::guess_the_number(player, min, guess);
        } else {
            Self::guess_the_number(player, guess, max);
        }

        return 0;
    }
}
