use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        let result =player.ask_to_compare(min+(max-min)/2);
        if result==0{
            if player.ask_if_equal(min+(max-min)/2) {
                return min+(max-min)/2;
            }
            return min;
        }
        else if result>0{
            Self::guess_the_number(player, min+(max-min)/2, max)
        }
        else{
            Self::guess_the_number(player, min, min+(max-min)/2)
        }
        }        
}
