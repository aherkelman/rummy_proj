
use rummy::tools::PlayingCard;
use rummy::tools::Suit;
use rummy::player::Player;
use rummy::player::AiP;
use rummy::player::UserP;
use gencard::Deck;
use rummy::tools::Hand;

 use std::io;

pub fn play_game(){

	let mut regular_cards:Vec<PlayingCard> = Vec::new();

	// Fill in values 1 - 13 (ace through king) for each suit
    for x in 0..4 
    {
        for y in 1..14
        {
        	match x {
                0 => regular_cards.push(PlayingCard{val:y, suit:Suit::Heart, numMatchs:0}),
                1 => regular_cards.push(PlayingCard{val:y, suit:Suit::Spade, numMatchs:0}),
                2 => regular_cards.push(PlayingCard{val:y, suit:Suit::Diamond, numMatchs:0}),
                3 => regular_cards.push(PlayingCard{val:y, suit:Suit::Club, numMatchs:0}),
                _ => println!("Error!"),

            }
        }
    }

    let mut deck = Deck::new(regular_cards);
    //println!("deck={:?}",deck);

    deck.reset();
    

    let mut player1Hand=Hand::new();
    let mut player2Hand=Hand::new();

    for x in 0..7
    {
    	match(deck.draw_card()){
    		Some(x)=>{
    			//println!("picked up a {:?}",x);
    			player1Hand.card_to_hand(x);
    			player2Hand.card_to_hand(deck.draw_card().unwrap());
    		},
    		None=>{
    			panic!("deck empty");
    		}
    	}
    	
    }

    let mut player1:Player<AiP> = Player::new(player1Hand);
    let mut player2:Player<UserP> = Player::new(player2Hand);

    let mut j = 0;
    let draw = deck.draw_card().unwrap();
    deck.discard(draw);

    let mut num_turns = 0;
    let rummy = false;
    loop
    {
        let mut discard = deck.current_deck[0];
        let mut drew_card = true;
        let (discard,drew_card,rummy) = player1.play_turn(&deck.discard_deck[deck.discard_deck.len()-1],&deck.current_deck[deck.current_deck.len()-1],&num_turns);
        num_turns += 1;
        if drew_card
        {
        	//println!("drawing card...");
            deck.draw_card();
            deck.discard(discard);
        }
        else 
        {
        	//println!("taking discarded card...");
        	deck.discard(discard);
        }
        if rummy
        {
        	println!("you lose!");
            break;
        }
                         let mut input = String::new();
                 io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
        let (discard,drew_card,rummy) = player2.play_turn(&deck.discard_deck[deck.discard_deck.len()-1],&deck.current_deck[deck.current_deck.len()-1],&num_turns);
        if drew_card
        {
            deck.draw_card();
            deck.discard(discard);
        }
        else 
        {
        	deck.discard(discard);
        }
        if rummy
        {
        	println!("you win!");
            break;
        }
        println!("rummy: {}",rummy);
    }

}