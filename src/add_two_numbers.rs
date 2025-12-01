#![allow(dead_code)]

fn solution(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l3: Option<Box<ListNode>> = None;
    let mut n1 = &l1;
    let mut n2 = &l2;
    let mut n3 = &mut l3;
    let mut has_remainder = false;

    loop {
        if n1.is_none() && n2.is_none() {
            if has_remainder && let Some(v3) = n3 {
                v3.next = Some(Box::new(ListNode::new(1)));
            }
            break;
        }

        let mut v3_next = Box::new(ListNode::new(if has_remainder { 1 } else { 0 }));

        if let Some(v1) = n1 {
            v3_next.val += v1.val;
            n1 = &v1.next;
        }

        if let Some(v2) = n2 {
            v3_next.val += v2.val;
            n2 = &v2.next;
        }

        has_remainder = v3_next.val > 9;
        v3_next.val %= 10;

        if let Some(v3) = n3 {
            v3.next = Some(v3_next);
            n3 = &mut v3.next;
        } else {
            l3 = Some(v3_next);
            n3 = &mut l3;
        }
    }

    l3
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        assert_eq!(
            solution(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            }))
        );

        assert_eq!(
            solution(
                Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode { val: 9, next: None }))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode { val: 1, next: None }))
                    }))
                }))
            }))
        )
    }
}
