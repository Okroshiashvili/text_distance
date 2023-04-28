
// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/


#[cfg(test)]
mod tests {
    use text_distance::Jaccard;

    #[test]
    fn test_distance() {
        assert_eq!(0.5555555555555556, Jaccard {src: "data is the new oil of the digital economy".to_string(),  tar: "data is a new oil".to_string(), qval: 0}.distance());
        assert_eq!(0.33333333333333337, Jaccard {src: "nelson".to_string(),  tar: "neilsen".to_string(), qval: 1}.distance());
        assert_eq!(0.44999999999999996, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 2}.distance());
        assert_eq!(0.6190476190476191, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 3}.distance());
    }

    #[test]
    fn test_normalized_distance() {
        assert_eq!(0.5555555555555556, Jaccard {src: "data is the new oil of the digital economy".to_string(),  tar: "data is a new oil".to_string(), qval: 0}.normalized_distance());
        assert_eq!(0.33333333333333337, Jaccard {src: "nelson".to_string(),  tar: "neilsen".to_string(), qval: 1}.normalized_distance());
        assert_eq!(0.44999999999999996, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 2}.normalized_distance());
        assert_eq!(0.6190476190476191, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 3}.normalized_distance());
    }

    #[test]
    fn test_similarity() {
        assert_eq!(0.4444444444444444, Jaccard {src: "data is the new oil of the digital economy".to_string(),  tar: "data is a new oil".to_string(), qval: 0}.similarity());
        assert_eq!(0.6666666666666666, Jaccard {src: "nelson".to_string(),  tar: "neilsen".to_string(), qval: 1}.similarity());
        assert_eq!(0.55, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 2}.similarity());
        assert_eq!(0.38095238095238093, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 3}.similarity());
    }

    #[test]
    fn test_normalized_similarity() {
        assert_eq!(0.4444444444444444, Jaccard {src: "data is the new oil of the digital economy".to_string(),  tar: "data is a new oil".to_string(), qval: 0}.normalized_similarity());
        assert_eq!(0.6666666666666666, Jaccard {src: "nelson".to_string(),  tar: "neilsen".to_string(), qval: 1}.normalized_similarity());
        assert_eq!(0.55, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 2}.normalized_similarity());
        assert_eq!(0.38095238095238093, Jaccard {src: "nelson has a car".to_string(),  tar: "neilsen has a cat".to_string(), qval: 3}.normalized_similarity());
    }

    #[test]
    #[should_panic(expected = "Can't create n-grams from text shorter than n-gram length")]
    fn test_jaccard_panic() {
        Jaccard {src: "nelson".to_string(),  tar: "neilsen".to_string(), qval: 100}.distance();
    }
}
