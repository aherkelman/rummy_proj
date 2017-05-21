
use std::fmt::Debug;
	use rand;
	use rand::Rng;
	


	#[derive(Debug)]
	pub struct Card<T:Debug>
	{
		details:T
	}

	impl<T:Debug> Card<T>
	{
	    // Function to create a new card based on passed on details passed in
	    pub fn new(detail:T)->Card<T>
	    {
	        Card{details:detail}
	    }
	    // Function to return the dtails of a card
	    pub fn get_specific(&self)->&T
	    {
	        &self.details
	    }
	    pub fn get_mut_specific(&mut self) -> &mut T
	    {
	    	&mut self.details
	    }

	}

	// Deck Struct needs the following:
	//     Members:
	//     Current Deck - This is the current draw deck.
	//     Current Discard - These are the cards currently in the disacard pile
	//     Shuffle - Method used to randomly shuffle the "deck". Simple swap with random value for each
	//         value in the array. No return.
	//     Reset - Move all cards from discard into deck.
	//     Draw Card - Return value of next card in deck, skip card if it is not of enum type "in deck"
	//         will also set the status of drawn card to "in play". If this is last card, call shuffle
	//         and reset the current position to the start of deck.
	//     New Game - Intialize all the values and get ready for game (shuffle so on) (create struct function)
	//     Discard Card - Helper funtion to discard a card, nothing complicated


#[derive(Debug)]
	    pub struct Deck<T:Debug>
	    {
	        pub current_deck:Vec<Card<T>>, // vector of cards in deck
	        pub discard_deck:Vec<Card<T>> // vector of cards in discard
	    }

	    impl<T:Debug> Deck<T>
	    {
	    
	        // Function to make new deck by passing in an array of card discriptions
	        pub fn new(arr:Vec<Card<T>>)->Deck<T>{
	            // Create new vectory to copy cards into
	            //let mut init_vec:Vec<Card<T>>=Vec::new();
	            // For each element of passed in array add a card 

	            //let init_vec:Vec<Card<T>>=arr.iter().map(|a|Card::new(*a)).collect();
	            /*
	            for i in arr.iter(){
	                init_vec.push(Card::new(*i));
	            }*/

	            // Inatialize the fields of current deck to all cards and discard to none
	            Deck{current_deck:arr, discard_deck:Vec::new()}
	        }
            
            // Randomly shuffle all cards in current deck
	        pub fn shuffle(&mut self)
	        {
	        	println!("Deck::shuffle");
	        	// For each card in the deck, swap with a random card
	        	for i in 0..self.current_deck.len()
	        	{
	        		// Find a random other card
	        		let num=rand::thread_rng().gen_range(0, self.current_deck.len());
                 	

	        		self.current_deck.swap(i,num);
	        		/*
                 	// Store current card in temp
                 	let temp:Card<T>=self.current_deck[i];
                 	// Set current card to random card
                 	self.current_deck[i] = self.current_deck[num];
                 	// Set random card to current card stored in temp
                 	self.current_deck[num] = temp;   
	        		*/
	        	}
	        }

	        // Draw top card, if this is the last card reset stuff
	        pub fn draw_card(&mut self) -> Option<Card<T>>
	        {
	        	// If the deck is empty reset
	        	if self.current_deck.is_empty()
	        	{
	        		println!("Deck is empty");
	        		
	        		self.reset();
	        		if self.current_deck.is_empty(){
	        			return None
	        		}
	        	}
	        	//println!("drawing cards");
	        	// Ask ken how to handle this situation if neither deck has cards
	        	// what can you do as a return?
	        	// if self.current_deck.len()!=0
	        	// {
	        		//println!("draw card 1");
	        	    // Set next card to last card in deck (backwards order to avoid shifting)
	        	    //let next_card:Card<T> = self.current_deck[self.current_deck.len()-1];
	        	    // Remove drawn card from deck
	        	    self.current_deck.pop()

	        	    //println!("draw card 2");
	        	    // Return next card
	        	    //Some(next_card)
	            // }
	            // else
	            // {
	            // }
	        	
	        }

            // Reset all cards into current deck and shuffle
	        pub fn reset(&mut self) 
	        {
	        	//println!("reset");
	        	// Move all cards from discard into the current and empty the discard
	        	// This happens to be exactly what the append function does!
	            self.current_deck.append(&mut self.discard_deck);
	            // Then call shuffle to shuffle current deck
	            self.shuffle();
	        }

	        // Function to put inputted card into the discard pile
	        pub fn discard(&mut self, card:Card<T>)
	        {
                println!("Deck::discard");

                self.discard_deck.push(card);
                println!("discard 2");
	        }
	    }

