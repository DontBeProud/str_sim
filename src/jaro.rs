use std::cmp;


/// jaro similarity
pub fn sim_jaro(s1: &str, s2: &str) -> f64 {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();
    if s1_len == 0 && s2_len == 0 { return 1.0; }

    let match_distance: isize = cmp::max(cmp::max(s1_len, s2_len) as isize / 2 - 1, 0);
    let mut s1_matches = vec![false; s1_len];
    let mut s2_matches = vec![false; s2_len];
    let mut m: isize = 0;
    for i in 0..s1_len {
        let start = cmp::max(0, i as isize - match_distance) as usize;
        let end = cmp::min(i + match_distance as usize + 1, s2_len);
        for j in start..end {
            if !s2_matches[j] && s1.chars().nth(i) == s2.chars().nth(j) {
                s1_matches[i] = true;
                s2_matches[j] = true;
                m += 1;
                break;
            }
        }
    }
    if m == 0 { return 0.0; }
    let mut t = 0.0;
    let mut k = 0;
    for i in 0..s1_len {
        if s1_matches[i] {
            while !s2_matches[k] { k += 1; }
            if s1.chars().nth(i) != s2.chars().nth(k) { t += 0.5; }
            k += 1;
        }
    }

    let m = m as f64;
    (m / s1_len as f64 + m / s2_len as f64 + (m  - t) / m) / 3.0
}
