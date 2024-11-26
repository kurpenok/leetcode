use std::collections::HashMap;

fn get_distance(r_1: i32, c_1: i32, r_2: i32, c_2: i32) -> i32 {
    (r_1 - r_2).abs() + (c_1 - c_2).abs()
}

pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let mut distances_map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            distances_map
                .entry(get_distance(r, c, r_center, c_center))
                .and_modify(|coords| coords.push(vec![r, c]))
                .or_insert(vec![vec![r, c]]);
        }
    }

    let mut distances: Vec<i32> = distances_map.keys().cloned().collect();
    distances.sort();

    let mut coords: Vec<Vec<i32>> = Vec::new();
    for distance in distances {
        coords.extend(distances_map[&distance].clone());
    }

    coords
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(all_cells_dist_order(1, 2, 0, 0), [[0, 0], [0, 1]]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            all_cells_dist_order(2, 2, 0, 1),
            [[0, 1], [0, 0], [1, 1], [1, 0]],
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            all_cells_dist_order(2, 3, 1, 2),
            [[1, 2], [0, 2], [1, 1], [0, 1], [1, 0], [0, 0]],
        );
    }
}
