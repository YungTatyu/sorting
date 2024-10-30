fn selection_sort(v: &mut Vec<i64>) {
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

fn main() {
    let mut v: Vec<i64> = vec![3, 1, 9, 0, 4];
    println!("{:?}", v);
    selection_sort(&mut v);
    println!("{:?}", v);
}
