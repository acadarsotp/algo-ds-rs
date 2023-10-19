pub fn insertion_sort<T: PartialOrd>(nums: &mut [T]) {
    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 && nums[j] < nums[j - 1] {
            nums.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn binary_insertion_sort<T: PartialOrd + Copy>(nums: &mut [T]) {
    for i in 1..nums.len() {
        let mut left = 0;
        let mut right = i;
        let temp = nums[i];

        while left < right {
            let mid = left + (right - left) / 2;
            if temp < nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        nums[left..=i].rotate_right(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_empty() {
        let mut vec: Vec<i32> = Vec::new();
        insertion_sort(&mut vec);
        assert_eq!(vec, vec![]);

        let mut vec: Vec<i32> = Vec::new();
        binary_insertion_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_insertion_sort_different_collections() {
        let mut vec = vec![5, 2, 1, 3, 6, 4];
        insertion_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6]);

        let mut arr = [5, 2, 1, 3, 6, 4];
        let len = arr.len();
        binary_insertion_sort(&mut arr[0..len]);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_insertion_sort_different_data_types() {
        let mut vec = vec![5.2, 2.1, 1.5, 3.7, 6.4, 4.3];
        insertion_sort(&mut vec);
        assert_eq!(vec, vec![1.5, 2.1, 3.7, 4.3, 5.2, 6.4]);

        let mut arr = ["apple", "dog", "cat", "banana", "elephant", "fox"];
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "cat", "dog", "elephant", "fox"]);
    }
}
