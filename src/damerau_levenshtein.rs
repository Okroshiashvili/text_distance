use std::cmp::{max, min};
use std::collections::HashMap;

/// Calculate the `Damerau-Levenshtein` distance between two strings.
/// The Damerau-Levenshtein distance is a string metric for measuring edit distance between two sequences.
/// The Damerau-Levenshtein distance differs from the Levenshtein distance by including transpositions as a single edit operation.
/// [For more information see wikipedia article](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)
/// 
/// ### Examples
/// 
/// ```
/// use text_distance::DamerauLevenshtein;
/// 
/// let damerau_levenshtein = DamerauLevenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string(), restricted: true};
/// 
/// assert_eq!(3, damerau_levenshtein.distance());
/// assert_eq!(0.42857142857142855, damerau_levenshtein.normalized_distance());
/// assert_eq!(4, damerau_levenshtein.similarity());
/// assert_eq!(0.5714285714285714, damerau_levenshtein.normalized_similarity());
/// 
/// ```
/// 
pub struct DamerauLevenshtein {
    pub src: String,
    pub tar: String,
    pub restricted: bool,
}

impl DamerauLevenshtein {
    fn restricted_distance(&self) -> usize {
        // get length of unicode chars
        let len_src = self.src.chars().count();
        let len_tar = self.tar.chars().count();

        let mut matrix: Vec<Vec<usize>> = vec![vec![0; len_tar + 1]; len_src + 1];

        for i in 0..(len_src + 1) {
            matrix[i][0] = i;
        }

        for j in 0..(len_tar + 1) {
            matrix[0][j] = j;
        }

        for (i, s_char) in self.src.chars().enumerate() {
            for (j, t_char) in self.tar.chars().enumerate() {
                let substitution_cost = if s_char == t_char { 0 } else { 1 };
                let operations = [
                    matrix[i][j + 1] + 1,             // deletion
                    matrix[i + 1][j] + 1,             // insertion
                    matrix[i][j] + substitution_cost, // substitution
                ];

                matrix[i + 1][j + 1] = operations.iter().min().unwrap().clone();

                // transposition
                if i > 0
                    && j > 0
                    && s_char == self.tar.chars().nth(j - 1).unwrap()
                    && t_char == self.src.chars().nth(i - 1).unwrap()
                {
                    matrix[i + 1][j + 1] = min(
                        matrix[i + 1][j + 1],                     // cost without swappping
                        matrix[i - 1][j - 1] + substitution_cost, // cost with swapping
                    );
                }
            }
        }

        return matrix[len_src][len_tar];
    }

    fn unrestricted_distance(&self) -> usize {
        // get length of unicode chars
        let len_src = self.src.chars().count();
        let len_tar = self.tar.chars().count();

        let max_dist = len_src + len_tar;

        let mut da: HashMap<char, usize> = HashMap::new();
        let mut matrix: Vec<Vec<usize>> = vec![vec![0; len_tar + 2]; len_src + 2];
        matrix[0][0] = max_dist;

        for i in 0..(len_src + 1) {
            matrix[i + 1][0] = max_dist;
            matrix[i + 1][1] = i;
        }

        for j in 0..(len_tar + 1) {
            matrix[0][j + 1] = max_dist;
            matrix[1][j + 1] = j;
        }

        for i in 1..(len_src + 1) {
            let mut db = 0;
            for j in 1..(len_tar + 1) {
                let k = da
                    .get(&self.tar.chars().nth(j - 1).unwrap())
                    .unwrap_or(&0)
                    .clone();
                let l = db;

                let mut substitution_cost = 1;
                if self.src.chars().nth(i - 1).unwrap() == self.tar.chars().nth(j - 1).unwrap() {
                    substitution_cost = 0;
                    db = j;
                }
                let operations = [
                    matrix[i][j] + substitution_cost,             // substitution
                    matrix[i + 1][j] + 1,                         // insertion
                    matrix[i][j + 1] + 1,                         // deletion
                    matrix[k][l] + (i - k - 1) + 1 + (j - l - 1), // transposition
                ];
                matrix[i + 1][j + 1] = operations.iter().min().unwrap().clone();
            }
            da.insert(self.src.chars().nth(i - 1).unwrap(), i);
        }

        return matrix[len_src + 1][len_tar + 1];
    }

    /// Calculate the `Damerau-Levenshtein` distance between two strings.
    /// The parameter `restricted` is used to determine the algorithm.
    /// If restricted is `true`, it calculates `Optimal String Alignment Distance`.
    /// If restricted is `false`, it calculates distance with `Adjacent Transpositions`.
    /// 
    /// ### Examples
    /// 
    /// ```
    /// use text_distance::DamerauLevenshtein;
    /// 
    /// let restricted_damerau_levenshtein = DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: true};
    /// let unrestricted_damerau_levenshtein = DamerauLevenshtein {src: "ca".to_string(),  tar: "abc".to_string(), restricted: false};
    /// 
    /// assert_eq!(3, restricted_damerau_levenshtein.distance());
    /// assert_eq!(2, unrestricted_damerau_levenshtein.distance());
    /// 
    /// ```
    pub fn distance(&self) -> usize {
        if self.restricted {
            self.restricted_distance()
        } else {
            self.unrestricted_distance()
        }
    }

    /// Calculate the `normalized distance` between two strings.
    /// The normalized distance is the distance divided by the length of the longest string.
    /// The normalized distance is always between 0.0 and 1.0.
    /// When 0.0 then two strings are equal.
    /// When 1.0 then two strings are completely different.
    /// 
    /// ## Examples
    /// ```
    /// use text_distance::DamerauLevenshtein;
    /// 
    /// let damerau_levenshtein = DamerauLevenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string(), restricted: true};
    /// 
    /// assert_eq!(0.42857142857142855, damerau_levenshtein.normalized_distance());
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
    /// ## Examples
    /// 
    /// ```
    /// use text_distance::DamerauLevenshtein;
    /// 
    /// let damerau_levenshtein = DamerauLevenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string(), restricted: true};
    /// 
    /// assert_eq!(4, damerau_levenshtein.similarity());
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
    /// ## Examples
    /// 
    /// ```
    /// use text_distance::DamerauLevenshtein;
    /// 
    /// let damerau_levenshtein = DamerauLevenshtein {src: "karolin".to_string(),  tar: "kathrin".to_string(), restricted: true};
    /// 
    /// assert_eq!(0.5714285714285714, damerau_levenshtein.normalized_similarity());
    /// 
    /// ```
    pub fn normalized_similarity(&self) -> f64 {
        let str_normalized_distance = self.normalized_distance();
        return 1.0 - str_normalized_distance;
    }
}
