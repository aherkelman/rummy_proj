
use rummy::tools::PlayingCard;
use rummy::tools::Suit;
use rummy::player::AiP;
use rummy::player::UserP;
use gencard::Deck;
use rummy::tools::Hand;
use sprite::*;
use std::io;
use rummy::draw;
use gencard::Card;

pub struct Game 
{
    deck:Deck<PlayingCard>,
    player1:AiP,
    player2:UserP,
    num_turns:usize,
    rummy:bool,
    game_state:usize
}
impl Game 
{
    pub fn new(sprite_manager:draw::CardSpriteManager)->Game
    {
        let mut regular_cards:Vec<Card<PlayingCard>> = Vec::new();


        // Fill in values 1 - 13 (ace through king) for each suit
        for x in 0..4 
        {
            for y in 1..14
            {
                
                let sp=sprite_manager.new_card(x,y);
                regular_cards.push(Card::new(PlayingCard{val:y, suit:x, numMatchs:0,sprite:sp}))
            }
        }

        let mut deck = Deck::new(regular_cards);
        println!("deck={:?}",deck);

        deck.reset();
        

        let mut player1Hand=Hand::new();
        let mut player2Hand=Hand::new();

        for x in 0..7
        {
            match(deck.draw_card()){
                Some(x)=>{
                    println!("picked up a {:?}",x);
                    player1Hand.card_to_hand(x);
                    player2Hand.card_to_hand(deck.draw_card().unwrap());
                },
                None=>{
                    panic!("deck empty");
                }
            }
            
        }

        let mut player1:AiP = AiP::new(player1Hand);
        let mut player2:UserP = UserP::new(player2Hand);

        let mut j = 0;
        let draw = deck.draw_card().unwrap();
        deck.discard(draw);
        let num_turns=0;
        let rummy = false;
        let game_state = 0;
        Game{deck,player1,player2,num_turns,rummy,game_state}
    }
    pub fn play_game(&mut self, xpos:f64, ypos:f64)
    {
        // Game state 0 waits for player to pick which card to draw
        if self.game_state == 0
        {
            //let mut discard = self.deck.current_deck[0];
            let mut drew_card = true;

            // Get selected card
            // input xpos ypos, return if draw, discard, or neither selected
            //let card_drawn = self.player2.drawing_card(&self.deck.discard_deck[self.deck.discard_deck.len()-1],
            //                          &self.deck.current_deck[self.deck.current_deck.len()-1]);

            self.player2.drawing_card(&mut self.deck);
            self.game_state=1;
            /*
            if card_drawn != 0
            {
                self.game_state=1;
                if card_drawn == 2
                {
                    self.deck.draw_card();
                }
                // update sprite display
            }*/
            //else nothing
            
        }
        // Game state 0 waits for player to discard a card and then plays AI turn
        else if self.game_state == 1
        {
            // Get Discarded card
            // input xpos ypos, return card num or None
            let (discard,x,discardedCard) = self.player2.discarding_card(&self.num_turns);
            if discardedCard
            {
                self.deck.discard(discard);
                self.rummy=x;
                self.game_state=0;
                let (discard,x) = 
                        self.player1.play_turn(
                                    &mut self.deck,
                                &self.num_turns);
                self.rummy=x;
                self.num_turns += 1;
                
                self.deck.discard(discard);
            }


        }



    }

}