use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

impl Strategy for Part2 {
<<<<<<< HEAD
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        loop{
            let num = (min + max)/2;
            let mut x = player.ask_to_compare(num);
            if x == 0 {
                return num;
            }
            else if x == -1{
                max = num;
            }
            else if x == 1{
                min = num + 1;
            }
        }



            return 0 as u32;
=======
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // base case: only one number left
        if min >= max {
            return min;
        }

        let mid = (min + max) / 2;
        let result = player.ask_to_compare(mid);

        if result == 0 {
            return mid;
        }
        else if result == -1 {
            // number is smaller than mid
            return Part2::guess_the_number(player, min, mid);
        }
        else {
            // number is greater than mid
            return Part2::guess_the_number(player, mid + 1, max);
        }
>>>>>>> origin/student2-push
    }
}
