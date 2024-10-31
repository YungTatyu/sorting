pub fn selection_sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        let mut min_index = i;
        for j in i + 1..v.len() {
            if v[j] < v[min_index] {
                min_index = j;
            }
        }
        v.swap(i, min_index);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn crate_actual_and_expect(mut v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut expect = v.clone();
        selection_sort(&mut v);
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
    selection_sort(&mut v);
    println!("{:?}", v);
}
