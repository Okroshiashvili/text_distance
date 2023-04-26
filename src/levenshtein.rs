use std::cmp::max;

/// Calculate the `Levenshtein` distance between two strings.
/// The Levenshtein distance between two words is the minimum number of single-character edits
/// (insertions, deletions or substitutions) required to change one word into the other.
/// [For more information see wikipedia article](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// ### Examples
///
/// ```
/// use text_distance::Levenshtein;
///
/// let levenshtein = Levenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string()};
///
/// assert_eq!(3, levenshtein.distance());
/// assert_eq!(0.42857142857142855, levenshtein.normalized_distance());
/// assert_eq!(4, levenshtein.similarity());
/// assert_eq!(0.5714285714285714, levenshtein.normalized_similarity());
///
/// ```
///
pub struct Levenshtein {
    /// Source string
    pub src: String,
    /// Target string
    pub tar: String,
}

impl Levenshtein {
    /// Calculate the `Levenshtein` distance between two strings.
    /// The distance is the number of edit operations needed to transform the source string into the target string.
    /// The edit operations are:
    /// - deletion (removal) of a single character from any position in the string.
    /// - insertion (addition) of a single character at any position in the string.
    /// - substitution (replacement) of a single character with another character.
    ///
    /// ### Examples
    ///
    /// ```
    /// use text_distance::Levenshtein;
    ///
    /// let levenshtein = Levenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    ///
    /// assert_eq!(3, levenshtein.distance());
    ///
    /// ```
    pub fn distance(&self) -> usize {
        let len_src = self.src.chars().count();
        let len_tar = self.tar.chars().count();

        // initialize the matrix
        let mut matrix: Vec<Vec<usize>> = vec![vec![0; len_tar + 1]; len_src + 1];

        for i in 1..(len_src + 1) {
            matrix[i][0] = i;
        }
        for i in 1..(len_tar + 1) {
            matrix[0][i] = i;
        }

        // apply edit operations
        for (i, s_char) in self.src.chars().enumerate() {
            for (j, t_char) in self.tar.chars().enumerate() {
                let substitution_cost = if s_char == t_char { 0 } else { 1 };
                let operations = [
                    matrix[i][j + 1] + 1,             // deletion
                    matrix[i + 1][j] + 1,             // insertion
                    matrix[i][j] + substitution_cost, // substitution
                ];
                matrix[i + 1][j + 1] = operations.iter().min().unwrap().clone();
            }
        }

        matrix[len_src][len_tar]
    }

    /// Calculate the `normalized distance` between two strings.
    /// The normalized distance is the distance divided by the length of the longest string.
    /// The normalized distance is always between 0.0 and 1.0.
    /// When 0.0 then two strings are equal.
    /// When 1.0 then two strings are completely different.
    ///
    /// ## Examples
    ///
    /// ```
    /// use text_distance::Levenshtein;
    ///
    /// let levenshtein = Levenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    ///
    /// assert_eq!(0.42857142857142855, levenshtein.normalized_distance());
    ///
    /// ```
    ///
    pub fn normalized_distance(&self) -> f64 {
        let maximum = max(
            self.src.clone().chars().count(),
            self.tar.clone().chars().count(),
        );
        let str_distance = self.distance();
        if maximum != 0 {
            return (str_distance as f64) / (maximum as f64);
        }

        0.0
    }

    /// Calculate the `similarity` between two strings.
    /// The similarity is the length of the longest string minus the distance.
    /// The similarity is always between 0 and the length of the longest string.
    /// When 0 then two strings are completely different.
    /// When the length of the longest string then two strings are equal.
    ///
    /// ## Examples
    ///
    /// ```
    /// use text_distance::Levenshtein;
    ///
    /// let levenshtein = Levenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    ///
    /// assert_eq!(4, levenshtein.similarity());
    ///
    /// ```
    ///
    pub fn similarity(&self) -> usize {
        let maximum = max(
            self.src.clone().chars().count(),
            self.tar.clone().chars().count(),
        );
        let str_distance = self.distance();

        maximum - str_distance
    }

    /// Calculate the `normalized similarity` between two strings.
    /// The normalized similarity is 1 minus normalized distance.
    /// The normalized similarity is always between 0.0 and 1.0.
    /// When 0.0 then two strings are completely different.
    /// When 1.0 then two strings are equal.
    ///
    /// ## Examples
    ///
    /// ```
    /// use text_distance::Levenshtein;
    ///
    /// let levenshtein = Levenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string()};
    ///
    /// assert_eq!(0.5714285714285714, levenshtein.normalized_similarity());
    ///
    /// ```
    ///
    pub fn normalized_similarity(&self) -> f64 {
        let str_normalized_distance = self.normalized_distance();

        1.0 - str_normalized_distance
    }
}
