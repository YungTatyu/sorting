use std::usize;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut clone = nums.clone();
    let target_index = nums.len() - k as usize;
    quick_select(&mut clone, 0, nums.len() - 1, target_index)
}

pub fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
    if left == right {
        return nums[left];
    }
    let pivot = partition(nums, left, right);
    if pivot + 1 == k {
        nums[k]
    } else if pivot + 1 > k {
        quick_select(nums, left, pivot - 1, k)
    } else {
        quick_select(nums, pivot + 1, right, k)
    }
}

pub fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut small_index = left;
    for i in left..right {
        if pivot > nums[i] {
            nums.swap(i, small_index);
            small_index += 1;
        }
    }
    nums.swap(small_index, right);
    small_index
}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn fkl_small_range() {
        assert_eq!(find_kth_largest(vec![4, 3, 9, 8, 2], 3), 3);
    }

    #[test]
    fn fkl_big_range() {
        assert_eq!(
            find_kth_largest(vec![-4, 10000, 239, -32348, 1000, 1], 2),
            1000
        );
    }

    #[test]
    fn fkl_duplicate_nums() {
        assert_eq!(find_kth_largest(vec![2, -1, 0, 7, 2, 7], 1), 7);
    }
}

fn main() {
    let v: Vec<i32> = vec![4, 3, 9, 8, 2];
    println!("{:?}", v);
    println!("{}", find_kth_largest(v, 3));
}
