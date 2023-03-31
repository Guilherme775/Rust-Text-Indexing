pub type Token = String;

pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn tokenize(input: &str) -> Self {
        let output = input
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>();

        Self { tokens: output }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tokens = self.tokens.clone();
        let first = tokens.first();

        match first {
            Some(token) => {
                self.tokens.remove(0);

                Some(token.clone())
            },
            None => None
        }
    }
}