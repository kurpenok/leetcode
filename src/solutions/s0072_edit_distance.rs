pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1_len = word1.chars().count();
    let word2_len = word2.chars().count();

    let mut matrix = vec![vec![0; word2_len + 1]; word1_len + 1];

    for i in 0..=word1_len {
        matrix[i][0] = i;
    }

    for j in 0..=word2_len {
        matrix[0][j] = j;
    }

    for (i, word1_char) in word1.chars().enumerate() {
        for (j, word2_char) in word2.chars().enumerate() {
            let substitution_cost = if word1_char == word2_char { 0 } else { 1 };

            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1, // deletion
                    matrix[i + 1][j] + 1, // insertion
                ),
                matrix[i][j] + substitution_cost, // substitution
            );
        }
    }

    matrix[word1_len][word2_len] as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
