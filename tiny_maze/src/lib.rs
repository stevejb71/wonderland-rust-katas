#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Piece {
    Start, Wall, Space, End, Marker
}

#[derive(Debug, PartialEq)]
pub struct Maze {
    maze: Vec<Piece>
}

impl Maze {
    pub fn new(maze: Vec<Piece>) -> Maze {
        Maze{maze: maze}
    }

    pub fn solve(&self) -> Maze {
        panic!("For you to do");
    }
}