// Private glob engine. Dynamic programming avoids recursive star backtracking.
#[derive(Clone, Copy)]
enum Token {
    Literal(char),
    Any,
    Star,
}

pub(crate) struct Pattern(Vec<Token>);

impl Pattern {
    pub(crate) fn new(pattern: &str) -> Self {
        let mut chars = pattern.chars();
        let mut tokens = Vec::new();
        while let Some(ch) = chars.next() {
            tokens.push(match ch {
                '\\' => Token::Literal(chars.next().unwrap_or('\\')),
                '?' => Token::Any,
                '*' => Token::Star,
                other => Token::Literal(other),
            });
        }
        Self(tokens)
    }

    pub(crate) fn matches(&self, word: &str) -> bool {
        let chars: Vec<_> = word.chars().collect();
        let mut previous = vec![false; chars.len() + 1];
        previous[0] = true;
        for token in &self.0 {
            let mut next = vec![false; chars.len() + 1];
            next[0] = matches!(token, Token::Star) && previous[0];
            for j in 1..=chars.len() {
                next[j] = match token {
                    Token::Star => previous[j] || next[j - 1],
                    Token::Any => previous[j - 1],
                    Token::Literal(ch) => previous[j - 1] && *ch == chars[j - 1],
                };
            }
            previous = next;
        }
        previous[chars.len()]
    }
}
