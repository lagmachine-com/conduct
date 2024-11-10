pub fn pretty_format_yaml(content: String) -> String {
    let mut result = "".to_string();

    for line in content.lines().into_iter() {
        let filter = [" ", "-", "identifier", "display_name"];

        if !filter.iter().any(|f| line.starts_with(f)) {
            result.push_str("\n");
        }
        result.push_str(line);
        result.push_str("\n");
    }

    return result;
}
