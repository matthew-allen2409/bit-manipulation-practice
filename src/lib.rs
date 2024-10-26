pub fn is_lsb_set(bytes: u32) -> bool {
    bytes & 0b1 != 0
}

pub fn is_msb_set(bytes: u32) -> bool {
    bytes & (0b1 << 31) != 0
}

pub fn get_nth_bit(bytes: u32, n: u8) -> bool {
    if n > 31 {
        panic!("n can only be in range of 0 - 31");
    }

    bytes & (0b1 << n) != 0
}

pub fn clear_nth_bit(bytes: &mut u32, n: u8) {
    if n > 31 {
        panic!("n can only be in range of 0 - 31");
    }

    *bytes &= !(1 << n);
}

pub fn toggle_nth_bit(bytes: &mut u32, n: u8) {
    if n > 31 {
        panic!("n can only be in range of 0 - 31");
    }

    *bytes ^= 1 << n;
}

pub fn get_highest_set_bit(bytes: &u32) -> Option<u8> {
    for n in (0..32).rev() {
        if bytes & (1 << n) != 0 {
            return Some(n);
        }
    }

    None
}

pub fn get_lowest_set_bit(bytes: &u32) -> Option<u8> {
    for n in 0..32 {
        if bytes & (1 << n) != 0 {
            return Some(n);
        }
    }

    None
}

pub fn count_trailing_zeroes(bytes: &u32) -> u8 {
    let mut count = 0;

    for n in 0..32 {
        if bytes & (1 << n) == 0 {
            count += 1;
        } else {
            break;
        }
    }

    count
}

pub fn count_leading_zeroes(bytes: &u32) -> u8 {
    let mut count = 0;

    for n in (0..32).rev() {
        if bytes & (1 << n) == 0 {
            count += 1;
        } else {
            break;
        }
    }

    count
}

pub fn flip_all_bits(bytes: &mut u32) {
    for n in 0..32 {
        *bytes ^= 1 << n;
    }
}

#[cfg(test)]
mod tests;
