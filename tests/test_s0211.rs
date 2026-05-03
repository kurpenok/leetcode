use leetcode::s0211_design_add_and_search_words_data_structure::WordDictionary;

#[test]
fn test_case_1() {
    let mut word_dictionary = WordDictionary::new();

    word_dictionary.add_word("bad".to_string());
    word_dictionary.add_word("dad".to_string());
    word_dictionary.add_word("mad".to_string());

    assert_eq!(word_dictionary.search("pad".to_string()), false);
    assert_eq!(word_dictionary.search("bad".to_string()), true);
    assert_eq!(word_dictionary.search(".ad".to_string()), true);
    assert_eq!(word_dictionary.search("b..".to_string()), true);
}
