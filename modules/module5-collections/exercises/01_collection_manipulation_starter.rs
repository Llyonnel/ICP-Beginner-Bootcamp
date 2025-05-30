use std::collections::HashMap;

fn main() {
    // Sample text for analysis
    let text = "Rust is a multi-paradigm, general-purpose programming language. \
                Rust emphasizes performance, type safety, and concurrency. \
                Rust enforces memory safety—that is, that all references point \
                to valid memory—without requiring the use of a garbage collector \
                or reference counting present in other memory-safe languages.";

    // TODO: 1. Split the text into words and store them in a vector
    let words: Vec<&str> = text.split_whitespace().collect();

    // TODO: 2. Count the frequency of each word and store in a HashMap
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    for word in &words {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }

    // TODO: 3. Find the longest word in the text
    let longest_word = words.iter().max_by_key(|w| w.len()).unwrap_or(&"");

    // TODO: 4. Convert all words to uppercase and store in a new vector
    let uppercase_words: Vec<String> = words.iter().map(|w| w.to_uppercase()).collect();

    // TODO: 5. Filter out words shorter than 4 characters
    let filtered_words: Vec<&str> = words.iter().filter(|w| w.len() >= 4).map(|&w| w).collect();

    // Print results
    println!("Word counts: {:?}", word_counts);
    println!("\nLongest word: {}", longest_word);
    println!("\nUppercase words: {:?}", uppercase_words);
    println!("\nWords longer than 3 characters: {:?}", filtered_words);
}