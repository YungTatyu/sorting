use std::usize;

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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn count_size(head: Option<Box<ListNode>>) -> usize {
        let mut cnt: usize = 0;
        let mut cur_node = head;
        while let Some(node) = cur_node {
            cur_node = node.next;
            cnt += 1;
        }
        cnt
    }
}

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut cur_node = head;
    while let Some(mut node) = cur_node {
        cur_node = node.next.take();
        let mut prev = &mut dummy;
        while let Some(ref next) = prev.next {
            if next.val > node.val {
                break;
            }
            prev = prev.next.as_mut().unwrap();
        }
        node.next = prev.next.take();
        prev.next = Some(node);
    }
    dummy.next
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
