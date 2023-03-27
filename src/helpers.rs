use std::cmp::max;
use std::cmp::min;

#[inline(always)]
pub fn min_of_3(a: usize, b: usize, c: usize) -> usize {
    return min(min(a, b), c);
}

#[inline(always)]
pub fn max_text_length(a: String, b: String) -> usize {
    // max length of a string
    return max(a.chars().count(), b.chars().count());
}
