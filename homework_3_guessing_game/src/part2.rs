use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
<<<<<<< HEAD
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
=======
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
<<<<<<< HEAD
>>>>>>> 9565db42d02bb4ea27ae00c1a4a8d625a800b2b4
=======
>>>>>>> 9531b82c64d23bd7fcc41c726bb297969aa3797f
>>>>>>> 7b9378b404b4fced43a6271579aeff551bc3a7bb
}
