// https://leetcode.com/problems/best-time-to-buy-and-sell-stock

fn main() {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;

        if prices.len() < 2 {
            return 0;
        }

        let mut min1 = &prices[0];
        let mut up = &prices[1];
        let mut min2 = up;

        if prices.len() >= 3 {
            for i in &prices[2..] {
                let cmax = up - min1;
                let i2 = i - min2;
                let i1 = i - min1;
                // if cmax > max(i1,i2) {
                //
                // }
                // else
                if i1 >= max(cmax,i2) {
                    up = i;
                } else if i2 >= max(cmax,i1) {
                    min1 = min2;
                    up = i;
                }
                if i < min2 || up == i {
                    min2 = i;
                }
            }
        }


        return max(0, up - min1);
    }

    // println!("res {}", max_profit(vec!(7, 1)));
    // println!("res {}", max_profit(vec!(7, 1, 5, 3, 6, 4)));
    println!("res {}", max_profit(vec!(7, 1, 3, 6, 4)));
    println!("res {}", max_profit(vec![7,6,4,3,1]));
}

