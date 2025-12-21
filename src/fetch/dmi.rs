use std::collections::HashMap;
use std::process::Command;

pub fn get_modules() -> Option<Vec<HashMap<String, String>>> {
    let output = Command::new("dmidecode")
        .args(["--type", "17"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut modules = Vec::new();

    for block in stdout.split("\nHandle ") {
        if block.contains("Size:") && !block.contains("No Module Installed") {
            let mut map = HashMap::new();
            for line in block.lines() {
                if let Some((k, v)) = line.split_once(':') {
                    map.insert(k.trim().to_lowercase(), v.trim().to_string());
                }
            }
            modules.push(map);
        }
    }
    Some(modules)
}
