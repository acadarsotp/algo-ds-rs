pub fn merge_sort<T: PartialOrd + Copy>(nums: &mut [T]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        // sort the first half
        merge_sort(&mut nums[..mid]);
        // sort the last half
        merge_sort(&mut nums[mid..]);
        // merge all
        merge(nums, mid);
    }
}

fn merge<T: PartialOrd + Copy>(nums: &mut [T], mid: usize) {
    // mark element in first half of data
    let mut i = 0;
    // mark element in last half of data
    let mut k = mid;
    let mut temp = Vec::new();

    for _j in 0..nums.len() {
        if k == nums.len() || i == mid {
            break;
        }

        // put into a temp collection
        if nums[i] < nums[k] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[k]);
            k += 1;
        }
    }

    // to make sure all data been solved
    if i < mid && k == nums.len() {
        for j in i..mid {
            temp.push(nums[j]);
        }
    } else if i == mid && k < nums.len() {
        for j in k..nums.len() {
            temp.push(nums[j]);
        }
    }

    // put temp data back to nums, finish sort
    for j in 0..nums.len() {
        nums[j] = temp[j];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_vec_i32() {
        let mut nums = vec![5, 3, 8, 1, 2];
        merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_merge_sort_array_i32() {
        let mut nums = [5, 3, 8, 1, 2];
        merge_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_merge_sort_i32() {
        let mut nums = vec![5, 3, 8, 1, 2];
        merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_merge_sort_f64() {
        let mut nums = vec![5.5, 3.3, 8.8, 1.1, 2.2];
        merge_sort(&mut nums);
        assert_eq!(nums, vec![1.1, 2.2, 3.3, 5.5, 8.8]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut nums = Vec::<i32>::new();
        merge_sort(&mut nums);
        assert_eq!(nums, Vec::<i32>::new());
    }
}
