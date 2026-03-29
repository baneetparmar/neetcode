impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        loop {
            let mut minimum = None;
            let mut no_more_nodes = true;
            for i in 0..lists.len() {
                if let Some(Some(node)) = lists.get(i) {
                    no_more_nodes  = false;
                    if minimum == None {
                        minimum = Some(i);
                    }
                    if let Some(Some(cur_min)) = lists.get(minimum.unwrap()) {
                        if node.val < cur_min.val {
                            minimum = Some(i);
                        }
                    };
                }
            }
            if no_more_nodes {
                break;
            }

            let idx = minimum.unwrap();

            let mut node = lists[idx].take().unwrap();
            lists[idx] = node.next.take();

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }

}
