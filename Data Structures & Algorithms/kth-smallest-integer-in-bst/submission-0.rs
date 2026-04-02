use std::cell::RefCell;
use std::rc::Rc;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn kth_smallest(root: T, k: i32) -> i32 {
        fn dfs(node: T, v: &mut Vec<i32>) {
            if let Some(n) = node {
                dfs(n.borrow().left.clone(), v);
                let val = n.borrow().val;
                v.push(val);
                dfs(n.borrow().right.clone(), v);
            }
        }

        let mut values = vec![];
        dfs(root, &mut values);
        values[(k - 1) as usize]
    }
}
