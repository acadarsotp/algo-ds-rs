pub fn binary_search_iterative<T: PartialEq + PartialOrd>(nums: &[T], target: T) -> Option<usize> {
    if nums.len() == 0 {
        return None;
    }

    let mut low = 0;
    let mut mid = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        mid = (low + high) >> 1;

        if target == nums[mid] {
            found = true;
        } else if target < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    if found {
        Some(mid)
    } else {
        None
    }
}

pub fn binary_search_recursive<T: PartialEq + PartialOrd>(nums: &[T], target: T) -> Option<usize> {
    if nums.len() == 0 {
        return None;
    }

    let mid = nums.len() >> 1;

    if target == nums[mid] {
        Some(mid)
    } else if target < nums[mid] {
        binary_search_recursive(&nums[..mid], target)
    } else {
        binary_search_recursive(&nums[mid + 1..], target)
    }
}

pub fn binary_search_exponential<T: PartialEq + PartialOrd>(
    nums: &[T],
    target: T,
) -> Option<usize> {
    if nums.len() == 0 {
        return None;
    }

    let mut high = 1;
    while high < nums.len() && nums[high] < target {
        high <<= 1;
    }

    let low = high >> 1;
    let range = low..nums.len().min(high + 1);

    binary_search_iterative(&nums[range], target).map(|index| index + low)
}

//interpolation giving a lot of trouble when trying to implement it with generics

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_iterative_with_integers() {
        let nums = vec![1, 3, 5, 7, 9, 11, 13, 15];
        assert_eq!(binary_search_iterative(&nums, 7), Some(3));
        assert_eq!(binary_search_iterative(&nums, 1), Some(0));
        assert_eq!(binary_search_iterative(&nums, 15), Some(7));
        assert_eq!(binary_search_iterative(&nums, 6), None);
    }

    #[test]
    fn test_binary_search_iterative_with_strings() {
        let words = vec!["apple", "banana", "cherry", "date", "fig", "grape"];
        assert_eq!(binary_search_iterative(&words, "cherry"), Some(2));
        assert_eq!(binary_search_iterative(&words, "banana"), Some(1));
        assert_eq!(binary_search_iterative(&words, "plum"), None);
        assert_eq!(binary_search_iterative(&words, "kiwi"), None);
    }

    #[test]
    fn test_binary_search_iterative_with_empty_collection() {
        let nums: Vec<i32> = Vec::new();
        assert_eq!(binary_search_iterative(&nums, 3), None);
    }

    #[test]
    fn test_binary_search_recursive_with_integers() {
        let nums = vec![1, 3, 5, 7, 9, 11, 13, 15];
        assert_eq!(binary_search_iterative(&nums, 7), Some(3));
        assert_eq!(binary_search_iterative(&nums, 1), Some(0));
        assert_eq!(binary_search_iterative(&nums, 15), Some(7));
        assert_eq!(binary_search_iterative(&nums, 6), None);
    }

    #[test]
    fn test_binary_search_recursive_with_strings() {
        let words = vec!["apple", "banana", "cherry", "date", "fig", "grape"];
        assert_eq!(binary_search_iterative(&words, "cherry"), Some(2));
        assert_eq!(binary_search_iterative(&words, "banana"), Some(1));
        assert_eq!(binary_search_iterative(&words, "plum"), None);
        assert_eq!(binary_search_iterative(&words, "kiwi"), None);
    }

    #[test]
    fn test_binary_search_recursive_with_empty_collection() {
        let nums: Vec<i32> = Vec::new();
        assert_eq!(binary_search_iterative(&nums, 3), None);
    }

    #[test]
    fn test_binary_search_exponential_with_integers() {
        let nums = vec![1, 3, 5, 7, 9, 11, 13, 15];
        assert_eq!(binary_search_exponential(&nums, 7), Some(3));
        assert_eq!(binary_search_exponential(&nums, 1), Some(0));
        assert_eq!(binary_search_exponential(&nums, 15), Some(7));
        assert_eq!(binary_search_exponential(&nums, 6), None);
    }

    #[test]
    fn test_binary_search_exponential_with_strings() {
        let words = vec!["apple", "banana", "cherry", "date", "fig", "grape"];
        assert_eq!(binary_search_exponential(&words, "cherry"), Some(2));
        assert_eq!(binary_search_exponential(&words, "banana"), Some(1));
        assert_eq!(binary_search_exponential(&words, "plum"), None);
        assert_eq!(binary_search_exponential(&words, "kiwi"), None);
    }

    #[test]
    fn test_binary_search_exponential_with_empty_collection() {
        let nums: Vec<i32> = Vec::new();
        assert_eq!(binary_search_exponential(&nums, 3), None);
    }
}
