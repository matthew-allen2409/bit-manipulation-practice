use crate::*;

#[test]
fn is_lsb_set_is_set_expect_true() {
    let bits: u32 = 21;

    assert!(is_lsb_set(bits))
}

#[test]
fn is_lsb_set_is_set_expect_false() {
    let bits: u32 = 20;

    assert!(!is_lsb_set(bits))
}

#[test]
fn is_msb_set_expect_true() {
    let bits: u32 = 2147483648;

    assert!(is_msb_set(bits))
}

#[test]
fn is_msb_set_expect_false() {
    let bits: u32 = 2147483648 - 1;

    assert!(!is_msb_set(bits))
}

#[test]
fn get_nth_bit_expect_1() {
    let bits: u32 = 10;
    let n = 3;

    assert!(get_nth_bit(bits, n))
}

#[test]
fn get_nth_bit_expect_0() {
    let bits: u32 = 1;
    let n = 3;

    assert!(!get_nth_bit(bits, n))
}

#[test]
fn clear_nth_bit_expect_cleared_bit() {
    let mut bytes: u32 = 0b1010;
    let expected: u32 = 0b0010;

    clear_nth_bit(&mut bytes, 3);

    assert_eq!(expected, bytes);
}

#[test]
fn clear_nth_bit_already_cleared_expect_cleared_bit() {
    let mut bytes: u32 = 0b0010;
    let expected: u32 = 0b0010;

    clear_nth_bit(&mut bytes, 3);

    assert_eq!(expected, bytes);
}

#[test]
fn toggle_nth_bit_0_to_1() {
    let mut bytes: u32 = 0b0010;
    let expected: u32 = 0b1010;

    toggle_nth_bit(&mut bytes, 3);

    assert_eq!(expected, bytes);
}

#[test]
fn toggle_nth_bit_1_to_0() {
    let mut bytes: u32 = 0b1010;
    let expected: u32 = 0b0010;

    toggle_nth_bit(&mut bytes, 3);

    assert_eq!(expected, bytes);
}

#[test]
fn get_highest_set_bit_expect_index() {
    let bytes: u32 = 0b0000100101110;
    let expected: Option<u8> = Some(8);

    let result = get_highest_set_bit(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn get_highest_set_bit_expect_none() {
    let bytes: u32 = 0;
    let expected: Option<u8> = None;

    let result = get_highest_set_bit(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn get_lowest_set_bit_expect_index() {
    let bytes: u32 = 0b0000100101110;
    let expected: Option<u8> = Some(1);

    let result = get_lowest_set_bit(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn get_lowest_set_bit_expect_none() {
    let bytes: u32 = 0;
    let expected: Option<u8> = None;

    let result = get_lowest_set_bit(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn count_trailing_zeroes_expect_n() {
    let bytes = 0b000101111000011000000;
    let expected = 6;

    let result = count_trailing_zeroes(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn count_trailing_zeroes_expect_zero() {
    let bytes = 0b111111111111111111;
    let expected = 0;

    let result = count_trailing_zeroes(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn count_leading_zeroes_expect_n() {
    let bytes: u32 = 0b00000000_00000010_11110000_11000000;
    let expected = 14;

    let result = count_leading_zeroes(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn count_leading_zeroes_expect_zero() {
    let bytes = 0b11111111_11111111_11111111_11111111;
    let expected = 0;

    let result = count_leading_zeroes(&bytes);

    assert_eq!(expected, result);
}

#[test]
fn flip_all_bits_expect() {
    let mut bytes: u32 = 0b10011011_00000000_11111111_00000000;
    let expected: u32 = 0b01100100_11111111_00000000_11111111;

    flip_all_bits(&mut bytes);

    assert_eq!(expected, bytes);
}
