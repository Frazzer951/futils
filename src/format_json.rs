use anyhow::{Context, Result};
use fs_err as fs;
use serde_json::{Map, Value};

pub fn format_json_file(filename: &str, output_filename: Option<&str>, sort: bool) -> Result<()> {
    // Read the file
    let data = fs::read_to_string(filename).context("Failed to read the file")?;

    // Parse the JSON data
    let mut json_value: Value = serde_json::from_str(&data).context("Failed to parse JSON")?;

    // Sort keys if it's a map
    if sort {
        if let Value::Object(map) = &mut json_value {
            sort_map(map);
        }
    }

    // Write the formatted JSON data back to the file
    let formatted_data = serde_json::to_string_pretty(&json_value).context("Failed to format JSON")?;
    let target_filename = output_filename.unwrap_or(filename);
    fs::write(target_filename, formatted_data).context("Failed to write back to the file")?;

    Ok(())
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
    fn test_valid_json() {
        let file = setup_test_file(r#"{"key": "value"}"#);
        let result = format_json_file(file.path().to_str().unwrap(), None, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_json() {
        let file = setup_test_file(r#"{"key": "value""#);
        let result = format_json_file(file.path().to_str().unwrap(), None, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_sort_json() {
        let file = setup_test_file(r#"{"b": "2", "a": "1"}"#);
        let _ = format_json_file(file.path().to_str().unwrap(), None, true).unwrap();

        let sorted_data = read_file(file.path()).expect("Failed to read the file");
        assert_eq!(
            sorted_data,
            r#"{
  "a": "1",
  "b": "2"
}"#
        );
    }

    #[test]
    fn test_nested_json() {
        let file = setup_test_file(r#"{"a": {"c": "3", "b": "2"}}"#);
        let _ = format_json_file(file.path().to_str().unwrap(), None, true).unwrap();

        let sorted_data = read_file(file.path()).expect("Failed to read the file");
        assert_eq!(
            sorted_data,
            r#"{
  "a": {
    "b": "2",
    "c": "3"
  }
}"#
        );
    }

    #[test]
    fn test_output_file() {
        let input_file = setup_test_file(r#"{"key": "value"}"#);
        let mut output_file = NamedTempFile::new().expect("Failed to create output test file");
        let _ = format_json_file(
            input_file.path().to_str().unwrap(),
            Some(output_file.path().to_str().unwrap()),
            false,
        )
        .unwrap();

        let mut output_data = String::new();
        output_file.read_to_string(&mut output_data).unwrap();
        assert_eq!(
            output_data,
            r#"{
  "key": "value"
}"#
        );
    }
}
