extern crate magic_square;

use magic_square::*;

#[test]
pub fn magic_square_contains_the_postage_stamp_values() {
    let mut values = create_magic_square().values;
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(POSTAGE_STAMP_VALUES, values);
}

#[test]
pub fn rows_sum_to_the_same_value() {
    let values = create_magic_square().values;
    let magic_number = magic_number();
    for &sum in sum_rows(values).iter() {
        assert_eq!(magic_number, sum);
    }
}

#[test]
pub fn columns_sum_to_the_same_value() {
    let values = create_magic_square().values;
    let magic_number = magic_number();
    for &sum in sum_columns(values).iter() {
        assert_eq!(magic_number, sum);
    }
}

#[test]
pub fn diagonals_sum_to_the_same_value() {
    let values = create_magic_square().values;
    let magic_number = magic_number();
    for &sum in sum_diagonals(values).iter() {
        assert_eq!(magic_number, sum);
    }
}

fn magic_number() -> f32 {
    sum_rows(create_magic_square().values)[0]
}

fn sum_rows(values: [f32; 9]) -> [f32; 3] {
    panic!("For you to do");
}

fn sum_columns(values: [f32; 9]) -> [f32; 3] {
    panic!("For you to do");
}

fn sum_diagonals(values: [f32; 9]) -> [f32; 3] {
    panic!("For you to do");
}
