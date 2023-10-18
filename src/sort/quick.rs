pub fn quick_sort<T: PartialOrd>(nums: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let mut lm = low;
    let mut rm = high;

    while lm < rm {
        // right marker move to left gradually
        while lm < rm && nums[low] <= nums[rm] {
            rm -= 1;
        }

        // left marker move to right gradually
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }

        // exchange data between position lm and rm
        nums.swap(lm, rm);
    }

    // exchange data between position low and lm
    nums.swap(low, lm);

    if lm > 1 {
        quick_sort(nums, low, lm - 1);
    }

    quick_sort(nums, rm + 1, high);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_empty() {
        let mut vec: Vec<i32> = Vec::new();
        let len = vec.len();
        quick_sort(&mut vec, 0, len);
        assert_eq!(vec, Vec::<i32>::new());
    }

    #[test]
    fn test_quick_sort_different_collections() {
        let mut vec = vec![5, 2, 1, 3, 6, 4];
        let len = vec.len();
        quick_sort(&mut vec, 0, len - 1);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6]);

        let mut arr = [5, 2, 1, 3, 6, 4];
        let len = arr.len();
        quick_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_quick_sort_different_data_types() {
        let mut vec = vec![5.2, 2.1, 1.5, 3.7, 6.4, 4.3];
        let len = vec.len();
        quick_sort(&mut vec, 0, len - 1);
        assert_eq!(vec, vec![1.5, 2.1, 3.7, 4.3, 5.2, 6.4]);

        let mut arr = ["apple", "dog", "cat", "banana", "elephant", "fox"];
        let len = arr.len();
        quick_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, ["apple", "banana", "cat", "dog", "elephant", "fox"]);
    }
}
