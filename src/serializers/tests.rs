#[cfg(test)]
mod tests {
    use crate::serializers::wpm_results::WpmResults;
    use serde::de::Deserialize;
    use serde::de::Visitor;
    use std::fs::File;
    use std::io::{Read, Write};

    static FILE: &str = "resource/score.json";

    #[test]
    fn test_serde() {
        let mut content = String::new();

        let new_wpm = WpmResults::new(27.0, 0.95, 1.5, 25.0);

        {
            let mut file = File::options().write(true).open(FILE).unwrap();
            file.write_all(new_wpm.to_json().as_bytes()).unwrap();
            file.sync_data();
        }

        let mut file = File::open(FILE).unwrap();

        file.read_to_string(&mut content);

        let wpm_results = serde_json::from_str::<WpmResults>(&content).unwrap();

        assert_eq!(wpm_results.wpm, 27.0);
    }
}
