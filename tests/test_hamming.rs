
// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/


#[cfg(test)]
mod tests {
    use text_distance::Hamming;

    #[test]
    fn test_distance() {
        assert_eq!(0.0, Hamming {src: "test".to_string(),  tar: "test".to_string()}.normalized_distance());
        assert_eq!(1, Hamming {src: "test".to_string(),  tar: "text".to_string()}.distance());
    }

    #[test]
    fn test_normalized_distance() {
        assert_eq!(0.0, Hamming {src: "test".to_string(),  tar: "test".to_string()}.normalized_distance());
        assert_eq!(0.25, Hamming {src: "test".to_string(),  tar: "text".to_string()}.normalized_distance());
    }

    #[test]
    fn test_similarity() {
        assert_eq!(4, Hamming {src: "test".to_string(),  tar: "test".to_string()}.similarity());
        assert_eq!(3, Hamming {src: "test".to_string(),  tar: "text".to_string()}.similarity());
    }

    #[test]
    fn test_normalized_similarity() {
        assert_eq!(1.0, Hamming {src: "test".to_string(),  tar: "test".to_string()}.normalized_similarity());
        assert_eq!(0.75, Hamming {src: "test".to_string(),  tar: "text".to_string()}.normalized_similarity());
    }

    #[test]
    #[should_panic(expected = "Hamming distance is only defined for strings of equal length")]
    fn test_hamming_panic() {
        Hamming {src: "test".to_string(),  tar: "textt".to_string()}.distance();
    }
}
