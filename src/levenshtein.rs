use std::cmp::min;


/// levenshtein edit distance
pub fn levenshtein_distance(s1: &str, s2:&str) -> usize{
    let row = s1.chars().count() + 1;
    let col = s2.chars().count() + 1;
    let mut matrix = vec![vec![0; col].into_boxed_slice(); row].into_boxed_slice();
    for i in 0..row{
        for j in 0..col{
            if i * j == 0{
                matrix[i][j] = i + j;
            }
            else {
                let b_not_equal = if s1.chars().nth(i - 1) != s2.chars().nth(j - 1) {1} else {0};
                matrix[i][j] = min(min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1), matrix[i - 1][j - 1] + b_not_equal);
            }
        }
    }
    matrix[row - 1][col - 1]
}