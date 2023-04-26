use std::collections::HashSet;

/// Calculate the `Jaccard` index between two sets.
/// The Jaccard index between two words/chars/ngrams is the intersection divided by the union.
/// [For more information see wikipedia article](https://en.wikipedia.org/wiki/Jaccard_index)
///
/// ### Examples
///
/// ```
/// use text_distance::Jaccard;
///
/// let jaccard = Jaccard {src: "karolin".to_string(),  tar: "kathrin".to_string(), qval: 1};
///
/// assert_eq!(0.44444442, jaccard.distance());
/// assert_eq!(0.44444442, jaccard.normalized_distance());
/// assert_eq!(0.5555556, jaccard.similarity());
/// assert_eq!(0.5555556, jaccard.normalized_similarity());
///
/// ```
///
pub struct Jaccard {
    /// Source string
    pub src: String,
    /// Target string
    pub tar: String,
    /// q-gram value.
    /// * If `qval = 0` then q-grams are words
    /// * If `qval = 1` then q-grams are chars
    /// * If `qval > 1` then q-grams are ngrams
    pub qval: usize,
}

impl Jaccard {
    // TODO: https://stackoverflow.com/a/51261570
    fn tokenize(&self, text: &String) -> HashSet<String> {
        match self.qval {
            // by words
            0 => {
                let tokens: HashSet<String> = text.split_whitespace().map(String::from).collect();
                tokens
            }
            // by chars
            1 => {
                let tokens: HashSet<String> = text.chars().map(String::from).collect();
                tokens
            }
            // by ngrams
            _ => {
                if text.chars().count() < self.qval {
                    panic!("Can't create n-grams from text shorter than n-gram length")
                }
                let tokens: HashSet<String> = text
                    .as_bytes()
                    .windows(self.qval)
                    .map(|ngram| String::from_utf8_lossy(ngram).to_string())
                    .collect();
                tokens
            }
        }
    }

    /// Calculate the `Jaccard` distance between two strings.
    /// The Jaccard distance is 1 minus Jaccard index.
    /// In other words it is 1 minus intersection over union.
    /// The Jaccard distance is always between 0.0 and 1.0.
    /// When 0.0 then two strings are equal.
    /// When 1.0 then two strings are completely different.
    ///
    /// ### Examples
    ///
    /// ```
    /// use text_distance::Jaccard;
    ///
    /// let jaccard = Jaccard {src: "karolin".to_string(),  tar: "kathrin".to_string(), qval: 1};
    ///
    /// assert_eq!(0.44444442, jaccard.distance())
    ///
    /// ```
    ///
    pub fn distance(&self) -> f32 {
        let src_tokens = self.tokenize(&self.src);
        let tar_tokens = self.tokenize(&self.tar);

        let tokens_intersection: HashSet<&String> = src_tokens.intersection(&tar_tokens).collect();
        let tokens_union: HashSet<&String> = src_tokens.union(&tar_tokens).collect();
        // let tokens_intersection = src_tokens.intersection(&tar_tokens).collect::<HashSet<&String>>();
        // let tokens_union = src_tokens.union(&tar_tokens).collect::<HashSet<&String>>();

        1.0 - (tokens_intersection.len() as f32 / tokens_union.len() as f32)
    }

    /// Calculate the `normalized distance`.
    /// The normalized distance is the distance divided by the 1.
    /// The normalized distance is always between 0.0 and 1.0.
    /// When 0.0 then two strings are equal.
    /// When 1.0 then two strings are completely different.
    ///
    /// ## Examples
    ///
    /// ```
    /// use text_distance::Jaccard;
    ///
    /// let jaccard = Jaccard {src: "karolin".to_string(),  tar: "kathrin".to_string(), qval: 1};
    ///
    /// assert_eq!(0.44444442, jaccard.normalized_distance());
    ///
    /// ```
    pub fn normalized_distance(&self) -> f32 {
        let str_distance = self.distance();

        str_distance / 1.0
    }

    /// Calculate the `similarity`.
    /// The Jaccard similarity/index between two sets is: intersection over union.
    /// The Jaccard similarity is always between 0.0 and 1.0.
    /// When 0.0 then two strings are completely different.
    /// When 1.0 then two strings are equal.
    ///
    /// ## Examples
    ///
    /// ```
    /// use text_distance::Jaccard;
    ///
    /// let jaccard = Jaccard {src: "karolin".to_string(),  tar: "kathrin".to_string(), qval: 1};
    ///
    /// assert_eq!(0.5555556, jaccard.similarity());
    ///
    /// ```
    pub fn similarity(&self) -> f32 {
        let str_distance = self.distance();

        1.0 - str_distance
    }

    /// Calculate the `normalized similarity`.
    /// The normalized similarity is 1 minus normalized distance.
    /// The normalized similarity is always between 0.0 and 1.0.
    /// When 0.0 then two strings are completely different.
    /// When 1.0 then two strings are equal.
    ///
    /// ## Examples
    ///
    /// ```
    /// use text_distance::Jaccard;
    ///
    /// let jaccard = Jaccard {src: "karolin".to_string(),  tar: "kathrin".to_string(), qval: 1};
    ///
    /// assert_eq!(0.5555556, jaccard.normalized_similarity());
    ///
    /// ```
    ///
    pub fn normalized_similarity(&self) -> f32 {
        let str_normalized_distance = self.normalized_distance();

        1.0 - str_normalized_distance
    }
}
