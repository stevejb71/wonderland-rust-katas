#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Actor {
	Fox, Goose, Corn, You
}

#[derive(Debug, PartialEq)]
pub struct Positions {
	pub west: Vec<Actor>,
	pub boat: Vec<Actor>,
	pub east: Vec<Actor>
}

impl Positions {
	pub fn new(west: Vec<Actor>, boat: Vec<Actor>, east: Vec<Actor>) -> Positions {
		Positions {west: west, boat: boat, east: east}
	}
}

pub fn crossing_plan() -> Vec<Positions> {
	panic!("implement")
}