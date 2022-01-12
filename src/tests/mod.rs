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
    fn test_get_concatenation() {
        let nums = vec![1, 2, 1];
        assert_eq!(vec![1, 2, 1, 1, 2, 1], Solution::get_concatenation(nums));
    }

    #[test]
    fn test_final_value_after_operations() {
        let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
        assert_eq!(1, Solution::final_value_after_operations(operations));
    }

    #[test]
    fn test_final_value_after_operations_2() {
        let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
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
            "this is great thanks very much".to_string(),
        ];
        assert_eq!(6, Solution::most_words_found(sentences));

        sentences = vec![
            "w jrpihe zsyqn l dxchifbxlasaehj".to_string(),
            "nmmfrwyl jscqyxk a xfibiooix xolyqfdspkliyejsnksfewbjom".to_string(),
            "xnleojowaxwpyogyrayfgyuzhgtdzrsyococuqexggigtberizdzlyrdsfvryiynhg".to_string(),
            "krpwiazoulcixkkeyogizvicdkbrsiiuhizhkxdpssynfzuigvcbovm".to_string(),
            "rgmz rgztiup wqnvbucfqcyjivvoeedyxvjsmtqwpqpxmzdupfyfeewxegrlbjtsjkusyektigr"
                .to_string(),
            "o lgsbechr lqcgfiat pkqdutzrq iveyv iqzgvyddyoqqmqerbmkxlbtmdtkinlk".to_string(),
            "hrvh efqvjilibdqxjlpmanmogiossjyxepotezo".to_string(),
            "qstd zui nbbohtuk".to_string(),
            "qsdrerdzjvhxjqchvuewevyzlkyydpeeblpc".to_string(),
        ];
        assert_eq!(6, Solution::most_words_found(sentences));
    }

    #[test]
    fn test_shuffle() {
        let mut nums = vec![2, 5, 1, 3, 4, 7];
        assert_eq!(vec![2, 3, 5, 4, 1, 7], Solution::shuffle(nums, 3));

        nums = vec![2, 5, 1, 3, 4, 7];
        assert_eq!(vec![2, 3, 5, 4, 1, 7], Solution::shuffle_with_fold(nums, 3));
    }

    #[test]
    fn test_maximum_wealth() {
        let accounts = vec![
            // 0  1  2
            vec![1, 2, 3], // customer 0
            vec![3, 2, 1], // customer 1
        ];
        assert_eq!(6, Solution::maximum_wealth(accounts));
    }

    #[test]
    fn test_kids_with_candies() {
        let candies = vec![2, 3, 5, 1, 3];
        assert_eq!(
            vec![true, true, true, false, true],
            Solution::kids_with_candies(candies, 3)
        );
    }

    #[test]
    fn test_num_identical_pairs() {
        let mut nums = vec![1, 2, 3, 1, 1, 3];
        assert_eq!(4, Solution::num_identical_pairs(nums));
        nums = vec![1, 2, 3, 1, 1, 3];
        assert_eq!(4, Solution::num_identical_pairs_1(nums));
    }

    #[test]
    fn test_smaller_numbers_than_current() {
        let nums = vec![8, 1, 2, 2, 3];
        assert_eq!(
            vec![4, 0, 1, 1, 3],
            Solution::smaller_numbers_than_current(nums)
        );
    }

    #[test]
    fn test_restore_string() {
        let s = "codeleet".to_string();
        let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];
        assert_eq!("leetcode".to_string(), Solution::restore_string(s, indices));
    }

    #[test]
    fn test_decode() {
        let encoded = vec![1, 2, 3];
        let first = 1;
        assert_eq!(vec![1, 0, 2, 1], Solution::decode(encoded, first))
    }

    #[test]
    fn test_decompress_rl_elist() {
        let mut nums = vec![1, 2, 3, 4];
        assert_eq!(vec![2, 4, 4, 4], Solution::decompress_rl_elist(nums));
        nums = vec![1, 2, 3, 4];
        assert_eq!(vec![2, 4, 4, 4], Solution::decompress_rl_elist_1(nums));
    }

    #[test]
    fn test_create_target_array() {
        let mut nums = vec![0, 1, 2, 3, 4];
        let mut index = vec![0, 1, 2, 2, 1];
        assert_eq!(
            vec![0, 4, 1, 3, 2],
            Solution::create_target_array(nums, index)
        );

        nums = vec![2, 1, 1, 7, 3];
        index = vec![0, 0, 1, 0, 1];

        assert_eq!(
            vec![7, 3, 1, 1, 2],
            Solution::create_target_array(nums, index)
        );
    }

    #[test]
    fn test_count_matches() {
        let mut items = vec![
            vec!["phone".to_owned(), "blue".to_owned(), "pixel".to_owned()],
            vec![
                "computer".to_owned(),
                "silver".to_owned(),
                "lenovo".to_owned(),
            ],
            vec!["phone".to_owned(), "gold".to_owned(), "iphone".to_owned()],
        ];
        let mut rule_key = "color".to_owned();
        let mut rule_value = "silver".to_owned();
        assert_eq!(1, Solution::count_matches(items, rule_key, rule_value));

        items = vec![
            vec!["phone".to_owned(), "blue".to_owned(), "pixel".to_owned()],
            vec![
                "computer".to_owned(),
                "silver".to_owned(),
                "phone".to_owned(),
            ],
            vec!["phone".to_owned(), "gold".to_owned(), "iphone".to_owned()],
        ];
        rule_key = "type".to_owned();
        rule_value = "phone".to_owned();
        assert_eq!(2, Solution::count_matches(items, rule_key, rule_value))
    }
}
