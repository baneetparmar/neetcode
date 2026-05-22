impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n: usize = accounts.iter().map(|a| a.len() - 1).sum();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank = vec![0usize; n];
        let mut email_to_idx: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let mut email_to_name: std::collections::HashMap<usize, &str> = std::collections::HashMap::new();

        let mut idx = 0;
        for account in &accounts {
            let name = &account[0];
            let mut first = None;
            for email in &account[1..] {
                let i = *email_to_idx.entry(email).or_insert_with(|| {
                    let i = idx;
                    idx += 1;
                    i
                });
                email_to_name.entry(i).or_insert(name);
                if let Some(f) = first {
                    Self::union(&mut parent, &mut rank, f, i);
                } else {
                    first = Some(i);
                }
            }
        }

        let mut groups: std::collections::HashMap<usize, Vec<&str>> = std::collections::HashMap::new();
        for (&email, &i) in &email_to_idx {
            let root = Self::find(&mut parent, i);
            groups.entry(root).or_default().push(email);
        }

        let mut res = vec![];
        for (root, mut emails) in groups {
            emails.sort();
            let name = email_to_name[&Self::find(&mut parent, root)];
            let mut account = vec![name.to_string()];
            account.extend(emails.into_iter().map(|e| e.to_string()));
            res.push(account);
        }
        res
    }

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x { parent[x] = Self::find(parent, parent[x]); }
        parent[x]
    }

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
        let (rx, ry) = (Self::find(parent, x), Self::find(parent, y));
        if rx == ry { return; }
        if rank[rx] < rank[ry] { parent[rx] = ry; }
        else if rank[rx] > rank[ry] { parent[ry] = rx; }
        else { parent[ry] = rx; rank[rx] += 1; }
    }
}