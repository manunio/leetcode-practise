use std::collections::HashMap;
use std::vec;

mod tests;

fn main() {
    println!("Leetcode practise session");
}

#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![];

        for i in 0..nums.len() {
            let val = nums[nums[i] as usize];
            answer.push(val);
        }

        answer
    }

    // [1,2,1] -> [1,2,1,1,2,1]
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0; nums.len() * 2];

        for (i, &ele) in nums.iter().enumerate() {
            answer[i] = ele;
            answer[i + nums.len()] = ele;
        }

        answer
    }

    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut ans = 0;

        for operation in operations.iter() {
            match operation.as_str() {
                "++X" | "X++" => ans += 1,
                "--X" | "X--" => ans -= 1,
                x => panic!("unknown operation {}", x),
            }
        }

        ans
    }

    pub fn final_value_after_operations_2(operations: Vec<String>) -> i32 {
        let mut ans = 0;

        for operation in operations {
            if operation.contains('+') {
                ans += 1;
            } else {
                ans -= 1;
            }
        }

        ans
    }

    // [1,2,3,4] -> [1,3,6,10]
    // [1, 1+2, 1+2+3, 1+2+3+4].
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();

        for i in 1..nums.len() {
            ans[i] += ans[i - 1];
        }

        ans
    }

    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max = 0;

        for sentence in sentences {
            let mut words_count = 0;

            for _ in sentence.split_whitespace() {
                words_count += 1;
            }

            if words_count > max {
                max = words_count;
            }
        }

        max
    }

    // [2,5,1,3,4,7], n = 3 ->  [2,3,5,4,1,7]
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());

        for i in 0..(nums.len() / 2) {
            ans.push(nums[i]);
            ans.push(nums[i + n as usize]);
        }

        ans
    }

    pub fn shuffle_with_fold(nums: Vec<i32>, n: i32) -> Vec<i32> {
        (0..n as usize).fold(Vec::new(), |mut acc, i| {
            acc.push(nums[i]);
            acc.push(nums[i + (n as usize)]);

            acc
        })
    }

    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for customers in accounts {
            let mut wealth = 0;
            for customer_wealth in customers {
                wealth += customer_wealth;
                if wealth > max {
                    max = wealth
                }
            }
        }

        max
    }

    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut res: Vec<bool> = Vec::new();

        let max_candies = candies.iter().max().unwrap();

        for v in candies.iter() {
            if v + extra_candies >= *max_candies {
                res.push(true);
            } else {
                res.push(false);
            }
        }

        res
    }

    // https://leetcode.com/problems/number-of-good-pairs/
    // [1, 2, 3, 1, 1, 3] -> 4
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut pairs = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    pairs += 1;
                }
            }
        }

        pairs
    }

    pub fn num_identical_pairs_1(nums: Vec<i32>) -> i32 {
        let mut pairs = 0;
        let mut map = HashMap::new();

        for i in nums {
            let i_pair = *map.get(&i).unwrap_or(&0);
            map.insert(i, i_pair + 1);
            pairs += i_pair;
        }

        pairs
    }

    // https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
    // [8,1,2,2,3] -> [4,0,1,1,3]
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut small_numbers = vec![0; nums.len()];

        for i in 0..nums.len() {
            let mut count = 0;
            for j in 0..nums.len() {
                if i != j && nums[i] > nums[j] {
                    count += 1;
                }
            }
            small_numbers[i] = count;
        }

        small_numbers
    }

    // https://leetcode.com/problems/shuffle-string/
    //Input: s = "codeleet", indices = [4,5,6,7,0,2,1,3]
    // Output: "leetcode"
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut v = Vec::<char>::new();
        v.resize(s.len(), 'a');

        for (i, iter) in indices.iter().enumerate() {
            v[*iter as usize] = s.chars().nth(i).unwrap();
        }

        let r = v.iter().collect();

        return r;
    }
}
