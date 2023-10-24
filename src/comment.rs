pub fn comment(text: &str, min_len: Option<usize>, symbol: Option<char>) -> String {
    let min_len = min_len.unwrap_or(10);
    let symbol = symbol.unwrap_or('*');

    let mut result = String::new();

    let len = text.len() + 6;
    let mut len = if len < min_len { min_len } else { len };

    // If text length is odd, ensure len is also odd
    if text.len() % 2 == 1 && len % 2 == 0 {
        len += 1;
    }

    // Calculate the number of symbol characters needed on each side
    let symbols_each_side = (len - text.len() - 6) / 2;

    // Add the first line
    result.push_str(&symbol.to_string().repeat(len));
    result.push('\n');

    // Add the second line
    result.push_str(&symbol.to_string().repeat(symbols_each_side + 2)); // +1 for the '*' at the start
    result.push(' ');
    result.push_str(text);
    result.push(' ');
    result.push_str(&symbol.to_string().repeat(symbols_each_side + 2)); // +1 for the '*' at the end
    result.push('\n');

    // Add the third line
    result.push_str(&symbol.to_string().repeat(len));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        let result = comment("test", None, None);
        let expected = "\
**********
** test **
**********";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_different_symbol() {
        let result = comment("test", None, Some('#'));
        let expected = "\
##########
## test ##
##########";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_different_length() {
        let result = comment("i", Some(7), None);
        let expected = "\
*******
** i **
*******";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_long() {
        let result = comment("long comment test", None, None);
        let expected = "\
***********************
** long comment test **
***********************";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_odd() {
        let result = comment("one", None, None);
        let expected = "\
***********
*** one ***
***********";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_short() {
        let result = comment("to", None, None);
        let expected = "\
**********
*** to ***
**********";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_long_odd() {
        let result = comment("long comment tests", None, None);
        let expected = "\
************************
** long comment tests **
************************";
        assert_eq!(result, expected);
    }
}
