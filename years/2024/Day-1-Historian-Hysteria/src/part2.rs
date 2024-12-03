use crate::build_arrs;

pub const fn calc_part_two() -> u32 {
    let (a, b) = build_arrs();

    // If we weren't in a const context I would do this efficiently, I however will not since I can just throw it in a const block
    let mut similarity_score = 0;
    let mut a_idx = 0;

    while a_idx < 1000 {
        let mut b_idx = 0;
        let mut times_seen = 0;
        let a_val = a[a_idx];
        while b_idx < 1000 {
            let b_val = b[b_idx];
            if a_val == b_val {
                times_seen += 1;
            }

            b_idx += 1;
        }
        similarity_score += times_seen * a_val;
        a_idx += 1;
    }

    similarity_score
}
