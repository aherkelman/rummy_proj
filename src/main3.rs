#![allow(dead_code)]
#![allow(non_snake_case)]
extern crate rand;
use std::io;



// Card Stuct needs the following:
//     Members:
//     Card Type - Specific Information related to a card, passed in on creation
//         doing this allows for different kinds of cards
/*
mod cards{
	use rand;
	use rand::Rng;
	use std::fmt::Debug;
	use PlayingCard;
    use Suit;
    use std::io;
}
*/

mod gencard;
mod rummy;
// Player Information





// Game Card Type
//    Preditor, Prey, Repro, or Enviorn
//    Can do like this example where some of the values have a cooresponding integer value
//    // An `enum` may either be `unit-like`,
//     Engineer,
//     Scientist,
//     // like tuple structs,
//     Height(i32),
//     Weight(i32),  






fn main() 
{
	rummy::game::play_game();
}
