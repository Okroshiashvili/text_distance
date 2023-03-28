use crate::helpers;

pub struct Levenshtein {
    pub s: String,
    pub t: String,
}

pub struct Hamming {
    pub s: String,
    pub t: String,
}

impl Hamming {
    pub fn distance(&self) -> usize {
        if self.s.chars().count() != self.t.chars().count() {
            panic!("Hamming distance is only defined for strings of equal length");
        }

        let mut distance_counter = 0;

        for (s_char, t_char) in self.s.chars().zip(self.t.chars()) {
            if s_char != t_char {
                distance_counter += 1;
            }
        }
        return distance_counter;
    }

    pub fn normalized_distance(&self) -> f64 {
        let maximum = helpers::max_text_length(self.s.clone(), self.t.clone());
        let str_distance = self.distance();
        if maximum != 0 {
            return (str_distance as f64) / (maximum as f64);
        }
        return 0.0;
    }

    pub fn similarity(&self) -> usize {
        let maximum = helpers::max_text_length(self.s.clone(), self.t.clone());
        let str_distance = self.distance();
        return maximum - str_distance;
    }

    pub fn normalized_similarity(&self) -> f64 {
        let str_normalized_distance = self.normalized_distance();
        return 1.0 - str_normalized_distance;
    }
}

impl Levenshtein {
    /// Calculate the Levenshtein distance between two strings.
    pub fn distance(&self) -> usize {
        let len_s = self.s.chars().count();
        let len_t = self.t.chars().count();

        // initialize the matrix
        let mut matrix: Vec<Vec<usize>> = vec![vec![0; len_t + 1]; len_s + 1];

        for i in 1..(len_s + 1) {
            matrix[i][0] = i;
        }
        for i in 1..(len_t + 1) {
            matrix[0][i] = i;
        }

        // apply edit operations
        for (i, s_char) in self.s.chars().enumerate() {
            for (j, t_char) in self.t.chars().enumerate() {
                let substitution = if s_char == t_char { 0 } else { 1 };
                matrix[i + 1][j + 1] = helpers::min_of_3(
                    matrix[i][j + 1] + 1,        // deletion
                    matrix[i + 1][j] + 1,        // insertion
                    matrix[i][j] + substitution, // substitution
                );
            }
        }

        return matrix[len_s][len_t];
    }

    pub fn normalized_distance(&self) -> f64 {
        let maximum = helpers::max_text_length(self.s.clone(), self.t.clone());
        let str_distance = self.distance();
        if maximum != 0 {
            return (str_distance as f64) / (maximum as f64);
        }
        return 0.0;
    }

    pub fn similarity(&self) -> usize {
        let maximum = helpers::max_text_length(self.s.clone(), self.t.clone());
        let str_distance = self.distance();
        return maximum - str_distance;
    }

    pub fn normalized_similarity(&self) -> f64 {
        let str_normalized_distance = self.normalized_distance();
        return 1.0 - str_normalized_distance;
    }
}
