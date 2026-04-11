use std::collections::{HashMap, HashSet, VecDeque};

struct Twitter {
    tweets: VecDeque<(i32, i32)>,
    followers: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    pub fn new() -> Self {
        Self {
            tweets: VecDeque::new(),
            followers: HashMap::new(),
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push_front((user_id, tweet_id));
        self.followers.entry(user_id).or_insert(HashSet::new());
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let empty = HashSet::new();
        let following = self.followers.get(&user_id).unwrap_or(&empty);
        let mut output = vec![];

        for &(id, tweet_id) in self.tweets.iter() {
            if id == user_id || following.contains(&id) {
                output.push(tweet_id);
                if output.len() == 10 {
                    break;
                }
            }
        }
        output
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.followers
            .entry(follower_id)
            .or_insert(HashSet::new())
            .insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.followers.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}
