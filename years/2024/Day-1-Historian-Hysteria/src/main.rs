use std::io::Write;

use part1::calc_part_one;
use part2::calc_part_two;

mod part1;
mod part2;

pub const fn build_arrs() -> ([u32; 1000], [u32; 1000]) {
    let input: [u8; 14000] = *include_bytes!("../input.txt");

    let mut a: [u32; 1000] = [0; 1000];
    let mut b: [u32; 1000] = [0; 1000];

    let mut ptr = input.as_ptr();
    let mut idx = 0;
    while idx < 1000 {
        a[idx] = unsafe { parse_left_num(&mut ptr) };
        b[idx] = unsafe { parse_right_num(&mut ptr) };
        idx += 1;
    }

    (a, b)
}

// basic atoi
const unsafe fn parse_left_num(bytes_ptr: &mut *const u8) -> u32 {
    let mut num = 0u32;
    while unsafe { **bytes_ptr != b' ' } {
        num = num * 10 + unsafe { **bytes_ptr - b'0' } as u32;
        *bytes_ptr = bytes_ptr.wrapping_add(1); // move pointer up
    }
    *bytes_ptr = bytes_ptr.wrapping_add(3); // skip 3 spaces to next number
    num
}

const unsafe fn parse_right_num(bytes_ptr: &mut *const u8) -> u32 {
    let mut num = 0u32;
    while unsafe { **bytes_ptr != b'\n' } {
        num = num * 10 + unsafe { **bytes_ptr - b'0' } as u32;
        *bytes_ptr = bytes_ptr.wrapping_add(1); // move pointer up
    }
    *bytes_ptr = bytes_ptr.wrapping_add(1); // skip newline
    num
}

// For day 1, I decided my challenge was to write all my logic in const functions
fn main() {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    let start_time = std::time::Instant::now();
    let result = const { calc_part_one() };
    let part_one_time = start_time.elapsed();
    writeln!(handle, "Part One: {}\nTime: {:?}", result, part_one_time).unwrap();

    let start_time = std::time::Instant::now();
    let result = const { calc_part_two() };
    let part_two_time = start_time.elapsed();
    writeln!(handle, "Part Two: {}\nTime: {:?}", result, part_two_time).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num() {
        let input = b"12345   45678\n91234   56789\n";
        let mut ptr = input.as_ptr();
        assert_eq!(unsafe { parse_left_num(&mut ptr) }, 12345);
        assert_eq!(unsafe { parse_right_num(&mut ptr) }, 45678);
        assert_eq!(unsafe { parse_left_num(&mut ptr) }, 91234);
        assert_eq!(unsafe { parse_right_num(&mut ptr) }, 56789);
    }
}
