#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    West, Boat, East
}

#[derive(Debug, PartialEq)]
pub struct Situation {
    pub fox_position: Position,
    pub goose_position: Position,
    pub corn_position: Position,
    pub your_position: Position
}

pub fn crossing_plan() -> Vec<Situation> {
    panic!("implement")
}