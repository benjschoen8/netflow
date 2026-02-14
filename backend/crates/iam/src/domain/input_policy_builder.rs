use crate::input_policy::InputPolicy;

pub struct InputPolicyBuilder {
    subject: &'static str,
    min_len: Option<usize>,
    max_len: Option<usize>,
    require_uppercase: bool,
    require_lowercase: bool,
    require_number: bool,
    require_symbol: bool,
    all_numbers: bool,
    must_contain: Vec<char>,
    illegal_characters: Vec<char>,
}

impl InputPolicyBuilder {
    pub fn new(subject: &'static str) -> Self {
        Self {
            subject,
            min_len: None,
            max_len: None,
            require_uppercase: false,
            require_lowercase: false,
            require_number: false,
            require_symbol: false,
            all_numbers: false,
            must_contain: Vec::new(),
            illegal_characters: Vec::new(),
        }
    }

    pub fn set_length(mut self, min: usize, max: usize) -> Self {
        self.min_len = Some(min);
        self.max_len = Some(max);
        self
    }

    pub fn require_complexity(mut self, upper: bool, lower: bool, digit: bool, symbol: bool) -> Self {
        self.require_uppercase = upper;
        self.require_lowercase = lower;
        self.require_number = digit;
        self.require_symbol = symbol;
        self
    }

    pub fn with_illegal_chars(mut self, chars: Vec<char>) -> Self {
        self.illegal_characters = chars;
        self
    }

    pub fn strictly_numeric(mut self) -> Self {
        self.all_numbers = true;
        self
    }

    pub fn with_required_substrings(mut self, c: Vec<char>) -> Self {
        self.must_contain = c;
        self
    }

    pub fn build(self) -> InputPolicy {
        InputPolicy {
            subject: self.subject,
            min_len: self.min_len,
            max_len: self.max_len,
            require_uppercase: self.require_uppercase,
            require_lowercase: self.require_lowercase,
            require_number: self.require_number,
            require_symbol: self.require_symbol,
            all_numbers: self.all_numbers,
            must_contain: self.must_contain,
            illegal_characters: self.illegal_characters,
        }
    }
}