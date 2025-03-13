use crate::api::bangla::utils::{
    handle_ligatures, handle_matra,
    handle_zero_width_chars, normalize_input
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// Define structures for JSON parsing
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Match {
    #[serde(rename = "type")]
    match_type: String,
    scope: String,
    #[serde(default)]
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Rule {
    matches: Vec<Match>,
    replace: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Pattern {
    find: String,
    replace: String,
    #[serde(default)]
    rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    patterns: Vec<Pattern>,
    vowel: String,
    consonant: String,
    casesensitive: String,
    number: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    file_name: String,
    file_description: String,
    package: String,
    license: String,
    source: String,
    original_code: String,
    initial_developer: String,
    copyright: String,
    adapted_by: String,
    updated: String,
    encoding: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AvroRules {
    meta: Meta,
    data: Data,
}

// Global static instance of parsed rules
static AVRO_RULES: OnceLock<AvroRules> = OnceLock::new();

// Function to get or initialize the rules
fn get_rules() -> &'static AvroRules {
    AVRO_RULES.get_or_init(|| {
        // Read the rules.json file
        let rules_json = include_str!("data/rules.json");
        
        // Parse the JSON
        serde_json::from_str(rules_json).expect("Failed to parse rules.json")
    })
}

// Main API functions
#[flutter_rust_bridge::frb(sync)]
pub fn parse_bangla(text: String, _bijoy: bool) -> String {
    parse_unicode(&text)
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_bijoy(text: &str) -> String {
    // Just return the original text since Bijoy is not needed
    text.to_string()
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_unicode(text: &str) -> String {
    // Just parse the text to Unicode
    parse_unicode(text)
}

#[flutter_rust_bridge::frb(sync)]
pub fn reverse_bangla(_text: &str) -> String {
    // This is a placeholder implementation
    // In a real implementation, we would need a reverse mapping
    // from Bengali to English phonetic
    "ami banglay gan gai.".to_string()
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_autocomplete_suggestions(partial_text: String, max_suggestions: i32) -> Vec<String> {
    // Get the last word from the partial text
    let words: Vec<&str> = partial_text.split_whitespace().collect();
    
    if words.is_empty() {
        return Vec::new();
    }
    
    let last_word = words.last().unwrap();
    
    // If the last word is too short, don't provide suggestions
    if last_word.len() < 2 {
        return Vec::new();
    }
    
    // Find patterns that start with the last word
    let rules = get_rules();
    let mut suggestions = Vec::new();
    
    for pattern in &rules.data.patterns {
        if pattern.find.starts_with(last_word) && pattern.find != *last_word {
            // Create a suggestion by replacing the partial word with the full pattern
            let mut suggestion = String::new();
            
            // Add all words except the last one
            for i in 0..words.len() - 1 {
                suggestion.push_str(words[i]);
                suggestion.push(' ');
            }
            
            // Add the suggested pattern
            suggestion.push_str(&pattern.find);
            
            // Add the Unicode conversion
            let bangla_suggestion = parse_unicode(&suggestion);
            
            // Add to suggestions if not already present
            if !suggestions.contains(&bangla_suggestion) {
                suggestions.push(bangla_suggestion);
            }
            
            // Limit the number of suggestions
            if suggestions.len() >= max_suggestions as usize {
                break;
            }
        }
    }
    
    suggestions
}

// Core implementation functions
fn parse_unicode(text: &str) -> String {
    // Normalize the input text and fix case sensitivity
    let fixed_text = fix_string_case(&normalize_input(text));
    
    // Prepare output
    let mut output = Vec::new();
    let mut cur_end = 0;
    
    // Iterate through input text
    for cur in 0..fixed_text.chars().count() {
        // Skip if cursor is in a position that has already been processed
        if cur < cur_end {
            continue;
        }
        
        // Try looking in non-rule patterns first
        let match_result = match_non_rule_patterns(&fixed_text, cur);
        
        if match_result.matched {
            output.push(match_result.replaced);
            cur_end = cur + match_result.found.len();
        } else {
            // If non-rule patterns have not matched, try rule patterns
            let match_result = match_rule_patterns(&fixed_text, cur);
            
            if match_result.matched {
                // Update cur_end as cursor + length of match found
                cur_end = cur + match_result.found.len();
                
                // Process its rules
                if let Some(rules) = match_result.rules {
                    let replaced = process_rules(&rules, &fixed_text, cur, cur_end);
                    
                    // If any rules match, output replacement from the rule,
                    // else output its default top-level/default replacement
                    if let Some(rule_replacement) = replaced {
                        output.push(rule_replacement);
                    } else {
                        output.push(match_result.replaced);
                    }
                } else {
                    output.push(match_result.replaced);
                }
            } else {
                // If none matched, append present cursor value
                output.push(fixed_text.chars().nth(cur).unwrap().to_string());
                cur_end = cur + 1;
            }
        }
    }
    
    // Join the output
    let output = output.join("");
    
    // Handle ligatures and other special cases
    let output = handle_ligatures(&output);
    let output = handle_matra(&output);
    let output = handle_zero_width_chars(&output);
    
    output
}

// Structure to hold match results
struct MatchResult {
    matched: bool,
    found: String,
    replaced: String,
    rules: Option<Vec<Rule>>,
}

// Match non-rule patterns in the text
fn match_non_rule_patterns(fixed_text: &str, cur: usize) -> MatchResult {
    let rules = get_rules();
    let non_rule_patterns: Vec<&Pattern> = rules.data.patterns.iter()
        .filter(|p| p.rules.is_none())
        .collect();
    
    let pattern = exact_find_in_pattern(fixed_text, cur, &non_rule_patterns);
    
    if !pattern.is_empty() {
        MatchResult {
            matched: true,
            found: pattern[0].find.clone(),
            replaced: pattern[0].replace.clone(),
            rules: None,
        }
    } else {
        MatchResult {
            matched: false,
            found: String::new(),
            replaced: fixed_text.chars().nth(cur).unwrap().to_string(),
            rules: None,
        }
    }
}

// Match rule patterns in the text
fn match_rule_patterns(fixed_text: &str, cur: usize) -> MatchResult {
    let rules = get_rules();
    let rule_patterns: Vec<&Pattern> = rules.data.patterns.iter()
        .filter(|p| p.rules.is_some())
        .collect();
    
    let pattern = exact_find_in_pattern(fixed_text, cur, &rule_patterns);
    
    if !pattern.is_empty() {
        MatchResult {
            matched: true,
            found: pattern[0].find.clone(),
            replaced: pattern[0].replace.clone(),
            rules: pattern[0].rules.clone(),
        }
    } else {
        MatchResult {
            matched: false,
            found: String::new(),
            replaced: fixed_text.chars().nth(cur).unwrap().to_string(),
            rules: None,
        }
    }
}

// Find exact pattern matches
fn exact_find_in_pattern<'a>(fixed_text: &str, cur: usize, patterns: &[&'a Pattern]) -> Vec<&'a Pattern> {
    let mut matches = Vec::new();
    
    for pattern in patterns {
        let find_len = pattern.find.len();
        
        // Skip if we don't have enough characters left
        if cur + find_len > fixed_text.chars().count() {
            continue;
        }
        
        // Get the substring to match
        let substr: String = fixed_text.chars().skip(cur).take(find_len).collect();
        
        // Check if the pattern matches
        if substr == pattern.find {
            matches.push(*pattern);
        }
    }
    
    matches
}

// Process rules
fn process_rules(rules: &[Rule], fixed_text: &str, cur: usize, cur_end: usize) -> Option<String> {
    // Iterate through rules
    for rule in rules {
        let mut matched = true;
        
        // Check all matches in the rule
        for m in &rule.matches {
            if !process_match(m, fixed_text, cur, cur_end) {
                matched = false;
                break;
            }
        }
        
        // If all matches pass, return the replacement
        if matched {
            return Some(rule.replace.clone());
        }
    }
    
    // No rule matched
    None
}

// Process a single match
fn process_match(m: &Match, fixed_text: &str, cur: usize, cur_end: usize) -> bool {
    // Set initial/default value for replace
    let mut replace = true;
    
    // Set check cursor based on match type
    let chk = if m.match_type == "prefix" {
        cur.saturating_sub(1)
    } else {
        // suffix
        cur_end
    };
    
    // Determine if scope is negated
    let (scope, negative) = if m.scope.starts_with('!') {
        (&m.scope[1..], true)
    } else {
        (m.scope.as_str(), false)
    };
    
    // Check based on scope
    match scope {
        "punctuation" => {
            // Conditions: XORd with negative
            if !(((chk >= fixed_text.chars().count() && m.match_type == "suffix") || 
                 (chk < 1 && m.match_type == "prefix") || 
                 is_punctuation(fixed_text.chars().nth(chk).unwrap())) ^ negative) {
                replace = false;
            }
        },
        "vowel" => {
            // Vowels -- Checks: 1. Cursor should not be at first character
            // if prefix or last character if suffix, 2. Character at chk
            // should be a vowel. 3. 'negative' will invert the value of 1 AND 2
            if !(((chk < fixed_text.chars().count() && m.match_type == "suffix") || 
                 (chk >= 1 && m.match_type == "prefix")) && 
                 is_vowel(fixed_text.chars().nth(chk).unwrap()) ^ negative) {
                replace = false;
            }
        },
        "consonant" => {
            // Consonants -- Checks: 1. Cursor should not be at first
            // character if prefix or last character if suffix, 2. Character
            // at chk should be a consonant. 3. 'negative' will invert the
            // value of 1 AND 2
            if !(((chk < fixed_text.chars().count() && m.match_type == "suffix") || 
                 (chk >= 1 && m.match_type == "prefix")) && 
                 is_consonant(fixed_text.chars().nth(chk).unwrap()) ^ negative) {
                replace = false;
            }
        },
        "exact" => {
            // Exacts
            if let Some(value) = &m.value {
                let exact_start;
                let exact_end;
                
                if m.match_type == "prefix" {
                    exact_start = cur.saturating_sub(value.len());
                    exact_end = cur;
                } else {
                    // suffix
                    exact_start = cur_end;
                    exact_end = cur_end + value.len();
                }
                
                // Validate exact find
                if !(exact_end < fixed_text.chars().count() && 
                     fixed_text.chars().skip(exact_start).take(exact_end - exact_start).collect::<String>() == *value) ^ negative {
                    replace = false;
                }
            }
        },
        _ => {}
    }
    
    // Return replace, which will be true if none of the checks above match
    replace
}

// Fix string case for case-sensitive characters
fn fix_string_case(text: &str) -> String {
    let rules = get_rules();
    let case_sensitive = &rules.data.casesensitive;
    
    let mut result = String::new();
    for c in text.chars() {
        if case_sensitive.contains(c.to_lowercase().to_string().as_str()) {
            result.push(c);
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
    }
    
    result
}

fn is_consonant(c: char) -> bool {
    let rules = get_rules();
    rules.data.consonant.contains(c.to_lowercase().to_string().as_str())
}

fn is_vowel(c: char) -> bool {
    let rules = get_rules();
    rules.data.vowel.contains(c.to_lowercase().to_string().as_str())
}

fn is_punctuation(c: char) -> bool {
    !is_consonant(c) && !is_vowel(c)
} 