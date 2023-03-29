
// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/


#[cfg(test)]
mod tests {
    use text_distance::Levenshtein;

    #[test]
    fn test_distance() {
        assert_eq!(1, Levenshtein {s: "test".to_string(),  t: "text".to_string()}.distance());
        assert_eq!(2, Levenshtein {s: "book".to_string(),  t: "back".to_string()}.distance());
        assert_eq!(5, Levenshtein {s: "person".to_string(),  t: "human".to_string()}.distance());

        assert_eq!(0, Levenshtein {s: "test".to_string(),  t: "test".to_string()}.distance());
        assert_eq!(0, Levenshtein {s: "t".to_string(),  t: "t".to_string()}.distance());

        assert_eq!(1, Levenshtein {s: "Test".to_string(),  t: "Text".to_string()}.distance());
        assert_eq!(2, Levenshtein {s: "test".to_string(),  t: "Text".to_string()}.distance()); // distnace is 2, should be 1

        assert_eq!(4, Levenshtein {s: "test".to_string(),  t: "".to_string()}.distance());
        assert_eq!(4, Levenshtein {s: "".to_string(),  t: "text".to_string()}.distance());
        assert_eq!(0, Levenshtein {s: "".to_string(),  t: "".to_string()}.distance());
    }

    #[test]
    fn test_normalized_distance() {
        assert_eq!(0.0, Levenshtein {s: "test".to_string(),  t: "test".to_string()}.normalized_distance());
        assert_eq!(0.25, Levenshtein {s: "test".to_string(),  t: "text".to_string()}.normalized_distance());
        assert_eq!(0.5, Levenshtein {s: "book".to_string(),  t: "back".to_string()}.normalized_distance());
        assert_eq!(0.8333333333333334, Levenshtein {s: "person".to_string(),  t: "human".to_string()}.normalized_distance());
        assert_eq!(1.0, Levenshtein {s: "test".to_string(),  t: "book".to_string()}.normalized_distance());
    }

    #[test]
    fn test_similarity() {
        assert_eq!(4, Levenshtein {s: "test".to_string(),  t: "test".to_string()}.similarity());
        assert_eq!(3, Levenshtein {s: "test".to_string(),  t: "text".to_string()}.similarity());
        assert_eq!(2, Levenshtein {s: "book".to_string(),  t: "back".to_string()}.similarity());
        assert_eq!(1, Levenshtein {s: "person".to_string(),  t: "human".to_string()}.similarity());
        assert_eq!(0, Levenshtein {s: "test".to_string(),  t: "book".to_string()}.similarity());
    }

    #[test]
    fn test_normalized_similarity() {
        assert_eq!(1.0, Levenshtein {s: "test".to_string(),  t: "test".to_string()}.normalized_similarity());
        assert_eq!(0.75, Levenshtein {s: "test".to_string(),  t: "text".to_string()}.normalized_similarity());
        assert_eq!(0.5, Levenshtein {s: "book".to_string(),  t: "back".to_string()}.normalized_similarity());
        assert_eq!(0.16666666666666663, Levenshtein {s: "person".to_string(),  t: "human".to_string()}.normalized_similarity());
        assert_eq!(0.0, Levenshtein {s: "test".to_string(),  t: "book".to_string()}.normalized_similarity());
    }
}
