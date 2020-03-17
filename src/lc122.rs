// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

fn main() {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;

        if prices.len() < 2 {
            return 0;
        }

        let mut prof = 0;

        let mut v = &prices[0];
        let mut le = v;
        for i in &prices[1..] {
            // let cp = i - v;
            if i < le {
                prof += le - v;
                v = i;
            }
            le = i;
        }

        prof += le - v;
        return prof;
    }

    // println!("res {}", max_profit(vec!(7, 1)));
    println!("res {}", max_profit(vec!(7, 1, 5, 3, 6, 4)));
    println!("res {}", max_profit(vec!(7, 1, 3, 6, 4)));
    println!("res {}", max_profit(vec![7, 6, 4, 3, 1]));
    println!("res {}", max_profit(vec![1,2,3,4,5]));
}

