fn main() {
    let password_range = PasswordRange::new(382_345, 843_167);
    println!("{}", password_range.count());
}

struct PasswordRange(isize, isize, isize);

impl PasswordRange {
    fn new(start: isize, end: isize) -> Self {
        Self(start, end, start)
    }
}

impl Iterator for PasswordRange {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.2;
            if current > self.1 {
                return None;
            }
            self.2 += 1;
            if is_valid_password(current) {
                return Some(current);
            }
        }
    }
}

fn is_valid_password(password: isize) -> bool {
    // we could reuse this buffer from one invocation to the next, but it's not like
    // we're writing efficient code here
    let base_10_decomposition = &mut [0; 6];
    make_base_10_decomposition(password, base_10_decomposition);
    //part 1
    //has_adjacent_digits(base_10_decomposition) && has_only_increasing_digits(base_10_decomposition)
    //part 2
    has_one_group_of_exactly_two_adjacent_digits(base_10_decomposition) && has_only_increasing_digits(base_10_decomposition)
}

// assume length buffer is big enough
fn make_base_10_decomposition(input: isize, buffer: &mut [isize]) {
    let mut next = input;
    let mut i = 0;
    while next >= 10 {
        let digit = next % 10;
        next /= 10;
        buffer[i] = digit;
        i += 1;
    }
    buffer[i] = next;
}

fn has_adjacent_digits(digits: &[isize]) -> bool {
    for slice in digits.windows(2) {
        if slice[0] == slice[1] {
            return true;
        }
    }
    false
}

fn has_one_group_of_exactly_two_adjacent_digits(digits: &[isize]) -> bool {
    let mut current_run = digits[0];
    let mut current_run_size = 1;
    for digit in digits.iter().skip(1) {
        if current_run != *digit {
            if current_run_size == 2 {
                return true;
            }
            current_run = *digit;
            current_run_size = 1;
        } else {
            current_run_size += 1;
        }
    }
    current_run_size == 2
}

// used only for part 1
fn has_only_increasing_digits(digits: &[isize]) -> bool {
    // our base10 decomposition has low digits at low indices
    for slice in digits.windows(2) {
        if slice[0] < slice[1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_base_10_decomposition() {
        let buffer = &mut [0 ; 6];
        make_base_10_decomposition(120456, buffer);
        assert_eq!(buffer, &mut [6, 5, 4, 0, 2, 1]);
    }

    #[test]
    fn test_has_one_group_of_exactly_two_adjacent_digits() {
        assert_eq!(has_one_group_of_exactly_two_adjacent_digits(&[1, 1, 2, 2, 3, 3]), true);
        assert_eq!(has_one_group_of_exactly_two_adjacent_digits(&[1, 2, 3, 4, 4, 4]), false);
        assert_eq!(has_one_group_of_exactly_two_adjacent_digits(&[1, 1, 1, 1, 2, 2]), true);
        assert_eq!(has_one_group_of_exactly_two_adjacent_digits(&[1, 1, 3, 4, 5, 6]), true);
        assert_eq!(has_one_group_of_exactly_two_adjacent_digits(&[3, 4, 5, 6, 1, 1]), true);
    }

}
