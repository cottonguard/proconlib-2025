use crate::bitset::*;

#[test]
fn test() {
    let mut a = [0b00010001u8, 0b00010011u8];
    assert_eq!(a.count_ones(), 5);
    let pos: Vec<_> = a.one_positions().collect();
    assert_eq!(pos, [0, 4, 8, 9, 12]);
    assert_eq!(a.display_bits().to_string(), "1000100011001000");
    assert_eq!(a.set_bit(0, true), true);
    assert_eq!(a.set_bit(1, true), false);
    assert_eq!(a.set_bit(10, true), false);
    assert_eq!(a.set_bit(0, false), true);
    assert_eq!(a.flip_bit(2), false);
    assert_eq!(a.display_bits().to_string(), "0110100011101000");
}
