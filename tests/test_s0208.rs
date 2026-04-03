use leetcode::s0208_implement_trie::Trie;

#[test]
fn test_case_1() {
    let mut trie = Trie::new();

    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);

    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}
