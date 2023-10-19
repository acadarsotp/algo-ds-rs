pub fn shell_sort<T: PartialOrd + Copy>(nums: &mut [T]) {
    let len = nums.len();
    let mut gap = len / 2;

    while gap > 0 {
        for start in 0..gap {
            insertion_sort_gap(nums, start, gap);
        }

        gap /= 2;
    }
}

fn insertion_sort_gap<T: PartialOrd + Copy>(nums: &mut [T], start: usize, gap: usize) {
    let len = nums.len();
    let mut i = start + gap;

    while i < len {
        let curr = nums[i];
        let mut pos = i;

        while pos >= gap && curr < nums[pos - gap] {
            nums.swap(pos, pos - gap);
            pos -= gap;
        }

        nums[pos] = curr;
        i += gap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort_empty_collection() {
        let mut nums: [i32; 0] = [];
        shell_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_shell_sort_single_element() {
        let mut nums = [5];
        shell_sort(&mut nums);
        assert_eq!(nums, [5]);
    }

    #[test]
    fn test_shell_sort_multiple_elements() {
        let mut nums_a = [5, 3, 8, 4, 2];
        shell_sort(&mut nums_a);
        assert_eq!(nums_a, [2, 3, 4, 5, 8]);

        let mut nums_b = vec![5, 3, 8, 4, 2];
        shell_sort(&mut nums_b);
        assert_eq!(nums_b, vec![2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_shell_sort_different_types() {
        let mut nums = [5.0, 3.0, 8.0, 4.0, 2.0];
        shell_sort(&mut nums);
        assert_eq!(nums, [2.0, 3.0, 4.0, 5.0, 8.0]);
    }
}
