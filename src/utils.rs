pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    todo!();
}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn fkl_small_range() {
        assert_eq!(find_kth_largest(vec![4, 3, 9, 8, 2], 4), 3);
    }

    #[test]
    fn fkl_big_range() {
        assert_eq!(
            find_kth_largest(vec![-4, 32034243, 239, -32348, 3204823, 0], 2),
            3204823
        );
    }

    #[test]
    fn fkl_duplicate_nums() {
        assert_eq!(find_kth_largest(vec![2, -1, 0, 7, 2, 7], 0), -1);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![3, 1, 9, 0, 4];
    println!("{:?}", v);
    println!("{}", find_kth_largest(v, 0));
}
