use trie::Trie;

fn reference(pattern: &[char], word: &[char]) -> bool {
    let Some((&first, rest)) = pattern.split_first() else {
        return word.is_empty();
    };
    match first {
        '*' => reference(rest, word) || (!word.is_empty() && reference(pattern, &word[1..])),
        '?' => !word.is_empty() && reference(rest, &word[1..]),
        '\\' => {
            let (literal, tail) = match rest.split_first() {
                Some((literal, tail)) => (*literal, tail),
                None => ('\\', rest),
            };
            word.first() == Some(&literal) && reference(tail, &word[1..])
        }
        literal => word.first() == Some(&literal) && reference(rest, &word[1..]),
    }
}

fn combinations(alphabet: &[char], depth: usize) -> Vec<String> {
    let mut all = vec![String::new()];
    let mut layer = vec![String::new()];
    for _ in 0..depth {
        layer = layer
            .iter()
            .flat_map(|prefix| alphabet.iter().map(move |ch| format!("{prefix}{ch}")))
            .collect();
        all.extend(layer.clone());
    }
    all
}

#[test]
fn globs_agree_with_a_small_recursive_reference_matcher() {
    let mut words = combinations(&['a', 'b', 'é'], 3);
    words.extend(["*", "?", "\\", "a*b"].map(String::from));
    let trie: Trie = words.into_iter().collect();
    for pattern in combinations(&['a', 'b', 'é', '?', '*', '\\'], 3) {
        let tokens: Vec<_> = pattern.chars().collect();
        let expected: Vec<_> = trie
            .words()
            .into_iter()
            .filter(|word| reference(&tokens, &word.chars().collect::<Vec<_>>()))
            .collect();
        assert_eq!(trie.wildcard(&pattern), expected, "{pattern:?}");
        assert_eq!(
            trie.wildcard_with_limit(&pattern, 2),
            expected.into_iter().take(2).collect::<Vec<_>>(),
            "{pattern:?}"
        );
    }
}
