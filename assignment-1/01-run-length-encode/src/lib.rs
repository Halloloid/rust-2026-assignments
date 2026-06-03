pub fn run_length_encode(input: &str) -> Vec<(char, u32)> {
    let s = input;
    let mut res:Vec<(char,u32)> = Vec::new();
    let mut checker:Vec<char> = Vec::new();
    let mut index = 0;
    let mut index2 = 0;
    checker.push(' ');
    for i in s.chars(){
        checker.push(i);
        index+=1;

        if checker[index-1] != i {
            res.push((i,1));
            index2 = res.len() - 1 ;
        } else {
            res[index2].1 += 1;
        }
    }
    // todo!("implement run_length_encode")
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_aaabbc() {
        assert_eq!(
            run_length_encode("aaabbc"),
            vec![('a', 3), ('b', 2), ('c', 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(run_length_encode(""), vec![]);
    }

    #[test]
    fn single_char() {
        assert_eq!(run_length_encode("x"), vec![('x', 1)]);
    }

    #[test]
    fn all_same() {
        assert_eq!(run_length_encode("aaaaa"), vec![('a', 5)]);
    }

    #[test]
    fn all_different() {
        assert_eq!(
            run_length_encode("abcd"),
            vec![('a', 1), ('b', 1), ('c', 1), ('d', 1)]
        );
    }

    #[test]
    fn alternating_runs() {
        assert_eq!(
            run_length_encode("aabbaa"),
            vec![('a', 2), ('b', 2), ('a', 2)]
        );
    }

    #[test]
    fn whitespace_counts_too() {
        assert_eq!(
            run_length_encode("aa  bb"),
            vec![('a', 2), (' ', 2), ('b', 2)]
        );
    }
}
