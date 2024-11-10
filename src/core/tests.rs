#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::{format, project};
    use colored::Colorize;
    use similar::{ChangeTag, TextDiff};

    #[test]
    fn parse_test() {
        let path = "./example/basic/manifest.yaml";
        let manifest = std::fs::read_to_string(path).unwrap();
        let p = PathBuf::from(path);
        let parsed = project::from_yaml(manifest.clone(), p);

        print!("Parsed project as: ${:#?}", parsed);

        let result = project::to_yaml(&parsed);
        let result = serde_yaml::to_string(&result).unwrap();
        let result = format::pretty_format_yaml(result);

        let diff = TextDiff::from_lines(manifest.as_str(), result.as_str());

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => " - ".white().bold().on_red(),
                ChangeTag::Insert => " + ".white().bold().on_green(),
                ChangeTag::Equal => " * ".white().on_white(),
            };
            print!("{}{}", sign, change);
        }

        for change in diff.iter_all_changes() {
            assert_eq!(change.tag(), ChangeTag::Equal);
        }
    }
}
