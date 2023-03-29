use crate::helpers;

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
