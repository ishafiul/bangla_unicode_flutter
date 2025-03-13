use regex::Regex;
use lazy_static::lazy_static;

// Check if a character is a vowel
pub fn is_vowel(c: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    vowels.contains(&c)
}

// Check if a character is a consonant
pub fn is_consonant(c: char) -> bool {
    let c_lower = c.to_lowercase().next().unwrap();
    c_lower.is_ascii_alphabetic() && !is_vowel(c_lower)
}

// Fix string pattern for regex
pub fn fix_string_pattern(pattern: &str) -> String {
    let mut result = String::new();
    for c in pattern.chars() {
        if c.is_alphanumeric() {
            result.push(c);
        } else {
            result.push('\\');
            result.push(c);
        }
    }
    result
}

// Handle Bengali ligatures (যুক্তাক্ষর)
pub fn handle_ligatures(text: &str) -> String {
    lazy_static! {
        static ref HASANT_REGEX: Regex = Regex::new(r"্([^\u09CD])").unwrap();
    }
    
    // Replace hasant (্) followed by a non-hasant with hasant + ZWJ + the character
    HASANT_REGEX.replace_all(text, "্\u{200D}$1").to_string()
}

// Handle matra (diacritical marks)
pub fn handle_matra(text: &str) -> String {
    // This is a placeholder implementation
    // In a real implementation, we would handle various matra combinations
    text.to_string()
}

// Handle ZWNJ/ZWJ issues
pub fn handle_zero_width_chars(text: &str) -> String {
    lazy_static! {
        // Pattern for detecting incorrect ZWJ/ZWNJ usage
        static ref INCORRECT_ZWJ: Regex = Regex::new(r"\u{200D}([কখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহড়ঢ়য়])").unwrap();
        static ref INCORRECT_ZWNJ: Regex = Regex::new(r"\u{200C}([কখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহড়ঢ়য়])").unwrap();
    }
    
    // Fix incorrect ZWJ/ZWNJ usage
    let text = INCORRECT_ZWJ.replace_all(text, "$1").to_string();
    INCORRECT_ZWNJ.replace_all(&text, "$1").to_string()
}

// Function to handle backspace correction
pub fn handle_backspace_correction(text: &str, cursor_pos: usize) -> (String, usize) {
    if cursor_pos == 0 || text.is_empty() {
        return (text.to_string(), cursor_pos);
    }
    
    // Get the character at cursor_pos - 1
    let chars: Vec<char> = text.chars().collect();
    let current_char = chars[cursor_pos - 1];
    
    // Check if it's part of a ligature
    if current_char == '\u{09CD}' && cursor_pos >= 2 {
        // If it's a hasant (্), remove the hasant and the previous character
        let mut new_text = text.chars().take(cursor_pos - 2).collect::<String>();
        new_text.push_str(&text.chars().skip(cursor_pos).collect::<String>());
        return (new_text, cursor_pos - 2);
    } else if cursor_pos >= 2 && chars[cursor_pos - 2] == '\u{09CD}' {
        // If previous character is hasant, remove both
        let mut new_text = text.chars().take(cursor_pos - 2).collect::<String>();
        new_text.push_str(&text.chars().skip(cursor_pos).collect::<String>());
        return (new_text, cursor_pos - 2);
    }
    
    // Default behavior: remove one character
    let mut new_text = text.chars().take(cursor_pos - 1).collect::<String>();
    new_text.push_str(&text.chars().skip(cursor_pos).collect::<String>());
    (new_text, cursor_pos - 1)
}

// Function to normalize input text
pub fn normalize_input(text: &str) -> String {
    // Replace multiple spaces with a single space
    let mut result = text.to_string();
    
    lazy_static! {
        static ref MULTI_SPACE: Regex = Regex::new(r"\s+").unwrap();
    }
    
    result = MULTI_SPACE.replace_all(&result, " ").to_string();
    
    // Trim the result
    result.trim().to_string()
} 