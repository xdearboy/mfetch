use std::collections::HashMap;
use std::process::Command;

pub fn get_data() -> Option<Vec<HashMap<String, String>>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("decode-dimms 2>/dev/null")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_spd(&stdout)
}

pub fn parse_spd(raw: &str) -> Option<Vec<HashMap<String, String>>> {
    let mut all_spd = Vec::new();

    for section in raw.split("Decoding EEPROM") {
        if !section.contains("DDR") || !section.contains("---===") {
            continue;
        }

        let mut data = HashMap::new();
        let mut timing_found = false;

        for line in section.lines() {
            let l_lower = line.to_lowercase();

            if l_lower.contains("module manufacturer") {
                insert_val(&mut data, "module_maker", line, "module manufacturer");
            }
            if l_lower.contains("dram manufacturer") {
                insert_val(&mut data, "dram_maker", line, "dram manufacturer");
            }
            if l_lower.contains("fundamental memory type") {
                insert_val(&mut data, "mem_type", line, "fundamental memory type");
            }
            if l_lower.contains("number of ranks") {
                insert_val(&mut data, "ranks", line, "number of ranks");
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
        line.split_once(':').unwrap().1.trim().to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        assert!(parse_spd("").is_none());
        assert!(parse_spd("garbage\nlines\n").is_none());
    }

    #[test]
    fn test_parse_single_module() {
        let sample = r#"Decoding EEPROM: /sys/bus/i2c/drivers/ee1004/0-0050
Guessing DIMM is in bank 1

---=== SPD EEPROM Information ===---
EEPROM CRC of bytes 0-125 OK (0xABCD)
# of bytes written to SDRAM EEPROM 1024
Total number of bytes in EEPROM 1024
Fundamental Memory type DDR5 SDRAM
SPD Revision 1.2
Module Type SO-DIMM

---=== Manufacturer Data ===---
Module Manufacturer Crucial Technology
DRAM Manufacturer Micron Technology

---=== Timing Parameters ===---
Minimum Cycle Time (tCKmin) 0.313 ns
Maximum Cycle Time (tCKmax) 1.500 ns
AA-RCD-RP-RAS (cycles) 40-40-40-77
Supported CAS Latencies 40, 38, 36, 34, 32

---=== Other Information ===---
Number of Ranks 1
"#;
        let spd = parse_spd(sample).unwrap();
        assert_eq!(spd.len(), 1);
        let s = &spd[0];
        assert!(s.get("module_maker").unwrap().contains("Crucial"));
        assert!(s.get("dram_maker").unwrap().contains("Micron"));
        assert!(s.get("timings").unwrap().contains("40-40-40-77"));
        assert!(s.get("mem_type").unwrap().contains("DDR5"));
        assert_eq!(s.get("ranks").unwrap(), "1");
    }
}
