use std::cmp::max;

pub struct Levenshtein {
    pub s: String,
    pub t: String,
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
                let substitution_cost = if s_char == t_char { 0 } else { 1 };
                let operations = [
                    matrix[i][j + 1] + 1,        // deletion
                    matrix[i + 1][j] + 1,        // insertion
                    matrix[i][j] + substitution_cost, // substitution
                ];
                matrix[i + 1][j + 1] = operations.iter().min().unwrap().clone();
            }
        }

        return matrix[len_s][len_t];
    }

    pub fn normalized_distance(&self) -> f64 {
        let maximum = max(
            self.s.clone().chars().count(),
            self.t.clone().chars().count(),
        );
        let str_distance = self.distance();
        if maximum != 0 {
            return (str_distance as f64) / (maximum as f64);
        }
        return 0.0;
    }

    pub fn similarity(&self) -> usize {
        let maximum = max(
            self.s.clone().chars().count(),
            self.t.clone().chars().count(),
        );
        let str_distance = self.distance();
        return maximum - str_distance;
    }

    pub fn normalized_similarity(&self) -> f64 {
        let str_normalized_distance = self.normalized_distance();
        return 1.0 - str_normalized_distance;
    }
}
