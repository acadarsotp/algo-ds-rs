pub fn quick_sort<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(nums);

    if pivot_index > 0 {
        quick_sort(&mut nums[0..pivot_index]);
    }

    if pivot_index < len - 1 {
        quick_sort(&mut nums[pivot_index + 1..]);
    }
}

fn partition<T: PartialOrd>(nums: &mut [T]) -> usize {
    let len = nums.len();
    let pivot_index = len / 2;

    nums.swap(pivot_index, len - 1); // Move the pivot element to the end

    let mut i = 0;
    for j in 0..len - 1 {
        if nums[j] <= nums[len - 1] {
            nums.swap(i, j);
            i += 1;
        }
    }

    nums.swap(i, len - 1); // Move the pivot element to its final position
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_empty() {
        let mut vec: Vec<i32> = Vec::new();
        quick_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_quick_sort_different_collections() {
        let mut vec = vec![5, 2, 1, 3, 6, 4];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6]);

        let mut arr = [5, 2, 1, 3, 6, 4];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_quick_sort_different_data_types() {
        let mut vec = vec![5.2, 2.1, 1.5, 3.7, 6.4, 4.3];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![1.5, 2.1, 3.7, 4.3, 5.2, 6.4]);

        let mut arr = ["apple", "dog", "cat", "banana", "elephant", "fox"];
        quick_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "cat", "dog", "elephant", "fox"]);
    }
}
