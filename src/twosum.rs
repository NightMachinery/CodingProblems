// https://leetcode.com/problems/two-sum/



fn main() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        use std::convert::TryInto;

        let mut v2i = HashMap::with_capacity(nums.len());

        for (i, v) in nums.iter().enumerate() {
            // println!("{}", v)
            match v2i.get(&(target - v)) {
                Some(&number) => return vec!(number, i as i32),
                _ => {
                    v2i.insert(v, i as i32);
                },
            }
        }
        return vec!();
    }

    println!("{:?}", two_sum(vec!(2, 5, 1, 99, 3, 1, 1), 8));
}
