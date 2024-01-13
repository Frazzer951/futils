pub fn comment(
    text: &str,
    min_symbols: Option<usize>,
    min_len: Option<usize>,
    symbol: Option<char>,
    prefix: Option<String>,
    pad_with_symbol: bool,
) -> String {
    let min_symbols = min_symbols.unwrap_or(1);
    let min_len = min_len.unwrap_or(10);
    let symbol = symbol.unwrap_or('*');

    let mut result = String::new();

    // This is the extra padding that goes around the text
    let min_padding = min_symbols * 2 + 2;

    let len = text.len() + min_padding;
    let mut len = if len < min_len { min_len } else { len };

    // If text length is odd, ensure len is also odd
    if text.len() % 2 == 1 && len % 2 == 0 {
        len += 1;
    }

    // Calculate the number of symbol characters needed on each side
    let symbols_each_side = (len - text.len() - min_padding) / 2;

    // Add the first line
    if let Some(ref prefix) = prefix {
        result.push_str(prefix);
    }
    result.push_str(&symbol.to_string().repeat(len));
    result.push('\n');

    // Add the second line
    if let Some(ref prefix) = prefix {
        result.push_str(prefix);
    }

    let pad_symbol = if pad_with_symbol { symbol } else { ' ' };

    result.push_str(&symbol.to_string().repeat(min_symbols));
    result.push_str(&pad_symbol.to_string().repeat(symbols_each_side));
    result.push(' ');
    result.push_str(text);
    result.push(' ');
    result.push_str(&pad_symbol.to_string().repeat(symbols_each_side));
    result.push_str(&symbol.to_string().repeat(min_symbols));
    result.push('\n');

    // Add the third line
    if let Some(prefix) = prefix {
        result.push_str(&prefix);
    }
    result.push_str(&symbol.to_string().repeat(len));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        let result = comment("test", Some(2), None, None, None, true);
        let expected = "\
**********
** test **
**********";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_different_symbol() {
        let result = comment("test", Some(2), None, Some('#'), None, true);
        let expected = "\
##########
## test ##
##########";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_different_length() {
        let result = comment("i", Some(2), Some(7), None, None, true);
        let expected = "\
*******
** i **
*******";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_long() {
        let result = comment("long comment test", Some(2), None, None, None, true);
        let expected = "\
***********************
** long comment test **
***********************";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_odd() {
        let result = comment("one", Some(2), None, None, None, true);
        let expected = "\
***********
*** one ***
***********";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_short() {
        let result = comment("to", Some(2), None, None, None, true);
        let expected = "\
**********
*** to ***
**********";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_long_odd() {
        let result = comment("long comment tests", Some(2), None, None, None, true);
        let expected = "\
************************
** long comment tests **
************************";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_prefix() {
        let result = comment("test", Some(2), None, None, Some("-- ".to_string()), true);
        let expected = "\
-- **********
-- ** test **
-- **********";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_simple() {
        let result = comment("test", None, None, None, None, false);
        let expected = "\
**********
*  test  *
**********";
        assert_eq!(result, expected);
    }
}
