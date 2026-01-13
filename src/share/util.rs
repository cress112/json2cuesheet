const INDENT: &str = "  ";
const LF: &str = "\n";

pub fn build_indent_string(base_string: &String) -> String {
    let result = String::from(INDENT);
    result + base_string
}

pub fn join_strings_with_lf(strings: &Vec<String>) -> String {
    strings.join(LF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_indent_string() {
        let test_string = String::from("hogege");

        let result = build_indent_string(&test_string);

        assert_eq!(result, String::from("  hogege"));
    }

    #[test]
    fn test_join_strings_with_lf() {
        let test_strings = vec![String::from("hoge"), String::from("fg")];

        let result = join_strings_with_lf(&test_strings);

        assert_eq!(result, String::from("hoge\nfg"));
    }
}
