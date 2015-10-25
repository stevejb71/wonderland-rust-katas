extern crate card_game_war;

use card_game_war::*;

#[test]
pub fn the_highest_rank_wins_the_cards_in_the_round() {
	assert_eq!(Player::Player2, play_round(&Card::new(Suit::Spades, Rank::Jack), &Card::new(Suit::Spades, Rank::King)));
}

#[test]
pub fn queens_are_higher_rank_than_jacks() {
	assert_eq!(Player::Player1, play_round(&Card::new(Suit::Spades, Rank::Queen), &Card::new(Suit::Spades, Rank::Jack)));
}

#[test]
pub fn aces_are_higher_rank_than_kings() {
	assert_eq!(Player::Player1, play_round(&Card::new(Suit::Spades, Rank::Ace), &Card::new(Suit::Spades, Rank::King)));
}

#[test]
pub fn if_the_ranks_are_equal_then_clubs_beat_spades() {
	assert_eq!(Player::Player2, play_round(&Card::new(Suit::Spades, Rank::Seven), &Card::new(Suit::Clubs, Rank::Seven)));
}

#[test]
pub fn if_the_ranks_are_equal_then_diamonds_beat_clubs() {
	assert_eq!(Player::Player2, play_round(&Card::new(Suit::Clubs, Rank::Seven), &Card::new(Suit::Diamonds, Rank::Seven)));
}

#[test]
pub fn if_the_ranks_are_equal_then_hearts_beat_diamonds() {
	assert_eq!(Player::Player1, play_round(&Card::new(Suit::Hearts, Rank::Seven), &Card::new(Suit::Diamonds, Rank::Seven)));
}

#[test]
pub fn the_player_loses_when_they_run_out_of_cards() {
	panic!("for you to implement");
}
