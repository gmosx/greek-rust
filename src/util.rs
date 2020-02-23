use regex::Regex;

pub struct Rule {
    pub re: Regex,
    pub replacement: &'static str,
}

impl Rule {
    pub fn new(expression: &'static str, replacement: &'static str) -> Rule {
        Rule {
            re: Regex::new(expression).unwrap(),
            replacement: replacement,
        }
    }

    pub fn apply(&self, input: &str) -> String {
        self.re.replace_all(&input, self.replacement).to_string()
    }
}
