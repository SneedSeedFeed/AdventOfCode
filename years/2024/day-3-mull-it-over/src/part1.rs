#[derive(PartialEq, Eq, Clone, Copy)]
enum PartOneReadState {
    SearchingForM,
    FoundM,
    FoundU,
    FoundL,
    ReadLNum(i64),
    ReadRNum { left: i64, right: i64 },
}

pub fn part_one(bytes: &[u8]) -> i64 {
    let ptr_range = bytes.as_ptr_range();

    let mut cursor = ptr_range.start;
    let mut sum: i64 = 0;
    let mut read_state = PartOneReadState::SearchingForM;
    while cursor < ptr_range.end {
        let cur_byte = unsafe { cursor.read() };
        match read_state {
            PartOneReadState::SearchingForM => {
                if cur_byte == b'm' {
                    read_state = PartOneReadState::FoundM
                }
            }
            PartOneReadState::FoundM => {
                read_state = if cur_byte == b'u' {
                    PartOneReadState::FoundU
                } else {
                    PartOneReadState::SearchingForM
                }
            }
            PartOneReadState::FoundU => {
                read_state = if cur_byte == b'l' {
                    PartOneReadState::FoundL
                } else {
                    PartOneReadState::SearchingForM
                }
            }
            PartOneReadState::FoundL => {
                read_state = if cur_byte == b'(' {
                    PartOneReadState::ReadLNum(0)
                } else {
                    PartOneReadState::SearchingForM
                }
            }
            PartOneReadState::ReadLNum(cur_l) => {
                if cur_l == 0 && !cur_byte.is_ascii_digit() {
                    // no left num starts with 0 so this check is fine
                    read_state = PartOneReadState::SearchingForM
                } else if cur_byte == b',' {
                    read_state = PartOneReadState::ReadRNum {
                        left: cur_l,
                        right: 0,
                    }
                } else if !cur_byte.is_ascii_digit() {
                    read_state = PartOneReadState::SearchingForM
                } else {
                    // We know that if the byte we are looking at is an ascii digit and we have already checked if we are moving on to the next num
                    let digit = (cur_byte - b'0') as i64;
                    read_state = PartOneReadState::ReadLNum(cur_l * 10 + digit)
                }
            }
            PartOneReadState::ReadRNum { left, right } => {
                if right == 0 && !cur_byte.is_ascii_digit() {
                    read_state = PartOneReadState::SearchingForM;
                } else if cur_byte == b')' {
                    sum += left * right;
                    read_state = PartOneReadState::SearchingForM;
                } else if !cur_byte.is_ascii_digit() {
                    read_state = PartOneReadState::SearchingForM;
                } else {
                    let digit = (cur_byte - b'0') as i64;
                    read_state = PartOneReadState::ReadRNum {
                        left,
                        right: right * 10 + digit,
                    }
                }
            }
        }
        unsafe { cursor = cursor.add(1) }
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::part_one;

    #[test]
    fn test_given_input() {
        let input = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let part_one = part_one(input);
        assert_eq!(part_one, 161)
    }
}
