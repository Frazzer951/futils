use anyhow::{Context, Result};
use fs_err as fs;
use serde_json::{Map, Value};

pub fn format_json_file(filename: &str, output_filename: Option<&str>) -> Result<()> {
    // Read the file
    let data = fs::read_to_string(filename).context("Failed to read the file")?;

    // Parse the JSON data
    let mut json_value: Value = serde_json::from_str(&data).context("Failed to parse JSON")?;

    // Sort keys if it's a map
    if let Value::Object(map) = &mut json_value {
        sort_map(map);
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
