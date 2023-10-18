pub fn insertion_sort<T: PartialOrd + Copy>(nums: &mut [T]) {
    if nums.len() <= 1 {
        return;
    }
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];
        while pos > 0 && curr < nums[pos - 1] {
            // move element to right
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }

        // insert element: curr
        nums[pos] = curr;
    }
}

pub fn binary_insertion_sort<T: PartialOrd + Copy>(nums: &mut [T]) {
    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() {
        left = 0;
        right = i - 1;
        temp = nums[i];

        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        for j in (left..i).rev() {
            nums.swap(j, j + 1);
        }

        if left != i {
            nums[left] = temp;
        }
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
