pub fn chunked_string(s: String, size: usize, max: usize) -> Vec<String> {
    s.split_whitespace()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .chunks(size)
        .take(max)
        .enumerate()
        .map(|(i, s)| {
            let s = s.join(" ");
            if i == max - 1 {
                format!("{s}...")
            } else {
                s
            }
        })
        .collect::<Vec<String>>()
}
