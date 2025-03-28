use std::collections::HashMap;

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let cities_counter: HashMap<&str, (bool, usize)> =
        paths.iter().fold(HashMap::new(), |mut map, path| {
            let _ = *map
                .entry(&path[0])
                .and_modify(|(_, count)| *count += 1)
                .or_insert((false, 1));
            let _ = *map
                .entry(&path[1])
                .and_modify(|(_, count)| *count += 1)
                .or_insert((true, 1));
            map
        });

    for (city, (is_last, count)) in cities_counter {
        if count == 1 && is_last {
            return city.to_string();
        }
    }

    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ]),
            "Sao Paulo"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            dest_city(vec![
                vec!["B".to_string(), "C".to_string()],
                vec!["D".to_string(), "B".to_string()],
                vec!["C".to_string(), "A".to_string()]
            ]),
            "A"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(dest_city(vec![vec!["A".to_string(), "Z".to_string()]]), "Z");
    }
}
