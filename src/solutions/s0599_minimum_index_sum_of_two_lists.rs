use std::collections::HashSet;

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut indexes: Vec<Vec<usize>> = Vec::new();
    let mut sum_indexes: Vec<usize> = Vec::new();

    for i in 0..list1.len() {
        for j in 0..list2.len() {
            if list1[i] == list2[j] {
                indexes.push(vec![i, j]);
                sum_indexes.push(i + j);
            }
        }
    }

    sum_indexes.sort();

    let mut result: HashSet<String> = HashSet::new();
    for index_pair in indexes {
        if index_pair[0] + index_pair[1] == sum_indexes[0] {
            result.insert(list1[index_pair[0]].clone());
        }
    }

    result.into_iter().collect()
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_case_1() {
//        assert_eq!(
//            find_restaurant(
//                vec![
//                    "Shogun".to_string(),
//                    "Tapioca Express".to_string(),
//                    "Burger King".to_string(),
//                    "KFC".to_string()
//                ],
//                vec![
//                    "Piatti".to_string(),
//                    "The Grill at Torrey Pines".to_string(),
//                    "Hungry Hunter Steakhouse".to_string(),
//                    "Shogun".to_string(),
//                ]
//            ),
//            vec!["Shogun"],
//        );
//    }
//
//    #[test]
//    fn test_case_2() {
//        assert_eq!(
//            find_restaurant(
//                vec![
//                    "Shogun".to_string(),
//                    "Tapioca Express".to_string(),
//                    "Burger King".to_string(),
//                    "KFC".to_string()
//                ],
//                vec![
//                    "KFC".to_string(),
//                    "Shogun".to_string(),
//                    "Burger King".to_string()
//                ]
//            ),
//            vec!["Shogun"],
//        );
//    }
//
//    #[test]
//    fn test_case_3() {
//        assert_eq!(
//            find_restaurant(
//                vec!["happy".to_string(), "sad".to_string(), "good".to_string()],
//                vec!["sad".to_string(), "happy".to_string(), "good".to_string()]
//            ),
//            vec!["sad", "happy"],
//        );
//    }
//}
