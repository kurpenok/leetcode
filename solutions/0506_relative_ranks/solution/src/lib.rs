pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut sorted_scores = score.clone();
    sorted_scores.sort_by(|a, b| b.cmp(a));

    let mut result: Vec<String> = Vec::new();

    for i in 0..score.len() {
        let mut index: usize = 0;

        for j in 0..sorted_scores.len() {
            if score[i] == sorted_scores[j] {
                index = j;
                break;
            }
        }

        if index == 0 {
            result.push("Gold Medal".to_string());
        } else if index == 1 {
            result.push("Silver Medal".to_string());
        } else if index == 2 {
            result.push("Bronze Medal".to_string());
        } else {
            result.push((index + 1).to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"],
        );
    }
}
