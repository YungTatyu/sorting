pub fn heap_sort(v: &mut Vec<i32>) {
    if v.len() <= 1 {
        return;
    }
    // heapデータ構造を作成
    // 配列の長さの半分から、max_heapifyを呼べば、heapデータ構造が完成する
    for i in (0..(v.len() / 2)).rev() {
        max_heapify(v, v.len(), i);
    }
    for i in (1..v.len()).rev() {
        v.swap(i, 0);
        // sortされていないheapから一番大きい値をrootにもってくる
        // 同時に子供ノードもsortする
        max_heapify(v, i, 0);
    }
}

pub fn max_heapify(v: &mut Vec<i32>, size: usize, index: usize) {
    let left_child = index * 2 + 1;
    let right_child = index * 2 + 2;
    let mut largest = index;
    if left_child < size && v[left_child] > v[largest] {
        largest = left_child;
    }
    if right_child < size && v[right_child] > v[largest] {
        largest = right_child;
    }
    if index != largest {
        v.swap(index, largest);
        // 再帰的に子供のnodeをソートする
        max_heapify(v, size, largest);
    }
}

pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = nums.clone();
    heap_sort(&mut ret);
    ret
}

#[cfg(test)]
mod heap_sort_test {
    use super::*;

    fn crate_actual_and_expect(mut v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut expect = v.clone();
        heap_sort(&mut v);
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
    heap_sort(&mut v);
    println!("{:?}", v);
}
