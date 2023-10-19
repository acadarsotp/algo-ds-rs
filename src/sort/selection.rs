pub fn selection_sort<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();

    for i in 0..len {
        if let Some((min_idx, _)) = nums[i..]
            .iter()
            .enumerate()
            .min_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap())
        {
            nums.swap(i, i + min_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort_i32() {
        let mut nums = vec![5, 2, 9, 1, 5, 6];
        selection_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_selection_sort_f64() {
        let mut nums = vec![5.5, 2.2, 9.9, 1.1, 5.5, 6.6];
        selection_sort(&mut nums);
        assert_eq!(nums, vec![1.1, 2.2, 5.5, 5.5, 6.6, 9.9]);
    }

    #[test]
    fn test_selection_sort_string() {
        let mut nums = vec!["apple", "banana", "kiwi", "grape", "orange"];
        selection_sort(&mut nums);
        assert_eq!(nums, vec!["apple", "banana", "grape", "kiwi", "orange"]);
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut nums: Vec<i32> = Vec::new();
        selection_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_selection_sort_single() {
        let mut nums = vec![5];
        selection_sort(&mut nums);
        assert_eq!(nums, vec![5]);
    }
}
