pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut b = vec![vec![0; img[0].len()]; img.len()];

    for i in 0..img.len() {
        for j in 0..img[0].len() {
            let (mut s, mut kk) = (0, 0);
            for l in i.saturating_sub(1)..(i + 2).min(img.len()) {
                for k in j.saturating_sub(1)..(j + 2).min(img[0].len()) {
                    kk += 1;
                    s += img[l][k];
                }
            }
            b[i][j] = s / kk as i32;
        }
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            [[137, 141, 137], [141, 138, 141], [137, 141, 137]],
        );
    }
}
