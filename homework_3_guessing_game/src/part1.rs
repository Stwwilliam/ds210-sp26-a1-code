use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part1 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part1 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
<<<<<<< HEAD
        for i in min..max {
            if player.ask_if_equal(i) {
                return i;
            }
        }
        return 0;
=======
        // YOUR SOLUTION GOES HERE.
        for i in min..max {
            if player.ask_if_equal(i) {
            return i;
            }
        }
        return min;
<<<<<<< HEAD
>>>>>>> 9565db42d02bb4ea27ae00c1a4a8d625a800b2b4
=======
>>>>>>> 9531b82c64d23bd7fcc41c726bb297969aa3797f
>>>>>>> 7b9378b404b4fced43a6271579aeff551bc3a7bb
    }
}
