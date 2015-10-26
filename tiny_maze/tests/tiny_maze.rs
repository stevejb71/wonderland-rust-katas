extern crate tiny_maze;

use tiny_maze::*;

#[test]
pub fn can_solve_3x3_maze() {
	let maze = Maze::new(vec![Piece::Start, Piece::Space, Piece::Wall,  Piece::Wall, Piece::Space, Piece::Wall,  Piece::Wall, Piece::Space, Piece::End]);
	let solved = Maze::new(vec![Piece::Marker, Piece::Marker, Piece::Wall,  Piece::Wall, Piece::Marker, Piece::Wall,  Piece::Wall, Piece::Marker, Piece::Marker]);

	assert_eq!(solved, maze.solve());
}

#[test]
pub fn can_solve_4x4_maze() {
	let maze = Maze::new(vec![Piece::Start, Piece::Space, Piece::Space, Piece::Wall,  Piece::Wall, Piece::Wall, Piece::Space, Piece::Wall,  Piece::Wall, Piece::Space, Piece::Space, Piece::Wall]);
	let solved = Maze::new(vec![Piece::Marker, Piece::Marker, Piece::Marker, Piece::Wall,  Piece::Wall, Piece::Wall, Piece::Marker, Piece::Wall,  Piece::Wall, Piece::Space, Piece::Marker, Piece::Wall,  Piece::Wall, Piece::Wall, Piece::Marker, Piece::Marker]);

	assert_eq!(solved, maze.solve());
}