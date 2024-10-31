pub fn bubble_sort(v: &mut Vec<i32>) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expect = heights.clone();
    bubble_sort(&mut expect);
    let mut count = 0;
    for i in 0..heights.len() {
        if heights[i] != expect[i] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod bubble_sort_test {
    use super::*;

    fn crate_actual_and_expect(mut v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut expect = v.clone();
        bubble_sort(&mut v);
        expect.sort();
        (v, expect)
    }

    #[test]
    fn small_range() {
        let (actual, expect) = crate_actual_and_expect(vec![4, 3, 9, 8, 2]);
        assert_eq!(actual, expect)
    }

    #[test]
    fn big_range() {
        let (actual, expect) = crate_actual_and_expect(vec![-4, 32034243, 239, -32348, 3204823, 0]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn duplicate_nums() {
        let (actual, expect) = crate_actual_and_expect(vec![2, -1, 0, 7, 2, 7]);
        assert_eq!(actual, expect);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![3, 1, 9, 0, 4];
    println!("{:?}", v);
    bubble_sort(&mut v);
    println!("{:?}", v);
}
