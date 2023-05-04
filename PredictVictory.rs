use std::collections::VecDeque;
pub struct Solution {}
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let (mut radiant, mut dire) = senate.chars().enumerate().fold((VecDeque::new(), VecDeque::new()), |(mut r, mut d), (i, c)| {
            if c == 'R' {
                r.push_back(i);
            } else {
                d.push_back(i);
            }

            (r, d)
        });

        while let (Some(&r), Some(&d)) = (radiant.front(), dire.front()) {
            radiant.pop_front();
            dire.pop_front();

            if r < d {
                radiant.push_back(r + n);
            } else {
                dire.push_back(d + n);
            }
        }

        if radiant.is_empty() { "Dire".to_string() } else { "Radiant".to_string() }
    }
}



fn main() {
    let senate = "RD".to_string();
    let ret = Solution::predict_party_victory(senate);
    println!("ret={}", ret);
}