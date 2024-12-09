use std::ops::Sub;

use crate::part1::RedNoseReactorReport;

impl RedNoseReactorReport {
    pub fn is_safe_part_two(&self) -> bool {
        for exclude in 0..self.levels.len() {
            let (split_a, split_b) = self.levels.split_at(exclude);
            // split_b contains the exclude so we shrink the slice by 1
            let split_b_len = split_b.len();
            let split_b_start = split_b.as_ptr();
            let split_b =
                unsafe { std::slice::from_raw_parts(split_b_start.add(1), split_b_len.sub(1)) };

            let safe_with_exclude =
                crate::part1::levels_safe(split_a.iter().copied().chain(split_b.iter().copied()));
            if safe_with_exclude {
                return true;
            }
        }

        false
    }
}
