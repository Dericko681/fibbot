pub fn extract_numbers(content: String) -> Vec<u32> {
    content
        .split_whitespace()
        .filter_map(|word| {
            let trimmed = word.trim_matches(|c: char| !c.is_digit(10)); // Keep only digit characters
            if !trimmed.is_empty() {
                trimmed.parse::<u32>().ok() // Attempt to parse the trimmed string to u32
            } else {
                None
            }
        })
        .collect()
}

