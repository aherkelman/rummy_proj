use gencard::Card;
use rummy::tools::PlayingCard;
use rummy::tools::RummyUtils;
use std::io;
use rummy::tools::Hand;

// pub trait Brain{
//     fn new()->Self;
//     // fn play_turn(hand:&mut Hand, topCard: &Card<PlayingCard>, deckCard: &Card<PlayingCard>, turnNum: &usize) -> (Card<PlayingCard>, bool, bool);
// }

pub struct AiP{
    hand:Hand
}
impl AiP{
    pub fn new(hand:Hand)->AiP{
        AiP{hand:hand}
    }
    // Call function to live out current turn      
    pub fn play_turn(&mut self, topCard: &Card<PlayingCard>, deckCard: &Card<PlayingCard>, turnNum: &usize)-> (Card<PlayingCard>, bool, bool)
    {
         println!("Top discard: {:?}", topCard);
         println!("Top Draw: {:?}",deckCard);

        // If top card of discard is in something then take it, otherwise draw a card.
        let mut drew_card = false;
         if RummyUtils::in_something(&self.hand.current_hand, topCard)
         {
            self.hand.card_to_hand(*topCard);
         }
         else 
         {
            self.hand.card_to_hand(*deckCard);
            drew_card = true;
         }

         // Mark how many matches each card has
         RummyUtils::mark_safe(&mut self.hand.current_hand);

         // Give each card a rating, if safe make rating 0, higher the rating the more you
         // want to get rid of the card
         let mut ratings:Vec<usize>=Vec::new();
         for i in self.hand.current_hand.iter()
         {
            if i.get_specific().numMatchs >= 2
            {
                ratings.push(0);
            }
            else
            {
                let mut rating_val = i.get_specific().val + turnNum*(i.get_specific().val/10);
                if i.get_specific().numMatchs != 0
                {
                    rating_val = rating_val - i.get_specific().numMatchs*(i.get_specific().val/3);
                }

                ratings.push(rating_val);
            }
         }
         
         // Find largest valued card and discard it (return and remove from hand)

         // should be a function but can't get it to work
         // start of function
        let mut max_index = 0;
        let mut max_val = 0;
        for i in 0..ratings.len()
        {
            if ratings[i] >= max_val
            {
                max_index = i;
                max_val = ratings[i];
            }
        }
        // end of function

         //print hand and ratings for debug
         println!("ratings: {:?}",ratings);
                          let mut k = 0;
        for l in self.hand.current_hand.iter()
        {
            println!("opponent: k{} {:?}",k,l);
            k +=1;
        }
        // Save card to return 
        let discarded_card = self.hand.current_hand[max_index];

        // remove card from vectors
        ratings.remove(max_index);
        self.hand.card_from_hand(max_index);

        // if sum of ratings is 0 the player has rummy
        // start of sum function
        let mut sum = 0;
        for i in ratings.iter()
        {
            sum = sum + i;
        }
        // end of sum function

         let mut rummy = false;
         if sum == 0
         {
             rummy = true;
         }


         println!("");
         println!("disacarded card: {:?}",discarded_card);
         // return results
         (discarded_card,drew_card, rummy)


    }
}

pub struct UserP{
    hand:Hand
}

impl UserP{
    pub fn new(hand:Hand)->UserP{
        UserP{hand:hand}
    }
    pub fn drawing_card(&mut self, topCard: &Card<PlayingCard>, deckCard: &Card<PlayingCard>)-> usize
    {
         RummyUtils::value_sort(&mut self.hand.current_hand);
         println!("Top discard: {:?}", topCard);
         println!("Top Draw: {:?}",deckCard);
         println!("Current Hand: ");
         for i in self.hand.current_hand.iter()
         {
            println!("{:?}",i);
         }
         println!("Enter 1 to take top discard, and 2 to draw card.");
        let mut n = String::new();
        io::stdin()
           .read_line(&mut n)
           .expect("failed to read input.");
        let input = n.trim().parse::<usize>().expect("invalid input");
        let mut drew_card = 0;
        if input == 1
        {
            self.hand.card_to_hand(*topCard);
             drew_card = 1;
        }
        else 
        {
            self.hand.card_to_hand(*deckCard);
            drew_card = 2;
        } 
        return drew_card;
    }
    // Function to get player to play cards
    pub fn discarding_card(&mut self, turnNum: &usize)-> (Card<PlayingCard>, bool,bool)
    {
        let discardedSomething = true;
        let discard_card = self.hand.current_hand[0];
        let rummy = false;
        RummyUtils::value_sort(&mut self.hand.current_hand);
        println!("Current Hand, Enter a number to discard:");
        let mut j = 0;
        for i in self.hand.current_hand.iter()
        {
            println!("Card: {} {:?}",j,i);
            j +=1;
        }
        let mut n = String::new();
        io::stdin()
           .read_line(&mut n)
           .expect("failed to read input.");
        let card_select = n.trim().parse::<usize>().expect("invalid input");
        println!("{:?}", card_select);

        let discard_card = self.hand.current_hand[card_select];
        self.hand.card_from_hand(card_select);

        RummyUtils::mark_safe(&mut self.hand.current_hand);
        let mut rummy = true;
        for i in self.hand.current_hand.iter()
        {
            if i.get_specific().numMatchs < 2
            {
                rummy = false;
                break;
            }
        }
        (discard_card,rummy,discardedSomething)

    }

}

// pub struct Player<T:Brain>{
//     hand:Hand,
//     userdata:T
// }       

// impl<T:Brain> Player<T>{
//     pub fn new(hand:Hand)->Player<T>{
//         Player::<T>{hand:hand,userdata:T::new()}
//     }
//     // pub fn play_turn(&mut self, topCard: &Card<PlayingCard>, deckCard: &Card<PlayingCard>, turnNum: &usize)-> (Card<PlayingCard>, bool, bool){
//     //     T::play_turn(&mut self.hand,topCard,deckCard,turnNum)
//     // }
    

// }
