use std::fs;

#[derive(Debug)]
struct PolicyConfigDto {
    min_length: Option<usize>,
    max_length: Option<usize>,
    require_uppercase: Option<bool>,
    require_lowercase: Option<bool>,
    require_number: Option<bool>,
    require_symbol: Option<bool>,
    all_numbers: Option<bool>,
    illegal_characters: Option<Vec<char>>,
    must_contain: Option<Vec<char>>,
}