
// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/


#[cfg(test)]
mod tests {
    use text_distance::Levenshtein;

    #[test]
    fn test_distance() {
        assert_eq!(0, Levenshtein {src: "".to_string(),  tar: "".to_string()}.distance());
        assert_eq!(4, Levenshtein {src: "test".to_string(),  tar: "".to_string()}.distance());
        assert_eq!(5, Levenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string()}.distance());
        assert_eq!(6, Levenshtein {src: "levenshtein".to_string(),  tar: "frankenstein".to_string()}.distance());
        assert_eq!(15, Levenshtein {src: "python is an interpreted language".to_string(),  tar: "rust is a compiled language".to_string()}.distance());
    }

    #[test]
    fn test_normalized_distance() {
        assert_eq!(0.0, Levenshtein {src: "".to_string(),  tar: "".to_string()}.normalized_distance());
        assert_eq!(1.0, Levenshtein {src: "test".to_string(),  tar: "".to_string()}.normalized_distance());
        assert_eq!(0.8333333333333334, Levenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string()}.normalized_distance());
        assert_eq!(0.5, Levenshtein {src: "levenshtein".to_string(),  tar: "frankenstein".to_string()}.normalized_distance());
        assert_eq!(0.45454545454545453, Levenshtein {src: "python is an interpreted language".to_string(),  tar: "rust is a compiled language".to_string()}.normalized_distance());
    }

    #[test]
    fn test_similarity() {
        assert_eq!(0, Levenshtein {src: "".to_string(),  tar: "".to_string()}.similarity());
        assert_eq!(0, Levenshtein {src: "test".to_string(),  tar: "".to_string()}.similarity());
        assert_eq!(1, Levenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string()}.similarity());
        assert_eq!(6, Levenshtein {src: "levenshtein".to_string(),  tar: "frankenstein".to_string()}.similarity());
        assert_eq!(18, Levenshtein {src: "python is an interpreted language".to_string(),  tar: "rust is a compiled language".to_string()}.similarity());
    }

    #[test]
    fn test_normalized_similarity() {
        assert_eq!(1.0, Levenshtein {src: "".to_string(),  tar: "".to_string()}.normalized_similarity());
        assert_eq!(0.0, Levenshtein {src: "test".to_string(),  tar: "".to_string()}.normalized_similarity());
        assert_eq!(0.16666666666666663, Levenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string()}.normalized_similarity());
        assert_eq!(0.5, Levenshtein {src: "levenshtein".to_string(),  tar: "frankenstein".to_string()}.normalized_similarity());
        assert_eq!(0.5454545454545454, Levenshtein {src: "python is an interpreted language".to_string(),  tar: "rust is a compiled language".to_string()}.normalized_similarity());
    }
}
