use core::num;
use std::vec;

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
                x => panic!("unknown operation {}", x)
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

            if words_count > max { max = words_count; }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_build_array() {
        let nums = vec![0, 2, 1, 5, 3, 4];
        let required = vec![0, 1, 2, 4, 5, 3];
        let expected = Solution::build_array(nums);
        assert_eq!(required, expected);
    }

    #[test]
    fn test_get_concatination() {
        let nums = vec![1, 2, 1];
        assert_eq!(vec![1, 2, 1, 1, 2, 1], Solution::get_concatenation(nums));
    }

    #[test]
    fn test_final_value_after_operations() {
        let operations = vec![
            "--X".to_string(),
            "X++".to_string(),
            "X++".to_string()
        ];
        assert_eq!(1, Solution::final_value_after_operations(operations));
    }

    #[test]
    fn test_final_value_after_operations_2() {
        let operations = vec![
            "--X".to_string(),
            "X++".to_string(),
            "X++".to_string()
        ];
        assert_eq!(1, Solution::final_value_after_operations_2(operations));
    }

    #[test]
    fn test_running_sum() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(nums));
    }

    #[test]
    fn test_most_words_found() {
        let mut sentences = vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string()
        ];
        assert_eq!(6, Solution::most_words_found(sentences));

        sentences = vec![
            "w jrpihe zsyqn l dxchifbxlasaehj".to_string(),
            "nmmfrwyl jscqyxk a xfibiooix xolyqfdspkliyejsnksfewbjom".to_string(),
            "xnleojowaxwpyogyrayfgyuzhgtdzrsyococuqexggigtberizdzlyrdsfvryiynhg".to_string(),
            "krpwiazoulcixkkeyogizvicdkbrsiiuhizhkxdpssynfzuigvcbovm".to_string(),
            "rgmz rgztiup wqnvbucfqcyjivvoeedyxvjsmtqwpqpxmzdupfyfeewxegrlbjtsjkusyektigr".to_string(),
            "o lgsbechr lqcgfiat pkqdutzrq iveyv iqzgvyddyoqqmqerbmkxlbtmdtkinlk".to_string(),
            "hrvh efqvjilibdqxjlpmanmogiossjyxepotezo".to_string(),
            "qstd zui nbbohtuk".to_string(),
            "qsdrerdzjvhxjqchvuewevyzlkyydpeeblpc".to_string(),
        ];
        assert_eq!(6, Solution::most_words_found(sentences));
    }
}
