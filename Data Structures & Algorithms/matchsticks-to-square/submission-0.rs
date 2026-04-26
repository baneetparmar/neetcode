impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 { return false; }
        let side = (sum / 4) as i32;
        let mut matchsticks = matchsticks;
        matchsticks.sort_by(|a, b| b.cmp(a));
        let mut sides = [0i32; 4];

        fn backtrack(sticks: &[i32], sides: &mut [i32; 4], idx: usize, side: i32) -> bool {
            if idx == sticks.len() {
                return sides[0] == side && sides[1] == side && sides[2] == side;
            }
            let mut seen = std::collections::HashSet::new();
            for i in 0..4 {
                if sides[i] + sticks[idx] > side { continue; }
                if !seen.insert(sides[i]) { continue; } 
                sides[i] += sticks[idx];
                if backtrack(sticks, sides, idx + 1, side) { return true; }
                sides[i] -= sticks[idx];
            }
            false
        }

        backtrack(&matchsticks, &mut sides, 0, side)
    }
}