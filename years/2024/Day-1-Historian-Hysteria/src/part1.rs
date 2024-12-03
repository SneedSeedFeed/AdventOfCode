use crate::build_arrs;

pub const fn calc_part_one() -> u32 {
    let (mut a, mut b) = build_arrs();

    insertion_sort(&mut a);
    insertion_sort(&mut b);

    let mut sum = 0;
    let mut idx = 0;
    while idx < 1000 {
        sum += a[idx].abs_diff(b[idx]);
        idx += 1;
    }

    sum
}

const fn insertion_sort(arr: &mut [u32]) {
    let mut i = 1;
    while i < arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            let tmp = arr[j];
            arr[j] = arr[j - 1];
            arr[j - 1] = tmp;
            j -= 1;
        }
        i += 1;
    }
}
