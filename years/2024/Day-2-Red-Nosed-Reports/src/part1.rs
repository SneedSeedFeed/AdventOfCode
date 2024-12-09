#[derive(Debug)]
pub struct RedNoseReactorReport {
    pub levels: Box<[i32]>,
}

pub fn levels_safe(mut levels: impl Iterator<Item = i32>) -> bool {
    let first = levels.next().unwrap();
    let second = levels.next().unwrap();

    let diff = first - second;
    if !(1..4).contains(&diff.abs()) {
        false
    } else {
        let diffs_should_be_positive = diff.is_positive(); // no need to account for 0

        let mut prev = second;
        for cur in levels {
            let diff = prev - cur;
            if !(1..4).contains(&diff.abs()) {
                return false;
            }
            if diff.is_positive() != diffs_should_be_positive {
                return false;
            }
            prev = cur;
        }
        true
    }
}

impl RedNoseReactorReport {
    pub fn construct_reports(input: &[u8]) -> Box<[Self]> {
        let mut ptr = input.as_ptr();
        let mut reports = Vec::new();
        while unsafe { ptr < input.as_ptr().add(input.len()) } {
            reports.push(unsafe { Self::parse_report(&mut ptr) });
            ptr = ptr.wrapping_add(1);
        }
        reports.into_boxed_slice()
    }

    unsafe fn parse_report(ptr: &mut *const u8) -> Self {
        let mut levels = Vec::new();
        while **ptr != b'\n' {
            levels.push(Self::parse_num(ptr));
        }
        Self {
            levels: levels.into_boxed_slice(),
        }
    }

    unsafe fn parse_num(ptr: &mut *const u8) -> i32 {
        let mut num = 0;
        loop {
            let val = **ptr;
            if val == b' ' {
                *ptr = ptr.wrapping_add(1);
                break;
            } else if val == b'\n' {
                break;
            } else {
                num = num * 10 + (val - b'0') as i32;
                *ptr = ptr.wrapping_add(1);
            }
        }
        num
    }

    pub fn is_safe_part_one(&self) -> bool {
        levels_safe(self.levels.iter().copied())
    }
}
