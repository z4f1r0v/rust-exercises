use std::cmp;
use std::vec::Vec;
use std::str;

/// Takes a vector of string slices and formats them
/// line by line in a flower box, e.g.
/// vec!["one", "two", "three"] becomes:
///
/// *********
/// * one   *
/// * two   *
/// * three *
/// *********
///
pub fn make_flower_box(elems: Vec<&str>) -> String {
    if elems.is_empty() {
        String::new()
    } else {
        // element length + 2 starts + 2 spaces
        let max_length_line = get_max_line_length(&elems) + 4;
        let mut flower_box = String::new();
        let formated_border = format_border("*", max_length_line);
        let border = formated_border.as_str();
        flower_box.push_str(border);
        elems.iter().for_each(|e| flower_box.push_str(format_line(e, max_length_line).as_str()));
        flower_box.push_str(border);
        flower_box
    }
}

/// Creates a string with a border and space padding.
///
/// Examples
/// ```
/// use problem4::format_line;
/// let actual = format_line("hello", 10);
/// let expected = String::from("* hello      *\n");
/// assert_eq!(actual, expected)
/// ```
pub fn format_line(s: &str, max_length: usize) -> String {
    // the number of spaces after the word on the line is equal to
    // total line length - word length - two stars and the space preceding the word
    let num_spaces_after_word = max_length - s.len() - 3;
    let spaces_after_word = " ".repeat(num_spaces_after_word);
    format!("* {}{}*\n", s, spaces_after_word)
}

/// Creates a border that can be used at the top and bottom
/// of our flower box.
///
/// Examples
/// ```
/// use problem4::format_border;
/// let actual = format_border("*", 5);
/// let expected = String::from("*****\n");
/// assert_eq!(actual, expected)
/// ```
pub fn format_border(s: &str, length: usize) -> String {
    let c = s.to_string().repeat(length);
    format!("{}\n", c)
}

/// Gets the longest line length from `elems`.
///
/// Examples
/// ```
/// use problem4::get_max_line_length;
/// let elems = vec!["fizz", "buzz", "fizzbuzz"];
/// assert_eq!(get_max_line_length(&elems), 8);
/// ```
pub fn get_max_line_length(elems: &Vec<&str>) -> usize {
    elems.iter().fold(0, |max_len, &x| {
        if max_len < x.len() {
            x.len()
        } else {
            max_len
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_max_line_length() {
        let elems = vec!["fizz", "buzz", "fizzbuzz"];
        assert_eq!(get_max_line_length(&elems), 8);
    }

    #[test]
    fn test_format_border() {
        let actual = format_border("*", 5);
        let expected = String::from("*****\n");
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_format_line() {
        let actual = format_line("hello", 10);
        let expected = String::from("* hello  *\n");
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_empty_box() {
        let expected = String::from("");
        let empty_vec: Vec<&str> = Vec::new();

        assert_eq!(expected, make_flower_box(empty_vec));
    }

    #[test]
    fn test_box() {
        let expected = r####"************
* one      *
* two      *
* three    *
* four     *
* five     *
* six      *
* seven    *
* eight    *
* nine     *
* ten      *
* eleven   *
* twelve   *
* thirteen *
************
"####.to_string();

        let test_vec = vec!["one", "two", "three", "four", "five", "six", "seven", "eight",
                            "nine", "ten", "eleven", "twelve", "thirteen"];

        assert_eq!(expected, make_flower_box(test_vec));
    }
}
