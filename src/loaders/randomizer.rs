use rand::Rng;
use rand::seq::SliceRandom;

pub fn randomizer(wordlist: &Vec<String>) -> Result<String, std::io::Error> {
    let mut rng = rand::thread_rng();
    let random_word = wordlist.choose(&mut rng).unwrap();
    Ok(random_word.to_string())
}

pub fn charize(word: &String) -> Vec<char> {
    word.chars().collect()
}