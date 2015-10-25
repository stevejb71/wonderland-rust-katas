#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
	Spades, Clubs, Diamonds, Hearts
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
	Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}

#[derive(Debug, PartialEq)]
pub struct Card {
	suit: Suit,
	rank: Rank
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
	Player1, Player2
}

impl Card {
	pub fn new(suit: Suit, rank: Rank) -> Card {
		Card {suit: suit, rank: rank}
	}
}

pub fn play_round(card1: &Card, card2: &Card) -> Player {
	panic!("returns the player that has the best card");
}

pub fn play_game(player1_cards: &Vec<&Card>, player2_cards: &Vec<&Card>) -> Player {
	panic!("returns the player that wins the game");
}