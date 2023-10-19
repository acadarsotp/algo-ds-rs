pub fn bubble_sort<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    for _ in 0..len {
        let mut compare = false;
        for j in 0..len - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                compare = true;
            }
        }
        if !compare {
            break;
        }
    }
}

//Linus torvalds would be disgusted with all the nesting
pub fn cocktail_sort<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let mut bubble = true;

    for i in 0..len / 2 {
        if bubble {
            bubble = false;
            for j in i..len - i - 1 {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }
            for j in (i + 1..len - i - 1).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j - 1);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    }
}

pub fn comb_sort<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let mut gap = len;
    let shrink_factor = 1.3;
    let mut swapped;

    loop {
        gap = (gap as f64 / shrink_factor) as usize;
        if gap <= 1 {
            gap = 1;
            swapped = false;
        } else {
            swapped = true;
        }

        let mut i = 0;
        while i + gap < len {
            if nums[i] > nums[i + gap] {
                nums.swap(i, i + gap);
                swapped = true;
            }
            i += 1;
        }

        if gap == 1 && !swapped {
            break;
        }
    }
}

pub fn cant_believe_it_can_sort_asc<T: PartialOrd>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

pub fn cant_believe_it_can_sort_desc<T: PartialOrd>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_integers() {
        let mut nums = [5, 2, 1, 3, 6];
        bubble_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 5, 6]);
    }

    #[test]
    fn test_bubble_sort_floats() {
        let mut nums = [2.2, 3.3, 1.1, 5.5, 4.4];
        bubble_sort(&mut nums);
        assert_eq!(nums, [1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    fn test_bubble_sort_chars() {
        let mut nums = ['c', 'a', 'b'];
        bubble_sort(&mut nums);
        assert_eq!(nums, ['a', 'b', 'c']);
    }

    #[test]
    fn test_bubble_sort_empty() {
        let mut nums: [i32; 0] = [];
        bubble_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_cocktail_sort_integers() {
        let mut nums = [5, 2, 1, 3, 6];
        cocktail_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 5, 6]);
    }

    #[test]
    fn test_cocktail_sort_floats() {
        let mut nums = [2.2, 3.3, 1.1, 5.5, 4.4];
        cocktail_sort(&mut nums);
        assert_eq!(nums, [1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    fn test_cocktail_sort_chars() {
        let mut nums = ['c', 'a', 'b'];
        cocktail_sort(&mut nums);
        assert_eq!(nums, ['a', 'b', 'c']);
    }

    #[test]
    fn test_cocktail_sort_empty() {
        let mut nums: [i32; 0] = [];
        cocktail_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_comb_sort_integers() {
        let mut nums = [5, 2, 1, 3, 6];
        comb_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 5, 6]);
    }

    #[test]
    fn test_comb_sort_floats() {
        let mut nums = [2.2, 3.3, 1.1, 5.5, 4.4];
        comb_sort(&mut nums);
        assert_eq!(nums, [1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    fn test_comb_sort_chars() {
        let mut nums = ['c', 'a', 'b'];
        comb_sort(&mut nums);
        assert_eq!(nums, ['a', 'b', 'c']);
    }

    #[test]
    fn test_comb_sort_empty() {
        let mut nums: [i32; 0] = [];
        comb_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_cant_believe_it_can_sort_asc_integers() {
        let mut nums = [5, 2, 1, 3, 6];
        cant_believe_it_can_sort_asc(&mut nums);
        assert_eq!(nums, [1, 2, 3, 5, 6]);
    }

    #[test]
    fn test_cant_believe_it_can_sort_asc_floats() {
        let mut nums = [2.2, 3.3, 1.1, 5.5, 4.4];
        cant_believe_it_can_sort_asc(&mut nums);
        assert_eq!(nums, [1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    fn test_cant_believe_it_can_sort_asc_chars() {
        let mut nums = ['c', 'a', 'b'];
        cant_believe_it_can_sort_asc(&mut nums);
        assert_eq!(nums, ['a', 'b', 'c']);
    }

    #[test]
    fn test_cant_believe_it_can_sort_asc_empty() {
        let mut nums: [i32; 0] = [];
        cant_believe_it_can_sort_asc(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_cant_believe_it_can_sort_desc_integers() {
        let mut nums = [5, 2, 1, 3, 6];
        cant_believe_it_can_sort_desc(&mut nums);
        assert_eq!(nums, [6, 5, 3, 2, 1]);
    }

    #[test]
    fn test_cant_believe_it_can_sort_desc_floats() {
        let mut nums = [2.2, 3.3, 1.1, 5.5, 4.4];
        cant_believe_it_can_sort_desc(&mut nums);
        assert_eq!(nums, [5.5, 4.4, 3.3, 2.2, 1.1]);
    }

    #[test]
    fn test_cant_believe_it_can_sort_desc_chars() {
        let mut nums = ['c', 'a', 'b'];
        cant_believe_it_can_sort_desc(&mut nums);
        assert_eq!(nums, ['c', 'b', 'a']);
    }

    #[test]
    fn test_cant_believe_it_can_sort_desc_empty() {
        let mut nums: [i32; 0] = [];
        cant_believe_it_can_sort_desc(&mut nums);
        assert_eq!(nums, []);
    }
}
