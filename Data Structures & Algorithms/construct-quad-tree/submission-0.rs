use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        fn builder(grid: &Vec<Vec<i32>>, row: usize, col: usize, size: usize) -> Option<Rc<RefCell<Node>>> {
            let first = grid[row][col];
            let all_same = (row..row + size)
                .all(|r| (col..col + size).all(|c| grid[r][c] == first));

            if all_same {
                // leaf node
                return Some(Rc::new(RefCell::new(Node::new(first == 1, true))));
            }

            let half = size / 2;
            let node = Node {
                val: true,
                is_leaf: false,
                top_left: builder(grid, row, col, half),
                top_right: builder(grid, row, col + half, half),
                bottom_left: builder(grid, row + half, col, half),
                bottom_right: builder(grid, row + half, col + half, half),
            };

            Some(Rc::new(RefCell::new(node)))
        }

        let size = grid.len();
        builder(&grid, 0, 0, size)
    }
}

