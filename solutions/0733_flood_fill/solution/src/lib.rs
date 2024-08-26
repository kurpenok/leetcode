pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    if image[sr as usize][sc as usize] == color {
        return image;
    }

    let mut stack: Vec<(usize, usize)> = vec![];
    let mut result: Vec<Vec<i32>> = image.clone();
    stack.push((sr as usize, sc as usize));

    while let Some(node) = stack.pop() {
        if result[node.0][node.1] != image[sr as usize][sc as usize] {
            continue;
        }

        let up = (node.0 - 1, node.1);
        let down = (node.0 + 1, node.1);
        let left = (node.0, node.1 - 1);
        let right = (node.0, node.1 + 1);

        if left.1 < usize::MAX {
            stack.push(left);
        }
        if right.1 < result[0].len() {
            stack.push(right);
        }
        if up.0 < usize::MAX {
            stack.push(up);
        }
        if down.0 < result.len() {
            stack.push(down);
        }

        result[node.0][node.1] = color;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
}
