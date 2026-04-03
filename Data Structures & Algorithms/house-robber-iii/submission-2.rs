use std::rc::Rc;
use std::cell::RefCell;

type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn rob(root: T) -> i32 {
        let (rob, not_rob) = Self::dfs(root);
        rob.max(not_rob)
    }

    fn dfs(node: T) -> (i32, i32) {
        if let Some(n) = node {
            let (left_rob, left_not) = Self::dfs(n.borrow().left.clone());
            let (right_rob, right_not) = Self::dfs(n.borrow().right.clone());

            let rob = n.borrow().val + left_not + right_not;
            let not_rob = left_rob.max(left_not) + right_rob.max(right_not);

            (rob, not_rob)
        } else {
            (0, 0)
        }
    }
}