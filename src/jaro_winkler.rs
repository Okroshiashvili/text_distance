use std::cmp::{max, min};

/// Calculate the `Jaro` or `Jaro-Winkler` similarity between two strings.
/// The `Jaro-Winkler` similarity is the modification of the `Jaro` similarity.
/// It increases the score if the characters at the start of both strings are the same.
/// [For more information see wikipedia article](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
///
/// ### Examples
///
/// ```
/// use text_distance::JaroWinkler;
///
/// let jaro_winkler = JaroWinkler {src: "karolin".to_string(),  tar: "kathrin".to_string(), winklerize: true};
///
/// assert_eq!(0.15238095238095228, jaro_winkler.distance());
/// assert_eq!(0.15238095238095228, jaro_winkler.normalized_distance());
/// assert_eq!(0.8476190476190477, jaro_winkler.similarity());
/// assert_eq!(0.8476190476190477, jaro_winkler.normalized_similarity());
///
/// ```
/// 
pub struct JaroWinkler {
    /// Source string
    pub src: String,
    /// Target string
    pub tar: String,
    /// * If `winklerize = true` it calculates `Jaro-Winkler similarity`.
    /// * If `winklerize = false` it calculates `Jaro similarity`.
    pub winklerize: bool,
}

impl JaroWinkler {
    fn jaro(&self) -> f64 {
        let src_len = self.src.chars().count();
        let tar_len = self.tar.chars().count();

        if src_len == 0 && tar_len == 0 {
            return 1.0;
        } else if src_len == 0 || tar_len == 0 {
            return 0.0;
        } else if self.src == self.tar {
            return 1.0;
        }

        let match_radius = max(src_len, tar_len) / 2 - 1;
        let mut src_matches = vec![false; src_len];
        let mut tar_matches = vec![false; tar_len];

        let mut common_chars: usize = 0;

        for (i, s_char) in self.src.chars().enumerate() {
            let low = if i > match_radius {
                max(0, i - match_radius) // There was overflow. Don't know why
            } else {
                0
            };
            let high = min(i + match_radius + 1, tar_len);

            for (j, t_char) in self.tar.chars().enumerate().take(high).skip(low) {
                if t_char == s_char && !tar_matches[j] {
                    src_matches[i] = true;
                    tar_matches[j] = true;
                    common_chars += 1;
                    break;
                }
            }
        }

        if common_chars == 0 {
            return 0.0;
        }

        // Transpositions
        let mut k = 0;
        let mut transpositions = 0;

        for (i, _value) in src_matches.iter().enumerate().take(src_len) {
            if !src_matches[i] {
                continue;
            }
            while !tar_matches[k] {
                k += 1;
            }
            if self.src.as_bytes()[i] != self.tar.as_bytes()[k] {
                transpositions += 1;
            }
            k += 1;
        }

        ((common_chars as f64 / src_len as f64)
            + (common_chars as f64 / tar_len as f64)
            + ((common_chars - (transpositions / 2)) as f64 / common_chars as f64))
            / 3.0
    }

    fn winkler(&self) -> f64 {
        let jaro_distance = self.jaro();
        let mut prefix_len = 0;
        if jaro_distance > 0.7 {
            for (_i, (s_char, t_char)) in self.src.chars().zip(self.tar.chars()).enumerate() {
                if s_char == t_char {
                    prefix_len += 1;
                } else {
                    break;
                }
            }
            prefix_len = min(4, prefix_len);
            return jaro_distance + (prefix_len as f64 * 0.1 * (1.0 - jaro_distance));
        }

        jaro_distance
    }

    /// Calculate the `Jaro-Winkler` distance between two strings.
    /// The parameter `winklerize` is used to determine the algorithm.
    /// If winklerize is `true`, it calculates `Jaro-Winkler` similarity.
    /// If winklerize is `false`, it calculates `Jaro` similarity.
    ///
    /// ### Examples
    ///
    /// ```
    /// use text_distance::JaroWinkler;
    ///
    /// let jaro_winkler = JaroWinkler {src: "frog".to_string(),  tar: "fog".to_string(), winklerize: true};
    /// let jaro = JaroWinkler {src: "frog".to_string(),  tar: "fog".to_string(), winklerize: false};
    ///
    /// assert_eq!(0.07500000000000007, jaro_winkler.distance());
    /// assert_eq!(0.08333333333333337, jaro.distance());
    ///
    /// ```
    pub fn distance(&self) -> f64 {
        if self.winklerize {
            1.0 - self.winkler()
        } else {
            1.0 - self.jaro()
        }
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
    /// use text_distance::JaroWinkler;
    ///
    /// let jaro_winkler = JaroWinkler {src: "karolin".to_string(),  tar: "kathrin".to_string(), winklerize: true};
    ///
    /// assert_eq!(0.15238095238095228, jaro_winkler.normalized_distance());
    ///
    /// ```
    pub fn normalized_distance(&self) -> f64 {
        let str_distance = self.distance();

        str_distance / 1.0
    }

    /// Calculate the `similarity`.
    /// The similarity is always between 0.0 and 1.0.
    /// When 0.0 then two strings are completely different.
    /// When 1.0 then two strings are equal.
    ///
    /// ## Examples
    ///
    /// ```
    /// use text_distance::JaroWinkler;
    ///
    /// let jaro_winkler = JaroWinkler {src: "karolin".to_string(),  tar: "kathrin".to_string(), winklerize: true};
    ///
    /// assert_eq!(0.8476190476190477, jaro_winkler.similarity());
    ///
    /// ```
    pub fn similarity(&self) -> f64 {
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
    /// use text_distance::JaroWinkler;
    ///
    /// let jaro_winkler = JaroWinkler {src: "karolin".to_string(),  tar: "kathrin".to_string(), winklerize: true};
    ///
    /// assert_eq!(0.8476190476190477, jaro_winkler.normalized_similarity());
    ///
    /// ```
    ///
    pub fn normalized_similarity(&self) -> f64 {
        let str_normalized_distance = self.normalized_distance();

        1.0 - str_normalized_distance
    }
}
