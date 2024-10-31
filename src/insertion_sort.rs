/**
 * @brief 小さい値を左から順に場所を確定させていく
 */
pub fn insertion_sort(v: &mut Vec<i32>) {
    for i in 1..v.len() {
        let mut cur_index = i;
        while cur_index > 0 {
            if v[cur_index] > v[cur_index - 1] {
                break;
            }
            v.swap(cur_index, cur_index - 1);
            cur_index -= 1;
        }
    }
}

#[cfg(test)]
mod insertion_sort_test {
    use super::*;

    fn crate_actual_and_expect(mut v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut expect = v.clone();
        insertion_sort(&mut v);
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
    insertion_sort(&mut v);
    println!("{:?}", v);
}
