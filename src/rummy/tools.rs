	  use gencard::Card;

// Standard Playing Card Type
//   Hearts, Spades, Diamonds, or Clubs
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}
#[derive(Clone,Copy,Debug)]
pub struct PlayingCard{
    pub val:usize,    // numerical value of card
    pub suit:Suit,   // suit of card
    pub numMatchs:usize, // number of matches of same val
}


pub struct Hand{
    pub current_hand:Vec<Card<PlayingCard>>
}


impl Hand{
    // Function to create a new caVec::new()rd based on passed on details passed in
    pub fn new()->Hand
    {
        Hand{current_hand:Vec::new()}
    }

    // Function to put card into hand
    pub fn card_to_hand(&mut self, card:Card<PlayingCard>)
    {
        //println!("AIPlayer::card_to_hand");
        self.current_hand.push(card);
    }

    // Function to remove a card from hand
    pub fn card_from_hand(&mut self, to_remove:usize) -> Card<PlayingCard>
    {
        //println!("AIPlayer::card_from_hand");
        self.current_hand.remove(to_remove)
    }  

}

        pub struct RummyUtils
	    {
            current_hand:Vec<Card<PlayingCard>>,
	    }
	    impl RummyUtils
	    {
	    	// returns index of max value
	    	pub fn vector_max(vector: &Vec<usize>) ->usize
	    	{
	    		//println!("RummyUtils::vector_max");

	    		let mut max_index = 0;
	    		let mut max_val = 0;
                for i in 0..vector.len()
	    		{
                    if vector[i] >= max_val
                    {
                    	max_index = i;
                    	max_val = vector[i];
                    }
	    		}
	    		max_index
	    	}

	    	// returns the sum of the inputted vector
	    	pub fn vector_sum(vector: &Vec<usize>) ->usize
	    	{
	    		let mut sum = 0;
	    		for i in vector.iter()
	    		{
	    			sum = sum + i;
	    		}
	    		sum
	    	}

	    	// Check if input card makes some kind of match with input hand
            pub fn in_something(hand: &Vec<Card<PlayingCard>>, card: &Card<PlayingCard>) -> bool 
            {
            	let cd = card.get_specific();
                 for i in hand.iter()
                 {
                 	let hd = i.get_specific();
                 	// if current card is in straight
                 	if (hd.suit == cd.suit) & ((cd.val == (hd.val-1)) | (cd.val == (hd.val+1)))
                 	{
                 		// return 1
                        return true;
                 	}
                 	// If card values are equal (for example two 7s)
                 	else if hd.val == cd.val
                 	{
                 		// return 1
                 		return true;
                 	}
                 }
                // if 1 isn't returned then return 0
                return false;
            }

            pub fn mark_safe(hand: &mut Vec<Card<PlayingCard>>)
            {
            	// Check how many matches each card has in terms of same card
                RummyUtils::value_sort(hand);
                let mut pv = hand[0].get_specific().val;
                let mut current_match = 0;

                for i in 1..hand.len()
	        	{
	        		let mut increase = false;
                    if pv == hand[i].get_specific().val
                    {
                        current_match+=1;
                        increase = true;
                    }
                	if !increase | (i == (hand.len()-1))
                    {
                    	let mut j = current_match+1;
                    	while j > 0
                    	{
                    		j-=1;
                    		let mut index = i-j-1;
                    	    if increase
                    	    {
                    		    index=index+1;
                    	    }
                    		hand[index].get_mut_specific().numMatchs = current_match;
                    	}
                    	current_match = 0;
                    	pv = hand[i].get_specific().val;
                    }
	        	}

                // Check how many matches each card has in terms of straights
                RummyUtils::suit_sort(hand);

                pv = hand[0].get_specific().val;
                let mut ps = hand[0].get_specific().suit;
                current_match = 0;
                for i in 1..hand.len()
                {
                	let mut increase = false;
                	if (ps == hand[i].get_specific().suit) & (pv == (hand[i].get_specific().val-1)) & (hand[i].get_specific().numMatchs < 3)
                	{
                        current_match += 1;
                        increase = true;
                	}
                	if !increase | (i == (hand.len()-1))
                	{
                		let mut j = current_match+1;
                    	while j > 0
                    	{
                    		j -=1 ;
                    		 if hand[i-j-1].get_specific().numMatchs < current_match
                    		 {
                    			let mut index = i-j-1;
                    			if increase
                    			{
                    				index=index+1;
                    			}
                    		    hand[index].get_mut_specific().numMatchs = current_match;
                    		 }
                    	}
                    	current_match = 0;
                    	pv = hand[i].get_specific().val;
                    	ps = hand[i].get_specific().suit;
                	}
                }
            }

            // insertion sort on passed in vector
            pub fn value_sort(hand: &mut Vec<Card<PlayingCard>>)
            {

            	for i in 1..hand.len()
	        	{
                    let mut j = i - 1;
                    let mut k = j as isize; 
                    let mut reff = hand[i];
                    while (k >= 0) & (hand[j].get_specific().val > reff.get_specific().val)
                    {
                    	let temp = hand[j];
                    	hand[j] = hand[j+1];
                    	hand[j+1] = temp;
                    	k = k - 1;
                    	if (k<0)
                    	{
                    		j = 0;
                    	}
                    	else
                    	{
                            j = k as usize;
                    	}
                    }
                    let mut num = j+1;
                    if (k<0)
                    {
                    	num = 0;
                    }
                    hand[num] = reff;

	            }

            }

            // sort that sorts an already sorted array by value
            pub fn suit_sort(hand: &mut Vec<Card<PlayingCard>>)
            {
            	let mut temp_hand:Vec<Card<PlayingCard>>=Vec::new();
            	for i in 0..hand.len()
            	{
            		if hand[i].get_specific().suit == Suit::Heart
            		{
            			temp_hand.push(hand[i]);
            		}
            	}
            	for i in 0..hand.len()
            	{
            		if hand[i].get_specific().suit == Suit::Club
            		{
            			temp_hand.push(hand[i]);
            		}
            	}
            	for i in 0..hand.len()
            	{
            		if hand[i].get_specific().suit == Suit::Spade
            		{
            			temp_hand.push(hand[i]);
            		}
            	}
            	for i in 0..hand.len()
            	{
            		if hand[i].get_specific().suit == Suit::Diamond
            		{
            			temp_hand.push(hand[i]);
            		}
            	}
            	hand.clone_from(&temp_hand);

            }
	    }
