pub fn patch(line: &str, pattern: &str, replacement: &str) -> String {
    line.replace(pattern, replacement)
}
