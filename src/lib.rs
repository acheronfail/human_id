pub mod words;

use rand::seq::SliceRandom;
use rand::thread_rng;

use words::{ADJECTIVES, NOUNS, VERBS};

/// Generates a "human id", with the given string acting as the separator between the words.
/// ```
/// use human_id::id;
///
/// id("-", false); // tame-lions-retire
/// id("", true);   // ChattyWombatsCare
/// ```
pub fn id<S>(separator: S, should_capitalize: bool) -> String
where
    S: Into<String>,
{
    let mut rng = thread_rng();
    let map = |x: &&str| {
        if should_capitalize {
            capitalize(*x)
        } else {
            x.to_string()
        }
    };

    [
        ADJECTIVES.choose(&mut rng).map(map).unwrap().to_string(),
        NOUNS.choose(&mut rng).map(map).unwrap().to_string(),
        VERBS.choose(&mut rng).map(map).unwrap().to_string(),
    ]
    .join(&separator.into())
}

fn capitalize<S>(input: S) -> String
where
    S: Into<String>,
{
    let string = input.into();
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {

    use crate::id;
    use crate::words::{ADJECTIVES, NOUNS, VERBS};

    fn extract_capitals<S>(s: S, separator: Option<String>) -> String
    where
        S: Into<String>,
    {
        s.into()
            .chars()
            .filter_map(|c| {
                let ch = c.to_string();

                // Remove separators.
                if separator == Some(ch.to_owned()) {
                    return None;
                }

                // Extract uppercased characters.
                if c.to_uppercase().collect::<String>() == ch {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<String>()
    }

    #[test]
    fn it_works_with_separator_and_no_capitalize() {
        let the_id = id("-", false);
        let capitals = extract_capitals(&the_id, Some("-".to_owned()));
        let parts = the_id.split("-").collect::<Vec<&str>>();
        assert_eq!(capitals.len(), 0);
        assert_eq!(parts.len(), 3);
        assert_eq!(ADJECTIVES.contains(&parts[0]), true);
        assert_eq!(NOUNS.contains(&parts[1]), true);
        assert_eq!(VERBS.contains(&parts[2]), true);
    }

    #[test]
    fn it_works_without_separator_and_capitalized() {
        let the_id = id("", true);
        let uppercase_chars = extract_capitals(&the_id, None);
        assert_eq!(uppercase_chars.len(), 3);
    }
}
