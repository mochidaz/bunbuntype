pub(crate) fn calculate_wpm(words: f32, time: f32) -> f32 {
    let wpm = ( words / 5.0) / time;
    wpm
}

pub(crate) fn calculate_accuracy(total: f32, correct: f32) -> f32 {
    let accuracy = (correct / total);
    accuracy
}

pub fn calculate_awpm(words: f32, time: f32, correct: f32) -> f32 {
    let wpm = calculate_wpm(words, time);
    let accuracy = calculate_accuracy(words, correct);

    let awpm = wpm * accuracy;
    awpm
}