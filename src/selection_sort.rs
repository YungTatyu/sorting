pub fn selection_sort(v: &mut Vec<i64>) {
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

    fn crate_actual_and_expect(mut v: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
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
}

fn main() {
    let mut v: Vec<i64> = vec![3, 1, 9, 0, 4];
    println!("{:?}", v);
    selection_sort(&mut v);
    println!("{:?}", v);
}
