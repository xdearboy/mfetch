use std::collections::HashMap;
use std::process::Command;

pub fn is_available() -> bool {
    Command::new("which")
        .arg("decode-dimms")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn get_data() -> Option<Vec<HashMap<String, String>>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("decode-dimms")
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut all_spd = Vec::new();

    for section in stdout.split("Decoding EEPROM") {
        if !section.contains("DDR") || !section.contains("---===") {
            continue;
        }

        let mut data = HashMap::new();
        let mut timing_found = false;

        for line in section.lines() {
            let l_lower = line.to_lowercase();

            // parsing manufacturer and timings yes yes
            if l_lower.contains("module manufacturer") {
                insert_val(&mut data, "module_maker", line, "module manufacturer");
            }
            if l_lower.contains("dram manufacturer") {
                insert_val(&mut data, "dram_maker", line, "dram manufacturer");
            }
            if !timing_found && l_lower.contains("aa-rcd-rp-ras (cycles)") {
                if let Some(val) = line.split_whitespace().last() {
                    data.insert("timings".to_string(), val.to_string());
                    timing_found = true;
                }
            }
        }
        if !data.is_empty() {
            all_spd.push(data);
        }
    }
    if all_spd.is_empty() {
        None
    } else {
        Some(all_spd)
    }
}

fn insert_val(map: &mut HashMap<String, String>, key: &str, line: &str, pattern: &str) {
    let val = if line.contains(':') {
        line.split_once(':').unwrap().1.to_string()
    } else {
        line.to_lowercase()
            .find(pattern)
            .map(|idx| line[idx + pattern.len()..].trim().to_string())
            .unwrap_or_default()
    };
    if !val.is_empty() {
        map.insert(key.to_string(), val);
    }
}
