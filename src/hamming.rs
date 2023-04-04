use std::cmp::max;

/// Calculate the `Hamming` distance between two strings of equal length.
/// The Hamming distance is the number of differing items in ordered sequences.
/// [For more information see wikipedia article](https://en.wikipedia.org/wiki/Hamming_distance)
/// 
/// ### Examples
/// 
/// ```
/// use text_distance::Hamming;
/// 
/// let hamming = Hamming {src: "karolin".to_string(),  tar: "kathrin".to_string()};
/// 
/// assert_eq!(3, hamming.distance());
/// assert_eq!(0.42857142857142855, hamming.normalized_distance());
/// assert_eq!(4, hamming.similarity());
/// assert_eq!(0.5714285714285714, hamming.normalized_similarity());
/// 
/// ```
/// 
pub struct Hamming {
    pub src: String,
    pub tar: String,
}

impl Hamming {
    /// Calculate the `Hamming` distance between two strings of equal length.
    /// If not equal length then panic.
    /// 
    /// ### Examples
    /// 
    /// ```
    /// use text_distance::Hamming;
    /// 
    /// let hamming = Hamming {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    /// 
    /// assert_eq!(3, hamming.distance());
    /// 
    /// ```
    pub fn distance(&self) -> usize {
        if self.src.chars().count() != self.tar.chars().count() {
            panic!("Hamming distance is only defined for strings of equal length");
        }

        let mut distance_counter = 0;

        for (s_char, t_char) in self.src.chars().zip(self.tar.chars()) {
            if s_char != t_char {
                distance_counter += 1;
            }
        }
        return distance_counter;
    }

    /// Calculate the `normalized distance` between two strings.
    /// The normalized distance is the distance divided by the length of the longest string.
    /// The normalized distance is always between 0.0 and 1.0.
    /// When 0.0 then two strings are equal.
    /// When 1.0 then two strings are completely different.
    /// 
    /// ### Examples
    /// 
    /// ```
    /// use text_distance::Hamming;;
    /// 
    /// let hamming = Hamming {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    /// 
    /// assert_eq!(0.42857142857142855, hamming.normalized_distance());
    /// 
    /// ```
    pub fn normalized_distance(&self) -> f64 {
        let maximum = max(
            self.src.clone().chars().count(),
            self.tar.clone().chars().count(),
        );
        let str_distance = self.distance();
        if maximum != 0 {
            return (str_distance as f64) / (maximum as f64);
        }
        return 0.0;
    }

    /// Calculate the `similarity` between two strings.
    /// The similarity is the length of the longest string minus the distance.
    /// The similarity is always between 0 and the length of the longest string.
    /// When 0 then two strings are completely different.
    /// When the length of the longest string then two strings are equal.
    /// 
    /// ### Examples
    /// 
    /// ```
    /// use text_distance::Hamming;
    /// 
    /// let hamming = Hamming {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    /// 
    /// assert_eq!(4, hamming.similarity());
    /// 
    /// ```
    pub fn similarity(&self) -> usize {
        let maximum = max(
            self.src.clone().chars().count(),
            self.tar.clone().chars().count(),
        );
        let str_distance = self.distance();
        return maximum - str_distance;
    }

    /// Calculate the `normalized similarity` between two strings.
    /// The normalized similarity is the similarity divided by the length of the longest string.
    /// The normalized similarity is always between 0.0 and 1.0.
    /// When 0.0 then two strings are completely different.
    /// When 1.0 then two strings are equal.
    /// The normalized similarity is the same as 1.0 minus the normalized distance.
    /// 
    /// ### Examples
    /// 
    /// ```
    /// use text_distance::Hamming;
    /// 
    /// let hamming = Hamming {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    /// 
    /// assert_eq!(0.5714285714285714, hamming.normalized_similarity());
    /// 
    /// ```
    pub fn normalized_similarity(&self) -> f64 {
        let str_normalized_distance = self.normalized_distance();
        return 1.0 - str_normalized_distance;
    }
}
