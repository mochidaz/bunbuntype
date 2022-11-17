#[cfg(test)]
mod tests {
    use super::*;
    use crate::loaders::loader::load_wordlist;
    use crate::loaders::randomizer::{charize, randomizer};
    use rand::random;

    #[test]
    fn test_loaders() {
        let load = load_wordlist("resource/wordlist");
        let random = randomizer(&load);
        assert!(random.is_ok());
    }

    #[test]
    fn test_charize() {
        let word = String::from("test");
        let charized = charize(&word);
        assert_eq!(charized, vec!['t', 'e', 's', 't']);
    }
}
