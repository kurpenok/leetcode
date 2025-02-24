pub fn simplify_path(path: String) -> String {
    let mut new_path: Vec<String> = Vec::new();

    for part in path.split("/") {
        if part == "" || part == "." {
            continue;
        } else if part == ".." {
            new_path.pop();
        } else {
            new_path.push(part.to_string());
        }
    }

    if new_path.len() == 0 {
        "/".to_string()
    } else {
        format!("/{}", new_path.join("/"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(simplify_path("/home/".to_string()), "/home");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(simplify_path("/home//foo/".to_string()), "/home/foo");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            simplify_path("/home/user/Documents/../Pictures".to_string()),
            "/home/user/Pictures"
        );
    }

    #[test]
    fn test_case_4() {
        assert_eq!(simplify_path("/../".to_string()), "/");
    }

    #[test]
    fn test_case_5() {
        assert_eq!(
            simplify_path("/.../a/../b/c/../d/./".to_string()),
            "/.../b/d"
        );
    }
}
