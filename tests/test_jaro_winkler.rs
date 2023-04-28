// TODO: Add approximate float comparison
// https://docs.rs/float-cmp/latest/float_cmp/
// https://docs.rs/approx/latest/approx/

#[cfg(test)]
mod tests {
    use text_distance::JaroWinkler;

    #[test]
    fn test_distance() {
        assert_eq!(0.0810185185185186, JaroWinkler {src: "faremviel".to_string(), tar: "farmville".to_string(), winklerize: true }.distance());
        assert_eq!(0.07500000000000007, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: true }.distance());
        assert_eq!(0.08333333333333337, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: false }.distance());
        assert_eq!(1.0, JaroWinkler {src: "fly".to_string(), tar: "ant".to_string(), winklerize: false }.distance());
    }

    #[test]
    fn test_normalized_distance() {
        assert_eq!(0.0810185185185186, JaroWinkler {src: "faremviel".to_string(), tar: "farmville".to_string(), winklerize: true }.normalized_distance());
        assert_eq!(0.07500000000000007, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: true }.normalized_distance());
        assert_eq!(0.08333333333333337, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: false }.normalized_distance());
        assert_eq!(1.0, JaroWinkler {src: "fly".to_string(), tar: "ant".to_string(), winklerize: false }.normalized_distance());
    }

    #[test]
    fn test_similarity() {
        assert_eq!(0.9189814814814814, JaroWinkler {src: "faremviel".to_string(), tar: "farmville".to_string(), winklerize: true }.similarity());
        assert_eq!(0.9249999999999999, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: true }.similarity());
        assert_eq!(0.9166666666666666, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: false }.similarity());
        assert_eq!(0.0, JaroWinkler {src: "fly".to_string(), tar: "ant".to_string(), winklerize: false }.similarity());
    }

    #[test]
    fn test_normalized_similarity() {
        assert_eq!(0.9189814814814814, JaroWinkler {src: "faremviel".to_string(), tar: "farmville".to_string(), winklerize: true }.normalized_similarity());
        assert_eq!(0.9249999999999999, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: true }.normalized_similarity());
        assert_eq!(0.9166666666666666, JaroWinkler {src: "frog".to_string(), tar: "fog".to_string(), winklerize: false }.normalized_similarity());
        assert_eq!(0.0, JaroWinkler {src: "fly".to_string(), tar: "ant".to_string(), winklerize: false }.normalized_similarity());
    }
}
