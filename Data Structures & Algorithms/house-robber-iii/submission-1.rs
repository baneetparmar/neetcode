use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn rob(root: T) -> i32 {
        let mut cache = HashMap::new();
        fn dfs(n: &T, cache:&mut HashMap<*const RefCell<TreeNode>, i32>) -> i32 {
            if let Some(node) = n {
                let key = Rc::as_ptr(node);
                if let Some(&val) = cache.get(&key) {
                    return val;
                }

                let node = node.borrow();
                let mut res = node.val;

                if let Some(ref left) = node.left {
                    let left = left.borrow();
                    res += dfs(&left.left, cache) + dfs(&left.right, cache);
                };
                if let Some(ref right) = node.right {
                    let right = right.borrow();
                    res += dfs(&right.left, cache) + dfs(&right.right, cache);
                };

                res = res.max(dfs(&node.left, cache) + dfs(&node.right, cache));
                cache.insert(key,res);
                res
            } else {
                0
            }
        }
        dfs(&root, &mut cache)
    }
}
