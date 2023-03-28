
// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/


#[cfg(test)]
mod tests {
    use text_distance::edit::Levenshtein;
    use text_distance::edit::Hamming;

    #[test]
    fn test_levenshtein() {

        // Distance
        assert_eq!(1, Levenshtein {s: "test".to_owned(),  t: "text".to_owned()}.distance());
        assert_eq!(2, Levenshtein {s: "book".to_owned(),  t: "back".to_owned()}.distance());
        assert_eq!(5, Levenshtein {s: "person".to_owned(),  t: "human".to_owned()}.distance());

        assert_eq!(0, Levenshtein {s: "test".to_owned(),  t: "test".to_owned()}.distance());
        assert_eq!(0, Levenshtein {s: "t".to_owned(),  t: "t".to_owned()}.distance());

        assert_eq!(1, Levenshtein {s: "Test".to_owned(),  t: "Text".to_owned()}.distance());
        assert_eq!(2, Levenshtein {s: "test".to_owned(),  t: "Text".to_owned()}.distance()); // distnace is 2, should be 1

        assert_eq!(4, Levenshtein {s: "test".to_owned(),  t: "".to_owned()}.distance());
        assert_eq!(4, Levenshtein {s: "".to_owned(),  t: "text".to_owned()}.distance());
        assert_eq!(0, Levenshtein {s: "".to_owned(),  t: "".to_owned()}.distance());

        // Normalized Distance
        assert_eq!(0.0, Levenshtein {s: "test".to_owned(),  t: "test".to_owned()}.normalized_distance());
        assert_eq!(0.25, Levenshtein {s: "test".to_owned(),  t: "text".to_owned()}.normalized_distance());
        assert_eq!(0.5, Levenshtein {s: "book".to_owned(),  t: "back".to_owned()}.normalized_distance());
        assert_eq!(0.8333333333333334, Levenshtein {s: "person".to_owned(),  t: "human".to_owned()}.normalized_distance());
        assert_eq!(1.0, Levenshtein {s: "test".to_owned(),  t: "book".to_owned()}.normalized_distance());

        // Similarity
        assert_eq!(4, Levenshtein {s: "test".to_owned(),  t: "test".to_owned()}.similarity());
        assert_eq!(3, Levenshtein {s: "test".to_owned(),  t: "text".to_owned()}.similarity());
        assert_eq!(2, Levenshtein {s: "book".to_owned(),  t: "back".to_owned()}.similarity());
        assert_eq!(1, Levenshtein {s: "person".to_owned(),  t: "human".to_owned()}.similarity());
        assert_eq!(0, Levenshtein {s: "test".to_owned(),  t: "book".to_owned()}.similarity());

        // Normalized Similarity
        assert_eq!(1.0, Levenshtein {s: "test".to_owned(),  t: "test".to_owned()}.normalized_similarity());
        assert_eq!(0.75, Levenshtein {s: "test".to_owned(),  t: "text".to_owned()}.normalized_similarity());
        assert_eq!(0.5, Levenshtein {s: "book".to_owned(),  t: "back".to_owned()}.normalized_similarity());
        assert_eq!(0.16666666666666663, Levenshtein {s: "person".to_owned(),  t: "human".to_owned()}.normalized_similarity());
        assert_eq!(0.0, Levenshtein {s: "test".to_owned(),  t: "book".to_owned()}.normalized_similarity());
    }

    #[test]
    fn test_hamming() {
        // Distance
        assert_eq!(0.0, Hamming {s: "test".to_owned(),  t: "test".to_owned()}.normalized_distance());
        assert_eq!(1, Hamming {s: "test".to_owned(),  t: "text".to_owned()}.distance());

        // Normalized Distance
        assert_eq!(0.0, Hamming {s: "test".to_owned(),  t: "test".to_owned()}.normalized_distance());
        assert_eq!(0.25, Hamming {s: "test".to_owned(),  t: "text".to_owned()}.normalized_distance());

        // Similarity
        assert_eq!(4, Hamming {s: "test".to_owned(),  t: "test".to_owned()}.similarity());
        assert_eq!(3, Hamming {s: "test".to_owned(),  t: "text".to_owned()}.similarity());

        // Normalized Similarity
        assert_eq!(1.0, Hamming {s: "test".to_owned(),  t: "test".to_owned()}.normalized_similarity());
        assert_eq!(0.75, Hamming {s: "test".to_owned(),  t: "text".to_owned()}.normalized_similarity());
    }

    #[test]
    #[should_panic]
    fn test_panics() {
        Hamming {s: "test".to_owned(),  t: "textt".to_owned()}.distance();
    }

}
