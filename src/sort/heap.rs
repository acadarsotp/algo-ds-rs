pub fn heap_sort<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    // Build a max-heap by starting from the first non-leaf node.
    for i in (0..len / 2).rev() {
        sift_down(nums, i, len);
    }

    // Repeatedly extract the maximum element from the heap and rebuild the heap.
    for end in (1..len).rev() {
        nums.swap(0, end);
        sift_down(nums, 0, end);
    }
}

fn sift_down<T: PartialOrd>(nums: &mut [T], mut parent: usize, max: usize) {
    loop {
        let left_child = (parent << 1) + 1;
        let right_child = left_child + 1;
        let mut largest = parent;

        if left_child < max && nums[left_child] > nums[largest] {
            largest = left_child;
        }

        if right_child < max && nums[right_child] > nums[largest] {
            largest = right_child;
        }

        if largest == parent {
            break;
        }

        nums.swap(parent, largest);
        parent = largest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_array_heap_sort() {
        let mut nums = [4, 10, 3, 5, 1];
        heap_sort(&mut nums);
        assert_eq!(nums, [1, 3, 4, 5, 10]);
    }

    #[test]
    fn test_f64_vector_heap_sort() {
        let mut nums = vec![4.2, 10.1, 3.7, 5.8, 1.0];
        heap_sort(&mut nums);
        assert_eq!(nums, vec![1.0, 3.7, 4.2, 5.8, 10.1]);
    }

    #[test]
    fn test_char_array_heap_sort() {
        let mut nums = ['c', 'a', 'd', 'b'];
        heap_sort(&mut nums);
        assert_eq!(nums, ['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_empty_case_heap_sort() {
        let mut nums: Vec<i32> = Vec::new();
        heap_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }
}
