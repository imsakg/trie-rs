use std::collections::BTreeSet;
use trie::Trie;

fn next(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);
    *state
}
fn word(state: &mut u64) -> String {
    let alphabet = ['a', 'b', 'C', 'é', '🦀', 'Σ', ' '];
    let length = (next(state) % 6) as usize;
    (0..length)
        .map(|_| alphabet[(next(state) % alphabet.len() as u64) as usize])
        .collect()
}

#[test]
fn mixed_mutations_agree_with_an_independent_ordered_set() {
    for insensitive in [false, true] {
        let mut trie = if insensitive {
            Trie::case_insensitive()
        } else {
            Trie::new()
        };
        let mut expected = BTreeSet::new();
        let mut state = 0x5eed;
        for step in 0..2_000 {
            let input = word(&mut state);
            let key = if insensitive {
                input.to_lowercase()
            } else {
                input.clone()
            };
            match next(&mut state) % 6 {
                0..=2 => assert_eq!(trie.insert(&input), expected.insert(key)),
                3 => assert_eq!(trie.remove(&input), expected.remove(&key)),
                4 => assert_eq!(trie.contains(&input), expected.contains(&key)),
                _ if step % 97 == 0 => {
                    trie.clear();
                    expected.clear();
                }
                _ => assert_eq!(
                    trie.clone().words(),
                    expected.iter().cloned().collect::<Vec<_>>()
                ),
            }
            assert_eq!(trie.len(), expected.len());
            assert_eq!(trie.is_empty(), expected.is_empty());
            assert_eq!(trie.words(), expected.iter().cloned().collect::<Vec<_>>());
        }
    }
}
