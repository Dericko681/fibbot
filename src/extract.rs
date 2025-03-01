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
// pub fn extract_numbers(content: String) -> Vec<u32> {
//     content
//         .replace("-", " ")
//         .split_whitespace()
//         .map(|word| word.chars().filter(|c| !c.is_ascii_punctuation()).collect::<String>())
//         .filter_map(|word| word.parse().ok())
//         .collect()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_extract_numbers_for_special_chars() {
//         let content = "Here are some numbers: 1; 42 and 100.".to_string();
//         let numbers = extract_numbers(content);
//         assert_eq!(numbers, vec![1, 42, 100]);
//     }

//     #[test]
//     fn test_extract_numbers_for_letters() {
//         let content = "Here are some numbers: 1b, 42D and E100F.".to_string();
//         let numbers = extract_numbers(content);
//         assert_eq!(numbers, vec![1, 42, 100]);
//     }

//     #[test]
//     fn test_extract_numbers_for_letters_special() {
//         let content = "Here are some numbers: 1b; w2d32-23?+ ++j12 42D and E100F.".to_string();
//         let numbers = extract_numbers(content);
//         assert_eq!(numbers, vec![1, 23223, 12, 42, 100]);
//     }

//     #[test]
//     fn test_extract_numbers_for_no_number() {
//         let content = "Here are some numbers: hello there".to_string();
//         let numbers = extract_numbers(content);
//         assert!(numbers.is_empty());
//     }
// }
