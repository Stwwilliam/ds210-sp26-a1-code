use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // println!("{min} to {max}");  # To see number range per iteration
        let guess = (min + max)/2;
        let result = player.ask_to_compare(guess);

        if result == 0 {
            return guess;
        } else if result == -1 {
            Self::guess_the_number(player, min, guess-1)
        } else {
            Self::guess_the_number(player, guess+1, max)
        }
    }
}
