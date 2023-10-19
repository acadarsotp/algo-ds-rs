pub fn sequential_search<T: PartialEq + Copy>(nums: &[T], target: T) -> Option<usize> {
    nums.iter().position(|&x| x == target)
}

pub fn ordered_sequential_search<T: PartialEq + PartialOrd + Copy>(
    nums: &[T],
    target: T,
) -> Option<usize> {
    nums.iter().enumerate().find_map(|(index, &x)| {
        if x == target {
            return Some(index);
        } else if x > target {
            return None;
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_search_with_arrays() {
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(sequential_search(&nums, 3), Some(2));
        assert_eq!(sequential_search(&nums, 6), None);
    }

    #[test]
    fn test_sequential_search_with_vectors() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(sequential_search(&nums, 3), Some(2));
        assert_eq!(sequential_search(&nums, 6), None);
    }

    #[test]
    fn test_sequential_search_with_empty_collection() {
        let nums: Vec<i32> = Vec::new();
        assert_eq!(sequential_search(&nums, 3), None);
    }

    #[test]
    fn test_sequential_search_with_strings() {
        let words = ["apple", "banana", "cherry"];
        assert_eq!(sequential_search(&words, "banana"), Some(1));
        assert_eq!(sequential_search(&words, "date"), None);
    }

    #[test]
    fn test_ordered_sequential_search_with_arrays() {
        let nums = [1, 3, 5, 7, 9];
        assert_eq!(ordered_sequential_search(&nums, 5), Some(2));
        assert_eq!(ordered_sequential_search(&nums, 6), None);
        assert_eq!(ordered_sequential_search(&nums, 0), None);
        assert_eq!(ordered_sequential_search(&nums, 10), None);
    }

    #[test]
    fn test_ordered_sequential_search_with_vectors() {
        let nums = vec![1, 3, 5, 7, 9];
        assert_eq!(ordered_sequential_search(&nums, 5), Some(2));
        assert_eq!(ordered_sequential_search(&nums, 6), None);
        assert_eq!(ordered_sequential_search(&nums, 0), None);
        assert_eq!(ordered_sequential_search(&nums, 10), None);
    }

    #[test]
    fn test_ordered_sequential_search_with_empty_collection() {
        let nums: Vec<i32> = Vec::new();
        assert_eq!(ordered_sequential_search(&nums, 3), None);
    }

    #[test]
    fn test_ordered_sequential_search_with_strings() {
        let words = ["apple", "banana", "cherry", "date", "fig"];
        assert_eq!(ordered_sequential_search(&words, "banana"), Some(1));
        assert_eq!(ordered_sequential_search(&words, "cherry"), Some(2));
        assert_eq!(ordered_sequential_search(&words, "blueberry"), None);
    }
}
