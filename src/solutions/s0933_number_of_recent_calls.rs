pub struct RecentCounter {
    pub requests: Vec<i32>,
}

impl RecentCounter {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);

        let mut inside_counter: i32 = 0;
        for r in self.requests.iter().rev() {
            if t - r <= 3000 {
                inside_counter += 1;
            } else {
                break;
            }
        }

        inside_counter
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut recent_counter = RecentCounter::new();
        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
