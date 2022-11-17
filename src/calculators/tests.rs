#[cfg(test)]
mod tests {
    use crate::calculators::calculators::*;
    use super::*;

    #[test]
    fn test_wpm() {
        let wpm = calculate_wpm(200.0, 1.5);
        assert_eq!(wpm.round(), 27.0);
    }

    #[test]
    fn test_accuracy() {
        let accuracy = calculate_accuracy(200.0, 190.0);
        assert_eq!(accuracy, 0.95);
    }

    #[test]
    fn test_awpm() {
        let wpm = calculate_awpm(200.0, 1.5, 190.0);
        assert_eq!(wpm.round(), 25.0);
    }
}