pub fn bubble_sort<T: PartialOrd>(nums: &mut [T]) {
    if nums.len() <= 1 {
        return;
    }

    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                compare = true;
            }
        }
        len -= 1;
    }
}

//Linus torvalds would be disgusted with all the nesting
pub fn cocktail_sort<T: PartialOrd>(nums: &mut [T]) {
    if nums.len() <= 1 {
        return;
    }

    let mut bubble = true;
    let len = nums.len();

    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;
            //Left to right
            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }
            //Right to left
            for j in (i + 1..=(len - i - 1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j - 1, j);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    }
}

pub fn comb_sort<T: PartialOrd>(nums: &mut [T]) {
    if nums.len() <= 1 {
        return;
    }

    let mut i;
    let mut gap = nums.len();

    // ordered basically
    while gap > 0 {
        gap = (gap as f64 * 0.8) as usize;
        i = gap;
        while i < nums.len() {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
            i += 1;
        }
    }

    // rearrange the elements properly.
    // exchange controls the process
    let mut exchange = true;
    while exchange {
        exchange = false;
        i = 0;
        while i < nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                exchange = true;
            }
            i += 1;
        }
    }
}

pub fn cant_believe_it_can_sort_asc<T: PartialOrd>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

pub fn cant_believe_it_can_sort_desc<T: PartialOrd>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] > nums[j] {
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
