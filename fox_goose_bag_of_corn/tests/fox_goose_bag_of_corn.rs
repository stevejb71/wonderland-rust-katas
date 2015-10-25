extern crate fox_goose_bag_of_corn;

use fox_goose_bag_of_corn::*;

#[test]
pub fn at_the_start_everyone_is_on_the_west_bank_of_the_river() {
	assert_eq!(Positions::new(vec![Actor::Fox, Actor::Goose, Actor::Corn, Actor::You], vec![], vec![]), crossing_plan()[0])
}

#[test]
pub fn at_the_end_everyone_is_on_the_east_bank_of_the_river() {
	assert_eq!(Positions::new(vec![], vec![], vec![Actor::Fox, Actor::Goose, Actor::Corn, Actor::You]), *crossing_plan().last().unwrap())
}

#[test]
pub fn the_fox_and_the_goose_are_never_left_alone_together() {
	assert!(crossing_plan().iter().all(|p| is_safe(Actor::Fox, Actor::Goose, &p)));
}

#[test]
pub fn the_goose_and_the_corn_are_never_left_alone_together() {
	assert!(crossing_plan().iter().all(|p| is_safe(Actor::Goose, Actor::Corn, &p)));
}

#[test]
pub fn the_boat_can_only_carry_you_and_one_other() {
	assert!(crossing_plan().iter().all(|p| p.boat.len() == 2 || p.boat.len() == 0));
}	

#[test]
pub fn only_you_and_one_other_can_move_each_step() {
	panic!("For you to implement");
}	

fn is_safe(a1: Actor, a2: Actor, p: &Positions) -> bool {
	!contains_both(a1, a2, &p.west) && !contains_both(a1, a2, &p.boat) && !contains_both(a1, a2, &p.east) 
}

fn contains_both(a1: Actor, a2: Actor, actors: &Vec<Actor>) -> bool {
	actors.contains(&a1) && actors.contains(&a2) && !actors.contains(&Actor::You)
}