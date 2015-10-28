extern crate fox_goose_bag_of_corn;

use fox_goose_bag_of_corn::*;

#[test]
pub fn at_the_start_everyone_is_on_the_west_bank_of_the_river() {
    assert_eq!(Situation {fox_position: Position::West, goose_position: Position::West, corn_position: Position::West, your_position: Position::West}, crossing_plan()[0])
}

#[test]
pub fn at_the_end_everyone_is_on_the_east_bank_of_the_river() {
    panic!("For you to implement");
}

#[test]
pub fn the_fox_and_the_goose_are_never_left_alone_together() {
    assert!(crossing_plan().iter().all(|s| s.your_position == s.goose_position || s.fox_position != s.goose_position));
}

#[test]
pub fn the_goose_and_the_corn_are_never_left_alone_together() {
    panic!("For you to implement");
}

#[test]
pub fn the_boat_can_only_carry_you_and_one_other() {
    panic!("For you to implement");
}   

#[test]
pub fn only_you_and_one_other_can_move_each_step() {
    panic!("For you to implement");
}   