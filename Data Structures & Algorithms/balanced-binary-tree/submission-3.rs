use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut balanced = true;
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, balanced:&mut bool) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();

                    let left_height =  dfs(left,balanced);
                    let right_height = dfs(right,balanced);
                    if (left_height - right_height).abs() > 1 {
                        *balanced = false;
                    }
                    1 + left_height.max(right_height)
                }
            }
        }
        dfs(root, &mut balanced);
        balanced
    }
}
