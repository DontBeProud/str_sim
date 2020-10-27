use crate::jaro::sim_jaro;
use std::cmp::min;


/// jaro-winkler similarity
pub fn sim_jaro_winkler(s1: &str, s2: &str, prefix_weight: f64) -> f64 {
    let jaro_sim = sim_jaro(s1, s2);
    let prefix_len = get_longest_prefix_length(s1, s2);
    let mut prefix_factor_coefficient = prefix_weight * prefix_len as f64;
    prefix_factor_coefficient = if prefix_factor_coefficient < 0.0 { 0.0 } else if prefix_factor_coefficient > 1.0 { 1.0 } else { prefix_factor_coefficient };
    jaro_sim + prefix_factor_coefficient * (1.0 - jaro_sim)
}


fn get_longest_prefix_length(s1: &str, s2: &str) -> usize{
    let min_len = min(s1.chars().count(), s2.chars().count());
    let mut longest_prefix_length = min_len;
    for i in 0..min_len{
        if s1.chars().nth(i) != s2.chars().nth(i){
            longest_prefix_length = i;
            break;
        }
    }
    longest_prefix_length
}