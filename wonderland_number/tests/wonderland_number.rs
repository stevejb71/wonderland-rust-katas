extern crate wonderland_number;

use wonderland_number::*;

#[test]
pub fn zero_is_not_the_wonderland_number() {
    assert!(wonderland_number::wonderland_number() != 0);
}

#[test]
pub fn wonderland_number_has_the_same_digits_when_multiplied_by_2() {
    assert_same_digits(wonderland_number::wonderland_number(), 2);
}

#[test]
pub fn wonderland_number_has_the_same_digits_when_multiplied_by_3() {
    assert_same_digits(wonderland_number::wonderland_number(), 3);
}

#[test]
pub fn wonderland_number_has_the_same_digits_when_multiplied_by_4() {
    assert_same_digits(wonderland_number::wonderland_number(), 4);
}

#[test]
pub fn wonderland_number_has_the_same_digits_when_multiplied_by_5() {
    assert_same_digits(wonderland_number::wonderland_number(), 5);
}

#[test]
pub fn wonderland_number_has_the_same_digits_when_multiplied_by_6() {
    assert_same_digits(wonderland_number::wonderland_number(), 6);
}

fn assert_same_digits(number: u32, multiplier: u32) {
    assert_eq!(sorted_digits(number), sorted_digits(number * multiplier));
}

/// You might not need to use a computer to solve this
/// But if you do, this function may be useful in the wonderland_number crate itself.
fn sorted_digits(number: u32) -> Vec<u8> {
    panic!("For you to do")
}
