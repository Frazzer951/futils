use anyhow::{Context, Result};
use fs_err as fs;
use serde::Serialize;
use serde_json::{
    ser::{PrettyFormatter, Serializer},
    Map, Value,
};

pub struct FormatJsonConfig {
    pub filename: String,
    pub output_filename: Option<String>,
    pub indent_size: Option<usize>,
    pub sort: bool,
    pub in_place: bool,
}

pub fn format_json_file(config: FormatJsonConfig) -> Result<String> {
    let indent_size = config.indent_size.unwrap_or(2);

    // Read the file
    let data = fs::read_to_string(&config.filename).context("Failed to read the file")?;

    // Parse the JSON data
    let mut json_value: Value = serde_json::from_str(&data).context("Failed to parse JSON")?;

    // Sort keys if it's a map
    if config.sort {
        if let Value::Object(map) = &mut json_value {
            sort_map(map);
        }
    }

    // Create a custom PrettyFormatter with the specified indent size
    let indent = " ".repeat(indent_size);
    let formatter = PrettyFormatter::with_indent(indent.as_bytes());

    // Serialize the JSON with the custom formatter
    let mut serialized_data = Vec::new();
    let mut serializer = Serializer::with_formatter(&mut serialized_data, formatter);
    json_value.serialize(&mut serializer).context("Failed to format JSON")?;

    // Write the formatted JSON data back to the file
    let formatted_data = String::from_utf8(serialized_data).context("Failed to convert bytes to string")?;

    if config.in_place {
        fs::write(&config.filename, &formatted_data).context("Failed to write back to the file")?;
    }
    if let Some(output_filename) = config.output_filename {
        fs::write(output_filename, &formatted_data).context("Failed to write back to the file")?;
    }

    Ok(formatted_data)
}

fn sort_map(map: &mut Map<String, Value>) {
    let mut sorted = std::collections::BTreeMap::new();
    for (k, v) in map.iter() {
        let mut v = v.clone();
        if let Value::Object(child_map) = &mut v {
            let mut new_map = Map::new();
            sort_map(child_map);
            new_map.extend(child_map.clone());
            v = Value::Object(new_map);
        }
        sorted.insert(k.clone(), v);
    }
    map.clear();
    for (k, v) in sorted {
        map.insert(k, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;
    use tempfile::NamedTempFile;

    fn setup_test_file(data: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create test file");
        file.write_all(data.as_bytes()).expect("Failed to write to test file");
        file
    }

    fn read_file(path: &Path) -> std::io::Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    #[test]
    fn test_basic_formatting() {
        let raw_json = r#"{"b":2,"a":1}"#;
        let expected_formatted_json = r#"{
  "b": 2,
  "a": 1
}"#;

        let test_file = setup_test_file(raw_json);
        let config = FormatJsonConfig {
            filename: test_file.path().to_str().unwrap().to_string(),
            output_filename: None,
            indent_size: None,
            sort: false,
            in_place: false,
        };

        let result = format_json_file(config).expect("Failed to format JSON");
        assert_eq!(result, expected_formatted_json);
    }

    #[test]
    fn test_basic_formatting_inplace() {
        let raw_json = r#"{"b":2,"a":1}"#;
        let expected_formatted_json = r#"{
  "b": 2,
  "a": 1
}"#;

        let test_file = setup_test_file(raw_json);
        let config = FormatJsonConfig {
            filename: test_file.path().to_str().unwrap().to_string(),
            output_filename: None,
            indent_size: None,
            sort: false,
            in_place: true,
        };

        format_json_file(config).expect("Failed to format JSON");
        let result = read_file(test_file.path()).expect("Failed to read test file");
        assert_eq!(result, expected_formatted_json);
    }

    #[test]
    fn test_basic_sort() {
        let raw_json = r#"{"b":2,"a":1}"#;
        let expected_formatted_json = r#"{
  "a": 1,
  "b": 2
}"#;

        let test_file = setup_test_file(raw_json);
        let config = FormatJsonConfig {
            filename: test_file.path().to_str().unwrap().to_string(),
            output_filename: None,
            indent_size: None,
            sort: true,
            in_place: false,
        };

        let result = format_json_file(config).expect("Failed to format JSON");
        assert_eq!(result, expected_formatted_json);
    }

    #[test]
    fn test_nested_sort() {
        let raw_json = r#"{"b":{"b":2,"a":1},"a":1}"#;
        let expected_formatted_json = r#"{
  "a": 1,
  "b": {
    "a": 1,
    "b": 2
  }
}"#;

        let test_file = setup_test_file(raw_json);
        let config = FormatJsonConfig {
            filename: test_file.path().to_str().unwrap().to_string(),
            output_filename: None,
            indent_size: None,
            sort: true,
            in_place: false,
        };

        let result = format_json_file(config).expect("Failed to format JSON");
        assert_eq!(result, expected_formatted_json);
    }

    #[test]
    fn test_output_file() {
        let raw_json = r#"{"b":2,"a":1}"#;
        let expected_formatted_json = r#"{
  "b": 2,
  "a": 1
}"#;

        let test_file = setup_test_file(raw_json);
        let output_file = NamedTempFile::new().expect("Failed to create test file");
        let config = FormatJsonConfig {
            filename: test_file.path().to_str().unwrap().to_string(),
            output_filename: Some(output_file.path().to_str().unwrap().to_string()),
            indent_size: None,
            sort: false,
            in_place: false,
        };

        format_json_file(config).expect("Failed to format JSON");
        let result = read_file(output_file.path()).expect("Failed to read test file");
        assert_eq!(result, expected_formatted_json);
    }
}
