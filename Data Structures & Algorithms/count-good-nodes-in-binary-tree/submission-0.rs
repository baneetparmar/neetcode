// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32, max: i32) {
            match node {
                None => {}
                Some(n) => {
                    let val = n.borrow().val;
                    let new_max = max.max(val);

                    if val >= max {
                        *res += 1;
                    }

                    dfs(n.borrow().left.clone(), res, new_max);
                    dfs(n.borrow().right.clone(), res, new_max);
                }
            }
        }

        let mut res = 0;
        let max = root.as_ref().unwrap().borrow().val;
        dfs(root, &mut res, max);
        res
    }
}
