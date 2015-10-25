extern crate doublets;

use doublets::*;

#[test]
pub fn word_to_itself() {
	assert_doublets(&vec!["head", "head"], "head", "head");
}

#[test]
pub fn head_to_tail() {
	assert_doublets(&vec!["head", "heal", "teal", "tell", "tall", "tail"], "head", "tail");
}

#[test]
pub fn door_to_lock() {
	assert_doublets(&vec!["door", "boor", "book", "look", "lock"], "door", "lock");
}

#[test]
pub fn bank_to_loan() {
	assert_doublets(&vec!["bank", "bonk", "book", "look", "loon", "loan"], "bank", "loan");
}

#[test]
pub fn wheat_to_bread() {
	assert_doublets(&vec!["wheat", "cheat", "cheap", "cheep", "creep", "creed", "breed", "bread"], "wheat", "bread");
}

#[test]
pub fn no_doublet_found() {
	assert_eq!(None, doublets("ye", "freezer"));
}

fn assert_doublets(expected: &Vec<&str>, start: &str, end: &str) {
	let expected_as_strings: Vec<String> = expected.iter().map(|x| x.to_string()).collect();
	assert_eq!(Some(expected_as_strings), doublets(start, end));
}