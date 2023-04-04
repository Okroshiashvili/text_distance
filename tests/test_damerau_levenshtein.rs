
// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/


#[cfg(test)]
mod tests {
    use text_distance::DamerauLevenshtein;

    // restricted
    #[test]
    fn test_distance() {
        assert_eq!(0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: true}.distance());
        assert_eq!(4, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: true}.distance());
        assert_eq!(5, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: true}.distance());
        assert_eq!(3, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: true}.distance());
    }

    #[test]
    fn test_normalized_distance() {
        assert_eq!(0.0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: true}.normalized_distance());
        assert_eq!(1.0, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: true}.normalized_distance());
        assert_eq!(0.8333333333333334, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: true}.normalized_distance());
        assert_eq!(1.0, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: true}.normalized_distance());
    }

    #[test]
    fn test_similarity() {
        assert_eq!(0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: true}.similarity());
        assert_eq!(0, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: true}.similarity());
        assert_eq!(1, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: true}.similarity());
        assert_eq!(0, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: true}.similarity());
    }

    #[test]
    fn test_normalized_similarity() {
        assert_eq!(1.0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: true}.normalized_similarity());
        assert_eq!(0.0, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: true}.normalized_similarity());
        assert_eq!(0.16666666666666663, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: true}.normalized_similarity());
        assert_eq!(0.0, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: true}.normalized_similarity());
    }

    // Unrestricted
    #[test]
    fn test_unrestricted_distance() {
        assert_eq!(0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: false}.distance());
        assert_eq!(4, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: false}.distance());
        assert_eq!(5, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: false}.distance());
        assert_eq!(2, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: false}.distance());
    }

    #[test]
    fn test_unrestricted_normalized_distance() {
        assert_eq!(0.0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: false}.normalized_distance());
        assert_eq!(1.0, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: false}.normalized_distance());
        assert_eq!(0.8333333333333334, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: false}.normalized_distance());
        assert_eq!(0.6666666666666666, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: false}.normalized_distance());
    }

    #[test]
    fn test_unrestricted_similarity() {
        assert_eq!(0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: false}.similarity());
        assert_eq!(0, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: false}.similarity());
        assert_eq!(1, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: false}.similarity());
        assert_eq!(1, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: false}.similarity());
    }

    #[test]
    fn test_unrestricted_normalized_similarity() {
        assert_eq!(1.0, DamerauLevenshtein {src: "".to_string(),  tar: "".to_string(), restricted: false}.normalized_similarity());
        assert_eq!(0.0, DamerauLevenshtein {src: "test".to_string(),  tar: "".to_string(), restricted: false}.normalized_similarity());
        assert_eq!(0.16666666666666663, DamerauLevenshtein {src: "abcdef".to_string(),  tar: "axb123".to_string(), restricted: false}.normalized_similarity());
        assert_eq!(0.33333333333333337, DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: false}.normalized_similarity());
    }
}
