pub fn longest_word(sentence: &str) -> Option<&str> {
    let s = sentence;
    let s = &format!("{s} ");
    let mut start_i = (0, 0);
    let mut end_i = (0, 0);
    let mut max = 0;
    let mut new_max;
    for i in s.chars() {
        if i == ' ' {
            new_max = end_i.1 - start_i.1;
            if max < new_max {
                max = new_max;
                start_i.0 = start_i.1;
                end_i.0 = end_i.1;
            }
            start_i.1 = end_i.1 + 1;
            end_i.1 = start_i.1;
        } else {
            end_i.1 += 1;
        }
    }
    let s = sentence;
    if let Some("") = s.get(start_i.0..end_i.0){
        return None;
    }else{
        return s.get(start_i.0..end_i.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_longest_of_four() {
        assert_eq!(longest_word("the quick brown fox"), Some("quick"));
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(longest_word("   "), None);
    }

    #[test]
    fn empty_input() {
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn ascending_lengths() {
        assert_eq!(longest_word("a bb ccc dd"), Some("ccc"));
    }

    #[test]
    fn single_word() {
        assert_eq!(longest_word("hello"), Some("hello"));
    }

    #[test]
    fn single_letter() {
        assert_eq!(longest_word("a"), Some("a"));
    }

    #[test]
    fn first_on_tie() {
        assert_eq!(longest_word("abc xyz def"), Some("abc"));
    }

    #[test]
    fn leading_and_trailing_whitespace() {
        assert_eq!(longest_word("  rust ferris  "), Some("ferris"));
    }
}
