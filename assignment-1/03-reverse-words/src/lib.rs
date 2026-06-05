pub fn reverse_words(sentence: &str) -> String {
    let s = sentence;
    let s = s.trim();
    let s = format!("{s} ");
    let mut res = String::new();
    let mut char_vec: Vec<String> = Vec::new();

    for i in s.chars(){
        if !i.is_whitespace() {
            res.push(i);
        }else {
            char_vec.push(res);
            res = String::new();
        }
    }
    res = String::new();
    let mut i = char_vec.len()-1;
    while i!=0 {
        if char_vec[i] != ""{
            res.push_str(&char_vec[i]);
            res.push(' ');
        }
        i-=1;
    }
    res.push_str(&char_vec[0]);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_words() {
        assert_eq!(reverse_words("hello world rust"), "rust world hello");
    }

    #[test]
    fn collapses_inner_whitespace() {
        assert_eq!(reverse_words("   one   two  "), "two one");
    }

    #[test]
    fn empty_input() {
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn single_word() {
        assert_eq!(reverse_words("single"), "single");
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(reverse_words("    "), "");
    }

    #[test]
    fn many_short_words() {
        assert_eq!(reverse_words("a b c d e"), "e d c b a");
    }

    #[test]
    fn tabs_and_newlines_count_as_whitespace() {
        assert_eq!(reverse_words("a\tb\nc"), "c b a");
    }

    #[test]
    fn leading_and_trailing_trim() {
        assert_eq!(
            reverse_words("  leading and trailing  "),
            "trailing and leading"
        );
    }
}
