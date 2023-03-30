
// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/


#[cfg(test)]
mod tests {
    use text_distance::Levenshtein;

    #[test]
    fn test_distance() {
        assert_eq!(1, Levenshtein {src: "test".to_string(),  tar: "text".to_string()}.distance());
        assert_eq!(2, Levenshtein {src: "book".to_string(),  tar: "back".to_string()}.distance());
        assert_eq!(5, Levenshtein {src: "person".to_string(),  tar: "human".to_string()}.distance());

        assert_eq!(0, Levenshtein {src: "test".to_string(),  tar: "test".to_string()}.distance());
        assert_eq!(0, Levenshtein {src: "t".to_string(),  tar: "t".to_string()}.distance());

        assert_eq!(1, Levenshtein {src: "Test".to_string(),  tar: "Text".to_string()}.distance());
        assert_eq!(2, Levenshtein {src: "test".to_string(),  tar: "Text".to_string()}.distance()); // distnace is 2, should be 1

        assert_eq!(4, Levenshtein {src: "test".to_string(),  tar: "".to_string()}.distance());
        assert_eq!(4, Levenshtein {src: "".to_string(),  tar: "text".to_string()}.distance());
        assert_eq!(0, Levenshtein {src: "".to_string(),  tar: "".to_string()}.distance());
    }

    #[test]
    fn test_normalized_distance() {
        assert_eq!(0.0, Levenshtein {src: "test".to_string(),  tar: "test".to_string()}.normalized_distance());
        assert_eq!(0.25, Levenshtein {src: "test".to_string(),  tar: "text".to_string()}.normalized_distance());
        assert_eq!(0.5, Levenshtein {src: "book".to_string(),  tar: "back".to_string()}.normalized_distance());
        assert_eq!(0.8333333333333334, Levenshtein {src: "person".to_string(),  tar: "human".to_string()}.normalized_distance());
        assert_eq!(1.0, Levenshtein {src: "test".to_string(),  tar: "book".to_string()}.normalized_distance());
    }

    #[test]
    fn test_similarity() {
        assert_eq!(4, Levenshtein {src: "test".to_string(),  tar: "test".to_string()}.similarity());
        assert_eq!(3, Levenshtein {src: "test".to_string(),  tar: "text".to_string()}.similarity());
        assert_eq!(2, Levenshtein {src: "book".to_string(),  tar: "back".to_string()}.similarity());
        assert_eq!(1, Levenshtein {src: "person".to_string(),  tar: "human".to_string()}.similarity());
        assert_eq!(0, Levenshtein {src: "test".to_string(),  tar: "book".to_string()}.similarity());
    }

    #[test]
    fn test_normalized_similarity() {
        assert_eq!(1.0, Levenshtein {src: "test".to_string(),  tar: "test".to_string()}.normalized_similarity());
        assert_eq!(0.75, Levenshtein {src: "test".to_string(),  tar: "text".to_string()}.normalized_similarity());
        assert_eq!(0.5, Levenshtein {src: "book".to_string(),  tar: "back".to_string()}.normalized_similarity());
        assert_eq!(0.16666666666666663, Levenshtein {src: "person".to_string(),  tar: "human".to_string()}.normalized_similarity());
        assert_eq!(0.0, Levenshtein {src: "test".to_string(),  tar: "book".to_string()}.normalized_similarity());
    }
}
