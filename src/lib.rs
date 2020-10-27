pub mod levenshtein;
pub mod jaro;
pub mod jaro_winkler;

pub use crate::levenshtein::levenshtein_distance;
pub use crate::jaro::sim_jaro;
pub use crate::jaro_winkler::sim_jaro_winkler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(levenshtein_distance("dontbeproud", "dontbepride"), 3);
        assert_eq!(jaro::sim_jaro("dontbeproud", "dontbepride"), 0.8787878787878789);
        assert_eq!(jaro_winkler::sim_jaro_winkler("dontbeproud", "dontbepride", 0.05), 0.9272727272727274);
    }
}
